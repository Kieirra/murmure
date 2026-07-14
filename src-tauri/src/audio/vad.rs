const FLOOR_INIT: f32 = 0.003;
const FLOOR_ALPHA_DOWN: f32 = 0.2;
const FLOOR_ALPHA_UP: f32 = 0.005;
const FLOOR_UP_MAX_RMS_RATIO: f32 = 10.0;
const K_SPEECH: f32 = 5.0;
const K_SILENCE: f32 = 3.0;
const SPEECH_THRESHOLD_MIN: f32 = 0.004;
const SPEECH_THRESHOLD_MAX: f32 = 0.08;
const VAD_EMA_ALPHA: f32 = 0.3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VoiceActivity {
    NotStarted,
    Silent,
    Active,
}

pub(crate) struct AdaptiveVad {
    noise_floor: f32,
    smoothed_rms: f32,
    has_speech_started: bool,
}

impl AdaptiveVad {
    pub(crate) fn new() -> Self {
        Self {
            noise_floor: FLOOR_INIT,
            smoothed_rms: 0.0,
            has_speech_started: false,
        }
    }

    pub(crate) fn observe(&mut self, rms: f32) {
        if rms < self.noise_floor {
            self.noise_floor = FLOOR_ALPHA_DOWN * rms + (1.0 - FLOOR_ALPHA_DOWN) * self.noise_floor;
        } else {
            let floor_base = self.noise_floor.max(SPEECH_THRESHOLD_MIN / K_SPEECH);
            if rms <= floor_base * FLOOR_UP_MAX_RMS_RATIO {
                self.noise_floor = FLOOR_ALPHA_UP * rms + (1.0 - FLOOR_ALPHA_UP) * self.noise_floor;
            }
        }
        self.smoothed_rms = VAD_EMA_ALPHA * rms + (1.0 - VAD_EMA_ALPHA) * self.smoothed_rms;
    }

    pub(crate) fn update(&mut self, rms: f32) -> VoiceActivity {
        self.observe(rms);

        if self.is_above_speech() {
            self.has_speech_started = true;
        }

        if !self.has_speech_started {
            VoiceActivity::NotStarted
        } else if self.smoothed_rms < self.silence_threshold() {
            VoiceActivity::Silent
        } else {
            VoiceActivity::Active
        }
    }

    pub(crate) fn speech_threshold(&self) -> f32 {
        (self.noise_floor * K_SPEECH).clamp(SPEECH_THRESHOLD_MIN, SPEECH_THRESHOLD_MAX)
    }

    pub(crate) fn silence_threshold(&self) -> f32 {
        (self.noise_floor * K_SILENCE).clamp(SPEECH_THRESHOLD_MIN * 0.6, SPEECH_THRESHOLD_MAX * 0.6)
    }

    pub(crate) fn is_above_speech(&self) -> bool {
        self.smoothed_rms > self.speech_threshold()
    }

