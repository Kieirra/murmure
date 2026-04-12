# Formatting Rules

![Formatting Rules](../assets/formatting-rules.png)

Formatting rules automatically transform your transcription before it's inserted. They're more powerful than the dictionary and support regex.

## Built-in Options

These toggles are available in **Settings** > **Formatting Rules**:

| Option                       | Description                                         |
| ---------------------------- | --------------------------------------------------- |
| **Trailing space**           | Adds a space after the transcription                |
| **Space before punctuation** | Adds a space before `?` and `!` (French typography) |
| **Text-to-number**           | Converts "vingt-trois" to "23", etc.                |

## Custom Rules

Create your own find-and-replace rules:

1. Go to **Settings** > **Formatting Rules**
2. Click "Add rule"
3. Enter the text to find and the replacement
4. Choose matching mode

### Matching Modes

- **Contains** - Matches anywhere in the text
- **Exact match** - Matches the entire transcription
- **Regex** (v1.8.0+) - Full regular expression support

### Regex Examples

**Dictation commands in French:**

| Pattern                          | Replacement | Effect                            |
| -------------------------------- | ----------- | --------------------------------- |
| `(?i)ouvrez les guillemets`      | `"`         | Voice command for opening quotes  |
| `(?i)fermez les guillemets`      | `"`         | Voice command for closing quotes  |
| `(?i)nouvelle ligne`             | `\n`        | Voice command for new line        |
| `(?i)point d'interrogation`      | `?`         | Voice command for question mark   |
| `(?i)(six\|6\|si) joint(e)?(s)?` | `ci-joint`  | Fix common French homophone error |

!!! tip
`(?i)` at the start makes the pattern case-insensitive.

## Rule Ordering

Rules are applied in order from top to bottom. You can drag and drop rules to reorder them (v1.8.0+).

This matters when one rule's output could be affected by another rule. For example, a "text-to-number" rule should run before a rule that formats numbers.

## When to Use Rules vs Dictionary

| Use Case                     | Dictionary | Formatting Rules |
| ---------------------------- | ---------- | ---------------- |
| Proper nouns (names, brands) | Yes        | -                |
| Multi-word replacements      | -          | Yes              |
| Words with numbers           | -          | Yes              |
| Regex patterns               | -          | Yes              |
| Voice commands ("new line")  | -          | Yes              |
| Simple word corrections      | Yes        | Yes              |

## Auto-Capitalization Behavior

By default, Parakeet capitalizes the first word and adds a period at the end. For short transcriptions (1-2 words used for inline corrections), this can be unwanted.

In v1.8.0+, a toggle removes capitalization and trailing punctuation for transcriptions under 3 words. This threshold is configurable.
