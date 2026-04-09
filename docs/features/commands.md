# Commands

Commands let you modify selected text using voice instructions. Instead of transcribing to a new location, Murmure reads your selected text and applies a voice command to it.

## How It Works

1. **Select** text in any application
2. Press the **Command shortcut** (configure in Settings > Shortcuts)
3. **Say your command** (e.g., "translate to English", "fix the grammar", "make it shorter")
4. Murmure reads the selected text, sends it with your voice command to the LLM, and replaces the selection with the result

## Requirements

Commands require [LLM Connect](llm-connect.md) to be configured, since the transformation is done by the LLM.

## Use Cases

- **Translation**: Select a paragraph, say "translate to English"
- **Grammar correction**: Select text, say "fix the grammar"
- **Reformulation**: Select text, say "make it more formal"
- **Summarization**: Select text, say "summarize in one sentence"
- **Code**: Select code, say "add error handling"

## Configuration

The command shortcut is separate from the recording shortcut. Set it in **Settings** > **Shortcuts** > **Command**.

You can also trigger commands via [Voice Mode](voice-mode.md) by setting a wake word for the command action.
