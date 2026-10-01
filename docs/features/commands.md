# Commands

Commands let you give a spoken instruction to the LLM. If text is selected, Murmure applies your instruction to it and replaces the selection. If nothing is selected, Murmure answers your request. Depending on your settings, the answer is written where your cursor is, or shown in the overlay.

!!! note "Command vs. Transform"
    Command asks you to speak your instruction every time. If you find yourself repeating the same instruction, save it as a prompt in a [Prompt Mode](llm-connect.md) prompt and use **Transform** instead: select text, press that prompt's Transform shortcut, and the saved prompt is applied without speaking.

## How It Works

1. **Select** text in any application
2. Press the **Command shortcut** (configure in Settings > Shortcuts)
3. **Say your command** (e.g., "translate to English", "fix the grammar", "make it shorter")
4. Murmure reads the selected text, sends it with your voice command to the LLM, and replaces the selection with the result

## Without Selection

If nothing is selected, your spoken request goes to the LLM as a question or as a request for a text. The answer is in plain text and in the language of your request. Depending on your settings, it is written where your cursor is, or shown in the overlay. Answers are short by default, one to three sentences. They are longer only when you ask for a text, such as an email or a list.

For example, with nothing selected, say "what does idempotent mean?". Murmure gives you a short definition.

## Turning It On

Open **Extensions** > **Command Mode** and click **Enable Command Mode**. The first time, Murmure asks you to connect a model (Ollama on your computer, or your own server). This setup is shared with Prompt Mode, so you only do it once.

If Command Mode is off, the Command shortcut does not start a recording. The overlay shows "Command Mode is off". Turning it off keeps your model and servers.

## Model

Commands have their own model, separate from the Prompt Mode prompts. Choose the provider (local or remote) and the model in **Extensions** > **Command Mode**. Changing the active prompt in Prompt Mode does not change the model used by commands.

If no model is chosen, the Command shortcut does not start a recording. The overlay shows "Command has no model" and Murmure asks you to choose a model in Command Mode.

## Requirements

Commands use the same model setup as [Prompt Mode](llm-connect.md), with Ollama or a remote server. Prompt Mode itself does not need to be on.

## Use Cases

- **Translation**: Select a paragraph, say "translate to English"
- **Grammar correction**: Select text, say "fix the grammar"
- **Reformulation**: Select text, say "make it more formal"
- **Summarization**: Select text, say "summarize in one sentence"
- **Code**: Select code, say "add error handling"

## Configuration

The command shortcut is separate from the recording shortcut. Set it in **Settings** > **Shortcuts** > **Command, free prompt**.

You can also trigger commands via [Voice Mode](voice-mode.md) by setting a wake word for the command action.
