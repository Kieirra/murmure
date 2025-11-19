import { useEffect, useState } from 'react';
import { Input } from '../../../components/input';
import { Button } from '../../../components/button';
import { BookText } from 'lucide-react';
import { invoke } from '@tauri-apps/api/core';
import { toast } from 'react-toastify';
import { Page } from '@/components/page';
import { Typography } from '@/components/typography';
import { useTranslation } from '@/i18n';

export const CustomDictionary = () => {
    const [customWords, setCustomWords] = useState<string[]>([]);
    const [newWord, setNewWord] = useState('');
    const { t } = useTranslation();

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

    const handleAddWord = () => {
        const trimmed = newWord.trim();
        if (!trimmed) return;
        if (customWords.includes(trimmed)) return;
        persist([...customWords, trimmed]);
        setNewWord('');
    };

    const handleRemoveWord = (word: string) => {
        const next = customWords.filter((w) => w !== word);
        persist(next);
    };

    const handleKeyDown = (e: React.KeyboardEvent) => {
        if (e.key === 'Enter') {
            e.preventDefault();
            handleAddWord();
        }
    };

    return (
        <main className="space-y-8">
            <Page.Header>
                <Typography.MainTitle data-testid="dictionary-title">
                    {t('Custom Dictionary')}
                </Typography.MainTitle>
                <Typography.Paragraph className="text-zinc-400">
                    {t(
                        'Personalize your Murmure experience by adding technical terms, names, or specialized vocabulary to the dictionary (optimized for both English and French).'
                    )}
                </Typography.Paragraph>
            </Page.Header>

            <div className="space-y-2 w-full">
                <Typography.Title className="space-x-2">
                    <BookText className="w-4 h-4 text-zinc-400 inline-block" />
                    <span>{t('Custom Words')}</span>
                </Typography.Title>
                <Typography.Paragraph>
                    {t('Add technical terms, names, or specialized vocabulary')}
                </Typography.Paragraph>
                <div className="flex items-center gap-2">
                    <Input
                        type="text"
                        value={newWord}
                        onChange={(e) => setNewWord(e.target.value)}
                        onKeyDown={handleKeyDown}
                        placeholder={t('Add a word')}
                        data-testid="custom-dictionary-input"
                    />
                    <Button
                        variant="outline"
                        onClick={handleAddWord}
                        disabled={!newWord.trim()}
                        data-testid="custom-dictionary-add-button"
                    >
                        {t('Add')}
                    </Button>
                </div>
                {customWords.length > 0 && (
                    <div className="flex flex-wrap gap-2 mt-4">
                        {customWords.map((word) => (
                            <button
                                key={word}
                                onClick={() => handleRemoveWord(word)}
                                className="inline-flex items-center gap-1.5 px-3 py-1.5 text-xs bg-zinc-800 hover:bg-zinc-700 text-zinc-300 rounded-md border border-zinc-700 transition-colors"
                                data-testid={`custom-dictionary-remove-button-${word}`}
                            >
                                <span
                                    data-testid={`custom-dictionary-word-${word}`}
                                >
                                    {word}
                                </span>
                                <span className="text-zinc-500">×</span>
                            </button>
                        ))}
                    </div>
                )}
            </div>
        </main>
    );
};
