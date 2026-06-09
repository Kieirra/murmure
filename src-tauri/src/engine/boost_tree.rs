use std::collections::HashMap;
use std::collections::VecDeque;

const CONTEXT_SCORE: f32 = 1.0;
const DEPTH_SCALING: f32 = 2.0;

pub type NodeId = usize;

const ROOT: NodeId = 0;

#[derive(Debug, Clone)]
struct Node {
    token_score: f32,
    node_score: f32,
    depth: usize,
    children: HashMap<i32, NodeId>,
    fail: NodeId,
    backoff_w: f32,
    is_end: bool,
}

impl Node {
    fn root() -> Self {
        Self {
            token_score: 0.0,
            node_score: 0.0,
            depth: 0,
            children: HashMap::new(),
            fail: ROOT,
            backoff_w: 0.0,
            is_end: false,
        }
    }
}

/// A token reachable from a boost state: its fusion score and the trie depth
/// of the node it leads to (1 = phrase start).
#[derive(Debug, Clone, Copy)]
pub struct BiasCandidate {
    pub token: i32,
    pub score: f32,
    pub depth: usize,
}

/// Weighted Aho-Corasick automaton used as a fused language model for phrase
/// boosting, mirroring NeMo's GPU-PB (`context_graph_universal.py` build +
/// `boosting_graph_batched.py` backoff). The automaton scores token
/// continuations; the greedy decoder adds these scores to non-blank logits.
#[derive(Debug, Clone)]
pub struct BoostTree {
    nodes: Vec<Node>,
}

impl BoostTree {
    pub fn new(phrases: &[Vec<i32>]) -> Self {
        let mut tree = Self {
            nodes: vec![Node::root()],
        };
        for phrase in phrases {
            tree.add_phrase(phrase);
        }
        tree.build_fail_links();
        tree
    }

    fn add_phrase(&mut self, phrase: &[i32]) {
        let last = phrase.len().saturating_sub(1);
        let mut cur = ROOT;
        for (i, &token) in phrase.iter().enumerate() {
            let depth_token_score = if i == 0 {
                CONTEXT_SCORE
            } else {
                CONTEXT_SCORE * DEPTH_SCALING + (i as f32 + 1.0).ln()
            };

            match self.nodes[cur].children.get(&token).copied() {
                Some(child) => {
                    let token_score = self.nodes[child].token_score.max(depth_token_score);
                    let parent_node_score = self.nodes[cur].node_score;
                    self.nodes[child].token_score = token_score;
                    self.nodes[child].node_score = parent_node_score + token_score;
                    if i == last {
                        self.nodes[child].is_end = true;
                    }
                    cur = child;
                }
                None => {
                    let parent_node_score = self.nodes[cur].node_score;
                    let child = self.nodes.len();
                    self.nodes.push(Node {
                        token_score: depth_token_score,
                        node_score: parent_node_score + depth_token_score,
                        depth: i + 1,
                        children: HashMap::new(),
                        fail: ROOT,
                        backoff_w: 0.0,
                        is_end: i == last,
                    });
                    self.nodes[cur].children.insert(token, child);
                    cur = child;
                }
            }
        }
    }

    fn build_fail_links(&mut self) {
        let mut queue: VecDeque<NodeId> = VecDeque::new();

        let root_children: Vec<NodeId> = self.nodes[ROOT].children.values().copied().collect();
        for child in root_children {
            self.nodes[child].fail = ROOT;
            self.set_backoff(child);
            queue.push_back(child);
        }

        while let Some(cur) = queue.pop_front() {
            let edges: Vec<(i32, NodeId)> = self.nodes[cur]
                .children
                .iter()
                .map(|(&t, &c)| (t, c))
                .collect();
            for (token, child) in edges {
                let fail = self.find_fail(self.nodes[cur].fail, token);
                self.nodes[child].fail = fail;
                self.set_backoff(child);
                queue.push_back(child);
            }
        }
    }

    fn find_fail(&self, mut state: NodeId, token: i32) -> NodeId {
        loop {
            if let Some(&child) = self.nodes[state].children.get(&token) {
                return child;
            }
            if state == ROOT {
                return ROOT;
            }
            state = self.nodes[state].fail;
        }
    }

    // backoff_w is negative: it reimburses the boost accumulated along this
    // branch when the decode diverges back toward the fail state. A completed
    // phrase keeps its boost (backoff 0, as in NeMo/icefall context graphs):
    // only abandoned partial matches are reimbursed.
    fn set_backoff(&mut self, node: NodeId) {
        let fail = self.nodes[node].fail;
        self.nodes[node].backoff_w = if self.nodes[node].is_end {
            0.0
        } else {
            self.nodes[fail].node_score - self.nodes[node].node_score
        };
    }

    /// Fusion score for every reachable token from `state`, following the
    /// backoff chain to the root while accumulating `backoff_w`. The deepest
    /// state wins for a given token (first seen along the chain).
    pub fn bias(&self, state: NodeId) -> Vec<BiasCandidate> {
        let mut result: HashMap<i32, BiasCandidate> = HashMap::new();
        let mut acc = 0.0;
        let mut cur = state;
        loop {
            for (&token, &child) in &self.nodes[cur].children {
                result.entry(token).or_insert_with(|| BiasCandidate {
                    token,
                    score: acc + self.nodes[child].token_score,
                    depth: self.nodes[child].depth,
                });
            }
            if cur == ROOT {
                break;
            }
            acc += self.nodes[cur].backoff_w;
            cur = self.nodes[cur].fail;
        }
        result.into_values().collect()
    }

