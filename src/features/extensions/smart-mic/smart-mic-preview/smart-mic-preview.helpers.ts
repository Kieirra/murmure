const FRENCH_SAMPLE = 'Bonjour, comment allez-vous ?';
const ENGLISH_SAMPLE = 'Hello, how are you?';

interface TranslationSample {
    said: string;
    read: string;
}

export const getTranslationSample = (language: string | undefined): TranslationSample =>
    language?.startsWith('fr') === true
        ? { said: FRENCH_SAMPLE, read: ENGLISH_SAMPLE }
        : { said: ENGLISH_SAMPLE, read: FRENCH_SAMPLE };
