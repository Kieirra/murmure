export const GROUPING_THRESHOLD = 10;

export const groupByLetter = (words: string[]) => {
    const groups = new Map<string, string[]>();
    for (const word of words) {
        const letter = word.charAt(0).toUpperCase();
        const existing = groups.get(letter);
        if (existing != null) {
            existing.push(word);
        } else {
            groups.set(letter, [word]);
        }
    }
    return [...groups.entries()]
        .map(([letter, groupedWords]) => ({ letter, words: groupedWords }))
        .sort((a, b) => a.letter.localeCompare(b.letter, undefined, { sensitivity: 'base' }));
};
