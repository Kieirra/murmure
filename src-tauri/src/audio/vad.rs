const VAD_SPEECH_THRESHOLD: f32 = 0.015;
const VAD_SILENCE_THRESHOLD: f32 = 0.01;
const VAD_EMA_ALPHA: f32 = 0.3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum VoiceActivity {
    NotStarted,
    Silent,
    Active,
}

pub(super) struct Vad {
    smoothed_rms: f32,
    has_speech_started: bool,
}

impl Vad {
    pub(super) fn new() -> Self {
        Self {
            smoothed_rms: 0.0,
            has_speech_started: false,
        }
    }

    pub(super) fn update(&mut self, rms: f32) -> VoiceActivity {
        self.smoothed_rms = VAD_EMA_ALPHA * rms + (1.0 - VAD_EMA_ALPHA) * self.smoothed_rms;

        if self.smoothed_rms > VAD_SPEECH_THRESHOLD {
            self.has_speech_started = true;
        }

        if !self.has_speech_started {
            VoiceActivity::NotStarted
        } else if self.smoothed_rms < VAD_SILENCE_THRESHOLD {
            VoiceActivity::Silent
        } else {
            VoiceActivity::Active
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vad_stays_not_started_below_speech_threshold() {
        let mut vad = Vad::new();
        for _ in 0..50 {
            assert_eq!(vad.update(0.005), VoiceActivity::NotStarted);
        }
    }

    #[test]
    fn vad_arms_speech_on_quiet_word_via_ema() {
        let mut vad = Vad::new();
        let mut armed = false;
        for _ in 0..20 {
            if vad.update(0.02) != VoiceActivity::NotStarted {
                armed = true;
                break;
            }
        }
        assert!(armed, "EMA should arm speech on a sustained quiet word");
    }

    #[test]
    fn vad_micro_peak_during_silence_does_not_return_active() {
        let mut vad = Vad::new();
        for _ in 0..20 {
            vad.update(0.05);
        }
        for _ in 0..20 {
            vad.update(0.0);
        }
        assert_eq!(vad.update(0.0), VoiceActivity::Silent);
        assert_eq!(vad.update(0.03), VoiceActivity::Silent);
    }

    #[test]
    fn vad_sustained_speech_returns_active_and_resets_silence() {
        let mut vad = Vad::new();
        for _ in 0..20 {
            vad.update(0.05);
        }
        assert_eq!(vad.update(0.05), VoiceActivity::Active);
    }
}
