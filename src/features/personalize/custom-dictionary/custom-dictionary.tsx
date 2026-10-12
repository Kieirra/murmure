import { useEffect, useRef, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { toast } from 'react-toastify';
import { X } from 'lucide-react';
import { Page } from '@/components/page';
import { Typography } from '@/components/typography';
import { useTranslation } from '@/i18n';
import { DictionaryToolbar } from './dictionary-toolbar/dictionary-toolbar';
import { DictionaryPanelHeader } from './dictionary-panel-header/dictionary-panel-header';
import { DictionaryEmptyState } from './dictionary-empty-state/dictionary-empty-state';
import { DictionaryWordList } from './dictionary-word-list/dictionary-word-list';
import { DictionaryRemovedToast } from './dictionary-removed-toast/dictionary-removed-toast';
import { useMarkRevampSeen } from './hooks/use-mark-revamp-seen';
import { restoreWord } from './custom-dictionary.helpers';

export const CustomDictionary = () => {
    const [customWords, setCustomWords] = useState<string[]>([]);
    const customWordsRef = useRef<string[]>([]);
    const [dismissed, setDismissed] = useState(false);
    const { t } = useTranslation();
    const { showRevampNotice } = useMarkRevampSeen();

    useEffect(() => {
        invoke<string[]>('get_dictionary').then((words) => {
            setCustomWords(words ?? []);
        });
    }, []);

    useEffect(() => {
        customWordsRef.current = customWords;
    }, [customWords]);

    const persist = (next: string[], onSaved?: () => void) => {
        setCustomWords(next);
        invoke('set_dictionary', { dictionary: next })
            .then(() => onSaved?.())
            .catch(() => toast.error(t('Failed to update dictionary')));
    };

    const notifyUpdated = () =>
        toast.info(t('Dictionary updated'), {
            autoClose: 1500,
        });

    const addWord = (word: string) => {
        const trimmed = word.trim();
        if (trimmed.length === 0) return;
        if (customWords.some((w) => w.toLowerCase() === trimmed.toLowerCase())) {
            toast.warning(t('Word already exists in the dictionary'));
            return;
        }
        if (/\d/.test(trimmed)) {
            toast.error(t('Numbers are not supported in the dictionary.'));
            return;
        }
        if (trimmed.split('').filter((c) => c === ' ').length > 1) {
            toast.error(t('Only single words or two-word pairs are allowed.'));
            return;
        }
        persist([...customWords, trimmed], notifyUpdated);
    };

    const undoRemoveWord = (word: string) => {
        persist(restoreWord(customWordsRef.current, word));
    };

    const handleRemoveWord = (word: string) => {
        persist(
            customWords.filter((w) => w !== word),
            () =>
                toast.info(
                    ({ closeToast }) => (
                        <DictionaryRemovedToast
                            word={word}
                            onUndo={() => {
                                undoRemoveWord(word);
                                closeToast();
                            }}
                        />
                    ),
                    { autoClose: 5000 }
                )
        );
    };

    const sortedWords = [...customWords].sort((a, b) => a.localeCompare(b, undefined, { sensitivity: 'base' }));

    return (
        <main className="flex flex-col space-y-6">
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
                </div>
            </Page.Header>

            <div className="flex flex-col gap-3 w-full">
                <DictionaryToolbar onAdd={addWord} />
                <div
                    className="rounded-md border border-surface-border bg-surface"
                    data-testid="custom-dictionary-panel"
                >
                    <DictionaryPanelHeader
                        wordCount={customWords.length}
                        onWordsChanged={setCustomWords}
                        onClear={() => persist([], notifyUpdated)}
                    />
                    <div className="p-3">
                        {sortedWords.length === 0 && <DictionaryEmptyState onAdd={addWord} />}
                        {sortedWords.length > 0 && (
                            <DictionaryWordList words={sortedWords} onRemove={handleRemoveWord} />
                        )}
                    </div>
                </div>
            </div>
        </main>
    );
};