    pub(crate) fn reset_speech_state(&mut self) {
        self.smoothed_rms = 0.0;
        self.has_speech_started = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TICKS_PER_SECOND: usize = 1000 / 33;

    fn observe_for(vad: &mut AdaptiveVad, rms: f32, seconds: usize) {
        for _ in 0..TICKS_PER_SECOND * seconds {
            vad.observe(rms);
        }
    }

    #[test]
    fn weak_micro_speech_is_detected_after_noise_calibration() {
        let mut vad = AdaptiveVad::new();
        observe_for(&mut vad, 0.0012, 2);

        let mut activity = VoiceActivity::NotStarted;
        for _ in 0..10 {
            activity = vad.update(0.0128);
            if activity == VoiceActivity::Active {
                break;
            }
        }

        assert_eq!(activity, VoiceActivity::Active);
        assert!((vad.speech_threshold() - 0.006).abs() < 0.001);

        for _ in 0..TICKS_PER_SECOND * 30 {
            assert_ne!(vad.update(0.0128), VoiceActivity::Silent);
        }
    }

    #[test]
    fn weak_variable_speech_remains_active_after_a_brief_threshold_crossing() {
        let mut vad = AdaptiveVad::new();
        observe_for(&mut vad, 0.0015, 2);
        let sequence = [
            0.013, 0.013, 0.013, 0.004, 0.004, 0.004, 0.004, 0.010, 0.010, 0.010, 0.010,
        ];

        let activities: Vec<VoiceActivity> =
            sequence.into_iter().map(|rms| vad.update(rms)).collect();

        assert_eq!(activities[0], VoiceActivity::NotStarted);
        assert_eq!(activities[1], VoiceActivity::NotStarted);
        assert_eq!(activities[2], VoiceActivity::Active);
        assert!(activities[3..]
            .iter()
            .all(|activity| *activity == VoiceActivity::Active));
        assert!(!vad.is_above_speech());
    }

    #[test]
    fn strong_micro_keeps_existing_start_delay() {
        let mut vad = AdaptiveVad::new();

        assert_eq!(vad.update(0.05), VoiceActivity::NotStarted);
        assert_eq!(vad.update(0.05), VoiceActivity::Active);
    }

    #[test]
    fn higher_permanent_background_noise_stabilizes_without_speech() {
        let mut vad = AdaptiveVad::new();
        observe_for(&mut vad, 0.0012, 2);
        for _ in 0..TICKS_PER_SECOND * 30 {
            vad.update(0.008);
        }

        assert!((vad.speech_threshold() - 0.04).abs() < 0.001);

        vad.reset_speech_state();
        for _ in 0..TICKS_PER_SECOND * 5 {
            assert_eq!(vad.update(0.008), VoiceActivity::NotStarted);
        }
    }

    #[test]
    fn near_zero_floor_recovers_on_background_noise() {
        let mut vad = AdaptiveVad {
            noise_floor: 0.00001,
            smoothed_rms: 0.0,
            has_speech_started: false,
        };

        for _ in 0..TICKS_PER_SECOND * 30 {
            vad.update(0.008);
        }

        assert!((vad.noise_floor - 0.0079).abs() < 0.001);
    }

    #[test]
    fn sustained_speech_does_not_pollute_floor_and_ends_normally() {
        let mut vad = AdaptiveVad::new();
        observe_for(&mut vad, 0.0012, 2);
        let calibrated_floor = vad.noise_floor;

        let mut activity = VoiceActivity::NotStarted;
        for _ in 0..TICKS_PER_SECOND * 30 {
            activity = vad.update(0.05);
            assert_ne!(activity, VoiceActivity::Silent);
        }

        assert_eq!(activity, VoiceActivity::Active);
        assert!(vad.noise_floor < calibrated_floor * 2.0);

        for _ in 0..10 {
            activity = vad.update(0.0012);
        }
        assert_eq!(activity, VoiceActivity::Silent);
    }

    #[test]
    fn speech_threshold_is_clamped_at_minimum() {
        let vad = AdaptiveVad {
            noise_floor: 0.00001,
            smoothed_rms: 0.0,
            has_speech_started: false,
        };

        assert_eq!(vad.speech_threshold(), SPEECH_THRESHOLD_MIN);
    }

    #[test]
    fn speech_threshold_is_clamped_at_maximum() {
        let vad = AdaptiveVad {
            noise_floor: 0.05,
            smoothed_rms: 0.0,
            has_speech_started: false,
        };

        assert_eq!(vad.speech_threshold(), SPEECH_THRESHOLD_MAX);
    }

    #[test]
    fn reset_speech_state_preserves_noise_floor() {
        let mut vad = AdaptiveVad::new();
        observe_for(&mut vad, 0.0012, 2);
        let calibrated_floor = vad.noise_floor;
        for _ in 0..10 {
            vad.update(0.0128);
        }

        vad.reset_speech_state();

        assert_eq!(vad.noise_floor, calibrated_floor);
        assert_eq!(vad.smoothed_rms, 0.0);
        assert!(!vad.has_speech_started);

        let mut activity = VoiceActivity::NotStarted;
        for _ in 0..10 {
            activity = vad.update(0.0128);
        }
        assert_eq!(activity, VoiceActivity::Active);
    }

    #[test]
    fn micro_peak_during_silence_does_not_return_active() {
        let mut vad = AdaptiveVad::new();
        for _ in 0..20 {
            vad.update(0.05);
        }
        for _ in 0..20 {
            vad.update(0.0);
        }
        assert_eq!(vad.update(0.0), VoiceActivity::Silent);
        assert_eq!(vad.update(0.005), VoiceActivity::Silent);
    }
}
