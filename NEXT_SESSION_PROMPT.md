# Prompt de Relance - Session Kokoro TTS French

Copie-colle ce texte dans une nouvelle conversation pour reprendre exactement là où nous en sommes :

---

**Bonjour, j'ai besoin de continuer le développement de Murmure sur la partie TTS (Kokoro v1.0). Voici l'état actuel :**

1. **Backend Rust OK** : Le moteur `engine.rs` est aligné sur Kokoro v1.0. Il gère maintenant le découpage par phrases (batching), la suppression des silences (trimming) et la sélection correcte des styles de voix.
2. **Phonemization** : On utilise un phonemisateur maison (`phonemizer.rs`) qui imite Misaki. Les règles pour les sons français complexes (`gn`, `oi`, `nasales`) ont été ajoutées.
3. **Fichiers** : `kokoro.onnx` et `fr.bin` (ff_siwis) sont configurés et fonctionnels.

**Objectif de cette session :**
- Tester la fluidité sur des paragraphes réels.
- Si la prononciation reste imparfaite sur certains mots, décider s'il faut pousser le système de règles Rust ou forcer l'installation de `espeak-ng` via `kokoroxide/voirs`.
- Nettoyer le code temporaire (`dump_*.py`, `verify_*.py`) à la racine.

*Consulte `KOKORO_INTEGRATION.md` pour les détails techniques précis sur les ID de tokens et la logique de batching.*

---
