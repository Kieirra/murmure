# Kokoro TTS V1.0 Integration - Status Report & Research Log

## 🎯 Current Status (March 2026)
After extensive testing, we have pivoted from a manual rule-based phonemizer to an integration of **espeak-ng** for French IPA generation. While technically functional, the manual mapping of IPA to Kokoro IDs proved too fragile for production (audio artifacts, truncated words). 

**Decision**: We are now moving towards evaluating specialized libraries (`kokorox` or `voirs`) to handle the linguistic pipeline properly.

## 🛠️ Key Technical Implementations & Discoveries

### 1. The Linguistic Pipeline
- **Generator**: Integration of `espeak-ng` via PowerShell/Command to ensure UTF-8 output on Windows.
- **Mapping**: 100% synchronization with the official `Kokoro-82M` vocabulary:
    - **Nasal (tilde)**: ID 17 (`\u0303`).
    - **Stress Primary**: ID 156 (`ˈ`).
    - **Stress Secondary**: ID 157 (`ˌ`).
    - **IPA Symbols**: Direct mapping starting at index 69.
- **Filtering**: Removal of liaison markers (`-`, `_`) and punctuation normalization to prevent "white noise" or "robotic bleeps".

### 2. Audio Engine Stability
- **Style Index**: Fixed to row `0` (stable voice) instead of dynamic indexing which caused noise on long sentences.
- **Padding Formula**: For Kokoro v1.0 stability, sequences must be padded with `0` at start and `10-20` zeros at the end to prevent syllable cutting.
- **Audio Trimming**: Re-enabled with a threshold of `0.0001` to remove ONNX technical silence.

## 🧠 Lessons Learned (The "Hics")

| Issue | Root Cause | Solution/Workaround |
|-------|------------|----------------------|
| **"If" sound** | ID 48 (f) prefix | Removed or followed by silence (ID 16). |
| **"Ssa" / "ntrue" bruits** | Incorrect ID mapping (Tilde) | Aligned ID 17 with combining tilde and filtered stress conflicts. |
| **Cutoff ends** | Model buffer exhaustion | Increased end padding (20x ID 0) and segmented by sentences. |
| **Silent segments** | Multiple ID 16 (spaces) | Implemented a de-duplicator to ensure only one pulse of silence between words. |
| **Windows Glitches** | Encodage (ANSI vs UTF-8) | Forced PowerShell with UTF-8 encoding for espeak interaction. |

## 🚀 Pivot Strategy Resolved: Intégration de Kokorox

La cartographie manuelle IPA a été abandonnée au profit du crate [`kokorox`](https://github.com/lucas3d/kokorox). Ce projet gère efficacement tout le cycle G2P et la tokenisation, en résolvant la plupart des problèmes rencontrés avec l'ancienne architecture "manuelle". 

### 🌟 Fonctionnalités Actuelles intégrées à Murmure
1. **Support Multilingue Dynamique**
   - Une intégration de la bibliothèque `whatlang` permet la détection **automatique** de la langue à la volée.
   - Si Murmure est configuré sur "Default", il détecte lui-même si le texte est en Français ou en Anglais et bascule automatiquement sur les bonnes configurations linguistiques.

2. **Voix Configurables**
   - Accès aux 13 voix anglophones packagées avec le modèle natif Kokoro (variantes américaines et britanniques masculines/féminines).
   - Les "TTS Voices" sont directement sélectionnables via le panel "Paramètres -> Système" de Murmure.
   - Sélection automatique de la voix britannique ou américaine lorsque le texte détecté est de l'Anglo-Saxon, ou maintien sur la voix française *`ff_siwis`* dans notre langue.

**En attente**: 
- *Potentielle mise à niveau du runtime ML via le crate `voirs` pour encore plus de performance, si le besoin s'en fait sentir dans les prochaines itérations du Speech-to-Speech (STT > LLM > TTS).*
