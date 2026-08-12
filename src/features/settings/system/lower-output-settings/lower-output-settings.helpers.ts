export const UNSUPPORTED_REASONS: Record<string, string> = {
    no_audio_server: 'No supported audio server detected (PipeWire or PulseAudio required).',
    no_volume_control: 'The current output device does not expose a volume control.',
    unsupported_platform: 'Volume control is not available on this platform.',
};
