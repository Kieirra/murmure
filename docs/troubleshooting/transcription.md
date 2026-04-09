# Transcription Issues

## Murmure Transcribes in the Wrong Language

This is the most reported issue. You speak French (or another language), but Murmure transcribes in English.

### Why This Happens

Parakeet detects the language automatically from audio characteristics. When the audio quality is poor - noisy background, low volume, bad microphone - the model tends to default to English.

### Diagnostic Steps

**Step 1: Test your microphone**

Record yourself with another app (Audacity, your OS voice recorder) and listen back. If the audio is noisy, muffled, or quiet, that's the problem.

**Step 2: Check the correct microphone is selected**

Go to **Settings** > **System** > **Microphone**. If set to "Automatic", try selecting your specific microphone manually.

**Step 3: Increase microphone volume**

Open your OS sound settings and increase the input volume for your microphone.

**Step 4: Reduce background noise**

Close windows, mute other audio sources, and move away from fans or air conditioning.

### Microphones Known to Cause Issues

- Laptop built-in microphones (especially in noisy environments)
- DJI Mini wireless microphones
- Cheap USB microphones without noise cancellation
- Virtual microphones that add processing (check if the raw microphone works better)

## Transcription Cuts Off Early

### In Push-to-Talk Mode

Make sure you're holding the shortcut for the entire duration you want to record. Releasing the key stops the recording.

### In Voice-Activated Mode

Voice mode stops recording when the volume drops below the silence threshold. If your microphone volume is too low, it triggers early cutoff.

**Fix**: Increase your microphone volume in your OS sound settings, or adjust the silence timeout in Voice Mode settings (default: 1.5 seconds).

### Maximum Duration

Recordings are limited to **5 minutes**. After 5 minutes, the recording automatically stops and transcription begins.

## Transcription Error: "ORT Error"

This error from the ONNX Runtime can have several causes:

| Error Variant | Cause | Fix |
|---|---|---|
| "Non-zero status code returned while running Pad node" | Audio too short (0 samples) | Recording stopped instantly - check shortcut behavior |
| ORT error on start | Model files corrupted | Reinstall Murmure |
| ORT error on Linux | Wayland permission issues | Switch to X11 |

## Empty or No Transcription

- Check that your microphone is not muted
- Check that the correct microphone is selected in Settings
- Try recording with another app to verify your mic works
- On macOS, verify microphone permission is granted (System Settings > Privacy & Security > Microphone)

## Poor Quality for Specific Languages

Parakeet's accuracy varies by language. French, English, German, and Swedish have the highest accuracy. Greek and some smaller languages have noticeably lower accuracy.

There is currently no way to force a specific language or improve accuracy for lower-performing languages.
