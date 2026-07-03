import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { toast } from 'react-toastify';
import { X } from 'lucide-react';
import { Page } from '@/components/page';
import { Typography } from '@/components/typography';
import { useTranslation } from '@/i18n';
import { DictionaryToolbar } from './dictionary-toolbar/dictionary-toolbar';
import { DictionaryEmptyState } from './dictionary-empty-state/dictionary-empty-state';
import { DictionaryWordList } from './dictionary-word-list/dictionary-word-list';
import { useMarkRevampSeen } from './hooks/use-mark-revamp-seen';

export const CustomDictionary = () => {
    const [customWords, setCustomWords] = useState<string[]>([]);
    const [dismissed, setDismissed] = useState(false);
    const { t } = useTranslation();
    const { showRevampNotice } = useMarkRevampSeen();

    useEffect(() => {
        invoke<string[]>('get_dictionary').then((words) => {
            setCustomWords(words ?? []);
        });
    }, []);

    const persist = (next: string[]) => {
        setCustomWords(next);
        invoke('set_dictionary', { dictionary: next })
            .then(() =>
                toast.info(t('Dictionary updated'), {
                    autoClose: 1500,
                })
            )
            .catch(() => toast.error(t('Failed to update dictionary')));
    };

    const isValidWord = (word: string) => {
        return word.split('').every((char) => /\p{L}/u.test(char));
    };

    const addWord = (word: string) => {
        const trimmed = word.trim();
        if (trimmed.length === 0) return;
        if (customWords.some((w) => w.toLowerCase() === trimmed.toLowerCase())) {
            toast.warning(t('Word already exists in the dictionary'));
            return;
        }
        if (!isValidWord(trimmed)) {
            toast.error(t('Invalid word format. Words must contain only letters (a-z, A-Z)'));
            return;
        }
        persist([...customWords, trimmed]);
    };

    const handleRemoveWord = (word: string) => {
        persist(customWords.filter((w) => w !== word));
    };

    const sortedWords = [...customWords].sort((a, b) => a.localeCompare(b, undefined, { sensitivity: 'base' }));

    return (
        <main className="relative flex flex-col space-y-6 min-h-[calc(100svh-5rem)]">
            <Page.Header>
                <div className="flex flex-col gap-6">
                    <Typography.MainTitle className="!mb-0" data-testid="dictionary-title">
                        {t('Dictionary')}
                    </Typography.MainTitle>
                    {showRevampNotice && !dismissed && (
                        <div
                            className="flex w-full items-start justify-between gap-2 rounded-md border border-sky-500/30 bg-sky-500/10 px-3 py-1.5"
                            data-testid="dictionary-revamp-banner"
                        >
                            <Typography.Paragraph className="flex-1 text-sm text-sky-300">
                                {t(
                                    'The dictionary has been improved. Your words now enrich the recognition model directly during transcription, instead of being corrected in post-processing.'
                                )}
                            </Typography.Paragraph>
                            <button
                                type="button"
                                onClick={() => setDismissed(true)}
                                aria-label={t('Close')}
                                className="shrink-0 cursor-pointer text-sky-300"
                                data-testid="dictionary-revamp-banner-close"
                            >
                                <X className="h-4 w-4" />
                            </button>
                        </div>
                    )}
                    <Typography.Paragraph className="text-muted-foreground">
                        {t(
                            "Add technical terms, names, or specialized vocabulary that is poorly recognized to enrich the AI's vocabulary."
                        )}
                    </Typography.Paragraph>
                </div>
            </Page.Header>

            <div className="flex flex-col flex-1 space-y-2 w-full">
                <DictionaryToolbar
                    wordCount={customWords.length}
                    onAdd={addWord}
                    onWordsChanged={setCustomWords}
                    onClear={() => persist([])}
                />
                {sortedWords.length === 0 && (
                    <div className="absolute inset-0 flex items-center justify-center pointer-events-none">
                        <div className="pointer-events-auto">
                            <DictionaryEmptyState onAdd={addWord} />
                        </div>
                    </div>
                )}
                {sortedWords.length > 0 && <DictionaryWordList words={sortedWords} onRemove={handleRemoveWord} />}
            </div>
        </main>
    );
};
