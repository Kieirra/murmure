export const restoreWord = (words: string[], word: string) => {
    if (words.some((w) => w.toLowerCase() === word.toLowerCase())) {
        return words;
    }
    return [...words, word];
};
