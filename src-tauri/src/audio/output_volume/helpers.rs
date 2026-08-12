pub const MAX_LOWERED_PERCENT: u8 = 80;

pub(super) fn lowered_levels(original: &[u32], percent: u8) -> Vec<u32> {
    let percent = u32::from(percent.min(MAX_LOWERED_PERCENT));
    original
        .iter()
        .map(|level| (u64::from(*level) * u64::from(percent) / 100) as u32)
        .collect()
}

pub(super) fn levels_match(current: &[u32], expected: &[u32], tolerance: u32) -> bool {
    current.len() == expected.len()
        && current
            .iter()
            .zip(expected)
            .all(|(a, b)| a.abs_diff(*b) <= tolerance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowers_each_channel_proportionally() {
        assert_eq!(lowered_levels(&[65536, 32768], 20), vec![13107, 6553]);
    }

    #[test]
    fn preserves_amplified_volume_above_full_scale() {
        assert_eq!(lowered_levels(&[73725], 20), vec![14745]);
    }

    #[test]
    fn clamps_percent_to_eighty() {
        assert_eq!(lowered_levels(&[100], 200), lowered_levels(&[100], 80));
    }

    #[test]
    fn zero_percent_silences_every_channel() {
        assert_eq!(lowered_levels(&[65536, 65536], 0), vec![0, 0]);
    }

    #[test]
    fn levels_match_accepts_rounding_drift() {
        assert!(levels_match(&[13100], &[13107], 655));
    }

    #[test]
    fn levels_match_rejects_user_change() {
        assert!(!levels_match(&[40000], &[13107], 655));
    }

    #[test]
    fn levels_match_rejects_channel_count_change() {
        assert!(!levels_match(&[13107], &[13107, 13107], 655));
    }
}