    /// Next state after emitting `token`: the child reached along the backoff
    /// chain (deepest match first), or root if the token is unreachable.
    pub fn advance(&self, state: NodeId, token: i32) -> NodeId {
        let mut cur = state;
        loop {
            if let Some(&child) = self.nodes[cur].children.get(&token) {
                return child;
            }
            if cur == ROOT {
                return ROOT;
            }
            cur = self.nodes[cur].fail;
        }
    }

    pub fn root(&self) -> NodeId {
        ROOT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-5
    }

    #[test]
    fn first_token_score_is_context_score() {
        let tree = BoostTree::new(&[vec![10, 20]]);
        let child = tree.nodes[ROOT].children[&10];
        assert!(approx(tree.nodes[child].token_score, CONTEXT_SCORE));
        assert!(approx(tree.nodes[child].node_score, CONTEXT_SCORE));
    }

    #[test]
    fn deeper_token_uses_depth_scaling() {
        let tree = BoostTree::new(&[vec![10, 20]]);
        let first = tree.nodes[ROOT].children[&10];
        let second = tree.nodes[first].children[&20];
        let expected = CONTEXT_SCORE * DEPTH_SCALING + 2.0_f32.ln();
        assert!(approx(tree.nodes[second].token_score, expected));
        assert!(approx(
            tree.nodes[second].node_score,
            CONTEXT_SCORE + expected
        ));
    }

    #[test]
    fn shared_prefix_keeps_max_token_score() {
        let tree = BoostTree::new(&[vec![10, 20], vec![10, 30]]);
        let first = tree.nodes[ROOT].children[&10];
        assert!(approx(tree.nodes[first].token_score, CONTEXT_SCORE));
        assert_eq!(tree.nodes[first].children.len(), 2);
    }

    #[test]
    fn backoff_reimburses_partial_matches_only() {
        let tree = BoostTree::new(&[vec![10, 20]]);
        let first = tree.nodes[ROOT].children[&10];
        let second = tree.nodes[first].children[&20];
        // Abandoning a partial match reimburses the boost...
        assert!(tree.nodes[first].backoff_w < 0.0);
        // ...but a completed phrase keeps it.
        assert!(approx(tree.nodes[second].backoff_w, 0.0));
    }

    #[test]
    fn bias_continues_phrase() {
        let tree = BoostTree::new(&[vec![10, 20]]);
        let first = tree.nodes[ROOT].children[&10];
        let bias = tree.bias(first);
        let cand = bias.iter().find(|c| c.token == 20).unwrap();
        let expected = CONTEXT_SCORE * DEPTH_SCALING + 2.0_f32.ln();
        assert!(approx(cand.score, expected));
        assert_eq!(cand.depth, 2);
    }

    #[test]
    fn bias_can_start_new_phrase_from_deep_state() {
        let tree = BoostTree::new(&[vec![10, 20], vec![50]]);
        let first = tree.nodes[ROOT].children[&10];
        let bias = tree.bias(first);
        // Token 50 starts a fresh phrase; reachable via backoff to root.
        let cand = bias.iter().find(|c| c.token == 50).unwrap();
        // Net score for restarting includes the backoff reimbursement.
        let expected = tree.nodes[first].backoff_w + CONTEXT_SCORE;
        assert!(approx(cand.score, expected));
        assert_eq!(cand.depth, 1);
    }

    #[test]
    fn completed_word_does_not_penalize_next_phrase_start() {
        let tree = BoostTree::new(&[vec![10, 20], vec![50]]);
        let end = tree.advance(tree.advance(ROOT, 10), 20);
        let bias = tree.bias(end);
        // Right after completing a word, starting another dictionary word
        // gets the full entry score, with no reimbursement of the first one.
        let cand = bias.iter().find(|c| c.token == 50).unwrap();
        assert!(approx(cand.score, CONTEXT_SCORE));
    }

    #[test]
    fn advance_follows_phrase() {
        let tree = BoostTree::new(&[vec![10, 20]]);
        let first = tree.advance(ROOT, 10);
        assert_ne!(first, ROOT);
        let second = tree.advance(first, 20);
        assert!(tree.nodes[second].is_end);
    }

    #[test]
    fn advance_returns_root_on_unreachable_token() {
        let tree = BoostTree::new(&[vec![10, 20]]);
        let first = tree.advance(ROOT, 10);
        assert_eq!(tree.advance(first, 999), ROOT);
    }

    #[test]
    fn advance_restarts_via_backoff() {
        let tree = BoostTree::new(&[vec![10, 20], vec![50]]);
        let first = tree.advance(ROOT, 10);
        let restarted = tree.advance(first, 50);
        let direct = tree.advance(ROOT, 50);
        assert_eq!(restarted, direct);
    }
}
