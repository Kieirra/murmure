use anyhow::Result;
use kokorox::tts::koko::{ModelVariant, TTSKoko};
use log::{debug, info};
use std::path::Path;

pub struct KokoroEngine {
    tts: TTSKoko,
}

impl KokoroEngine {
    pub async fn new(model_path: &Path, voices_path: &Path) -> Result<Self> {
        info!("Initializing KokoroEngine with model: {:?}", model_path);

        // Environment setup for espeak-ng on Windows
        #[cfg(target_os = "windows")]
        {
            let scoop_espeak = "C:\\Users\\jp_22\\scoop\\apps\\espeak-ng\\current";
            let espeak_data = format!("{}\\{}", scoop_espeak, "espeak-ng-data");
            let espeak_exe = format!("{}\\{}", scoop_espeak, "espeak-ng.exe");
            let espeak_dll = format!("{}\\{}", scoop_espeak, "libespeak-ng.dll");

            if Path::new(&espeak_data).exists() {
                debug!("Setting ESPEAK_DATA_PATH to {}", espeak_data);
                std::env::set_var("ESPEAK_DATA_PATH", &espeak_data);
                std::env::set_var("ESPEAK_NG_DATA_PATH", &espeak_data);
            }

            if Path::new(&espeak_exe).exists() {
                debug!("Setting PHONEMIZER_ESPEAK_EXECUTABLE to {}", espeak_exe);
                std::env::set_var("PHONEMIZER_ESPEAK_EXECUTABLE", &espeak_exe);
            }

            if Path::new(&espeak_dll).exists() {
                debug!("Setting PHONEMIZER_ESPEAK_LIBRARY to {}", espeak_dll);
                std::env::set_var("PHONEMIZER_ESPEAK_LIBRARY", &espeak_dll);
            }

            if let Ok(path) = std::env::var("PATH") {
                if !path.contains(scoop_espeak) {
                    debug!("Adding {} to PATH", scoop_espeak);
                    std::env::set_var("PATH", format!("{};{}", scoop_espeak, path));
                }
            }
        }

        let model_str = model_path.to_str();
        let voices_str = voices_path.to_str();

        let tts =
            TTSKoko::new_with_variant(model_str, voices_str, None, ModelVariant::V1English).await;

        info!("KokoroEngine initialization complete");
        Ok(Self { tts })
    }

    pub fn generate_audio(
        &self,
        text: &str,
        lang: &str,
        voice: &str,
        speed: f32,
    ) -> Result<Vec<f32>> {
        debug!(
            "Synthesizing audio: \"{}\" [lang={}, voice={}]",
            text, lang, voice
        );

        // Generate raw audio (f32 samples)
        // Signature: (txt, lan, style_name, speed, initial_silence, auto_detect_language, force_style, phonemes)
        let audio = self
            .tts
            .tts_raw_audio(text, lang, voice, speed, None, false, true, false)
            .map_err(|e| anyhow::anyhow!("TTS error: {}", e))?;

        Ok(audio)
    }
}
