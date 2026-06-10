# Dictionary

![Dictionary](../assets/dictionary.png)

The dictionary helps Murmure recognize words it might otherwise miss or misspell - proper nouns, technical terms, brand names, etc.

## How It Works

Dictionary words are boosted directly inside the ASR decoding: when the audio is compatible with one of your entries, Parakeet is nudged toward transcribing it. A spelling correction pass then fixes near-misses, but only when the model was not confident about what it heard, so common words you clearly pronounced are never replaced by dictionary entries.

**Example**: You add "Kieirra" to the dictionary. When you say "Kieirra" and Parakeet would have transcribed "Kierra" or "Kyera", the dictionary corrects it to "Kieirra".

## Adding Words

1. Go to **Settings** > **Dictionary** (or the Personalize section)
2. Type the word and click Add
3. The word is immediately active for future transcriptions

## Best Practices

!!! warning "Less is more"
    The dictionary works best with a small, targeted list of words. Every entry is a candidate the decoder considers, so hundreds of entries increase the risk of false positives.

**Do:**

- Add proper nouns that Parakeet consistently gets wrong (company names, people's names)
- Add technical terms specific to your field
- Add acronyms that should be capitalized (e.g., "LAMOTRIGINE", "NestJS")

**Don't:**

- Add entire medication lists or full glossaries
- Add common words that Parakeet already handles well
- Add words with numbers or special characters (not supported - use [Formatting Rules](formatting-rules.md) instead)

### What Happens With a Large Dictionary?

Safeguards scale down automatically: the decoding boost gets weaker as the dictionary grows, and the spelling correction pass is disabled entirely beyond 100 entries (exact matches keep their dictionary casing). Only add words that are frequently mis-recognized.

## Dictionary Limitations

- **Alphabetical characters only** - Numbers, hyphens, and special characters are not supported in dictionary entries
- **Single words only** - Multi-word phrases are not supported
- **No context understanding** - Words are matched by sound and spelling, not by meaning

For complex replacements (multi-word, with numbers, context-dependent), use [Formatting Rules](formatting-rules.md) with regex instead.

## Import / Export

You can import and export your dictionary for backup or sharing:

- **Export**: Downloads your dictionary as a file
- **Import**: Loads words from a previously exported file
- **Clear all**: Removes all dictionary entries

Medical presets are available for common medical terminology.
