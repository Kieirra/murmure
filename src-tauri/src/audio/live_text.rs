/// Mirroring streaming.rs VAD: hysteresis on an EMA-smoothed RMS so a quiet short word still arms speech and a micro-peak during a pause does not reset the silence timer.
const LIVE_TEXT_SPEECH_THRESHOLD: f32 = 0.015;
const LIVE_TEXT_SILENCE_THRESHOLD: f32 = 0.01;
const LIVE_TEXT_EMA_ALPHA: f32 = 0.3;

/// Whether the smoothed signal is in silence once speech has started.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum LiveTextSilence {
    NotStarted,
    Silent,
    Active,
}

/// EMA + hysteresis VAD. The silence-to-boundary timer lives in the writer thread, not here.
pub(super) struct LiveTextVad {
    smoothed_rms: f32,
    has_speech_started: bool,
}

impl LiveTextVad {
    pub(super) fn new() -> Self {
        Self {
            smoothed_rms: 0.0,
            has_speech_started: false,
        }
    }

    pub(super) fn update(&mut self, rms: f32) -> LiveTextSilence {
        self.smoothed_rms =
            LIVE_TEXT_EMA_ALPHA * rms + (1.0 - LIVE_TEXT_EMA_ALPHA) * self.smoothed_rms;

        if self.smoothed_rms > LIVE_TEXT_SPEECH_THRESHOLD {
            self.has_speech_started = true;
        }

        if !self.has_speech_started {
            LiveTextSilence::NotStarted
        } else if self.smoothed_rms < LIVE_TEXT_SILENCE_THRESHOLD {
            LiveTextSilence::Silent
        } else {
            LiveTextSilence::Active
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vad_stays_not_started_below_speech_threshold() {
        let mut vad = LiveTextVad::new();
        // A faint signal under the speech threshold never arms speech.
        for _ in 0..50 {
            assert_eq!(vad.update(0.005), LiveTextSilence::NotStarted);
        }
    }

    #[test]
    fn vad_arms_speech_on_quiet_word_via_ema() {
        let mut vad = LiveTextVad::new();
        // A quiet word just above the speech threshold arms speech once the EMA
        // converges, even though a single raw frame is borderline.
        let mut armed = false;
        for _ in 0..20 {
            if vad.update(0.02) != LiveTextSilence::NotStarted {
                armed = true;
                break;
            }
        }
        assert!(armed, "EMA should arm speech on a sustained quiet word");
    }

    #[test]
    fn vad_micro_peak_during_silence_does_not_return_active() {
        let mut vad = LiveTextVad::new();
        // Arm speech with clear speech-level input.
        for _ in 0..20 {
            vad.update(0.05);
        }
        // Settle into silence.
        for _ in 0..20 {
            vad.update(0.0);
        }
        assert_eq!(vad.update(0.0), LiveTextSilence::Silent);
        // A single micro-peak frame is absorbed by the EMA and must not flip
        // the state back to Active (which would reset the silence timer).
        assert_eq!(vad.update(0.03), LiveTextSilence::Silent);
    }

    #[test]
    fn vad_sustained_speech_returns_active_and_resets_silence() {
        let mut vad = LiveTextVad::new();
        for _ in 0..20 {
            vad.update(0.05);
        }
        assert_eq!(vad.update(0.05), LiveTextSilence::Active);
    }
}
