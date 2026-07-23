export enum DictionaryZone {
    Optimal = 'optimal',
    Reduced = 'reduced',
    Diluted = 'diluted',
}

export const OPTIMAL_MAX_WORDS = 50;
export const REDUCED_MAX_WORDS = 100;

export const getDictionaryZone = (wordCount: number) => {
    if (wordCount <= OPTIMAL_MAX_WORDS) return DictionaryZone.Optimal;
    if (wordCount <= REDUCED_MAX_WORDS) return DictionaryZone.Reduced;
    return DictionaryZone.Diluted;
};
