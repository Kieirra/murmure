export interface CliCommandDescriptor {
    id: string;
    label: string;
    command: string;
}

// New CLI flags must be added here AND in the Rust `CliCommand` enum and tauri.conf.json.
export const CLI_COMMANDS: CliCommandDescriptor[] = [
    { id: 'transcription', label: 'Toggle transcription', command: 'murmure --transcription' },
    { id: 'transcription-llm', label: 'Toggle transcription (LLM)', command: 'murmure --transcription-llm' },
    {
        id: 'transcription-command',
        label: 'Toggle transcription (Command)',
        command: 'murmure --transcription-command',
    },
    { id: 'paste-last', label: 'Paste last transcript', command: 'murmure --paste-last' },
    { id: 'cancel', label: 'Cancel recording', command: 'murmure --cancel' },
    { id: 'voice-mode', label: 'Toggle Voice Mode', command: 'murmure --voice-mode' },
    { id: 'llm-mode-1', label: 'Switch to LLM mode 1', command: 'murmure --llm-mode 1' },
    { id: 'llm-mode-2', label: 'Switch to LLM mode 2', command: 'murmure --llm-mode 2' },
    { id: 'llm-mode-3', label: 'Switch to LLM mode 3', command: 'murmure --llm-mode 3' },
    { id: 'llm-mode-4', label: 'Switch to LLM mode 4', command: 'murmure --llm-mode 4' },
];

export const CLI_DOC_URL = 'https://docs.murmure.app/configure-shortcuts-on-linux/';
