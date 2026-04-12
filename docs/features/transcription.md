# Transcription

Murmure's core feature is local speech-to-text powered by NVIDIA's Parakeet TDT 0.6B v3 model.

## How It Works

1. You press the recording shortcut
2. Audio is captured from your microphone as a 16kHz mono WAV
3. The Parakeet model transcribes the audio locally
4. Post-processing is applied (dictionary, formatting rules, LLM if enabled)
5. The resulting text is inserted into the focused application

All processing happens on your CPU - no GPU required, no internet connection needed.

## Language Detection

Parakeet automatically detects the language from the audio. There is currently no way to force a specific language.

**Supported languages**: Bulgarian, Croatian, Czech, Danish, Dutch, English, Estonian, Finnish, French, German, Greek, Hungarian, Italian, Latvian, Lithuanian, Maltese, Polish, Portuguese, Romanian, Slovak, Slovenian, Spanish, Swedish, Russian, Ukrainian.

!!! note "Language accuracy varies"
Some languages have higher accuracy than others. French, English, German, and Swedish work very well. Greek and some smaller languages have lower accuracy.

## Recording Limits

- **Maximum duration**: 5 minutes per recording
- **Minimum duration**: Very short recordings (< 1 second) may fail or produce no output

After 5 minutes, the recording automatically stops and transcription begins.

## Tips for Better Transcription

### Microphone Quality Matters

The single biggest factor in transcription quality is your microphone. A poor microphone with background noise will lead to:

- Wrong language detection (e.g., French speech transcribed as English)
- Missing or garbled words
- Low confidence transcriptions

**Test your microphone**: Record yourself with another app (like Audacity or your OS voice recorder) and listen back. If it sounds noisy or muffled, Murmure will struggle too.

### Best Practices

- Speak at a natural pace - no need to slow down
- Keep a consistent distance from the microphone
- Minimize background noise (close windows, mute other audio sources)
- Use a dedicated microphone rather than your laptop's built-in one when possible
- For long dictation, pause briefly between sentences

### Why Does Murmure Transcribe in the Wrong Language?

This is the most common issue reported. Parakeet detects language from audio characteristics. When audio quality is poor, it tends to default to English.

**Solutions:**

1. Check your microphone quality (test with another recording app)
2. Make sure the correct microphone is selected in Settings > System
3. Reduce background noise
4. Increase your microphone volume in your OS sound settings

See [Transcription Troubleshooting](../troubleshooting/transcription.md) for a detailed diagnostic guide.

## Recording Overlay

While recording, Murmure displays a small overlay on your screen. You can configure this in Settings > System:

- **Always**: Overlay is always visible
- **Recording**: Overlay only appears during recording
- **Never**: No overlay

The overlay position can be changed in the settings.

## History

Murmure keeps your last 5 transcriptions accessible in the sidebar. Click any entry to copy it to your clipboard. Your transcription history also shows:

- Words per minute
- Total words transcribed
- Estimated data savings vs. cloud services
