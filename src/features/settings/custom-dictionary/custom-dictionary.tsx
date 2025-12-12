import { useEffect, useState } from 'react';
import { Input } from '../../../components/input';
import { BookText } from 'lucide-react';
import { invoke } from '@tauri-apps/api/core';
import { toast } from 'react-toastify';
import { Page } from '@/components/page';
import { Typography } from '@/components/typography';
import { useTranslation } from '@/i18n';
import { open } from '@tauri-apps/plugin-dialog';

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

    const handleExportDictionary = async () => {
        try {
            const directory = await open({
                directory: true,
                multiple: false,
                title: 'Select directory to export dictionary',
            });
            if (directory == null) {
                return;
            }
            await invoke('export_dictionary', {
                directory: directory as string,
            });
            toast.success(t('Dictionary exported successfully'), {
                autoClose: 2000,
            });
        } catch (error) {
            console.error('Error exporting dictionary:', error);
            toast.error(t('Failed to export dictionary'));
        }
    };

    const persistImportedDictionary = async (filePath: string) => {
        invoke('import_dictionary', { filePath: filePath })
            .then(() =>
                toast.info(t('Dictionary updated'), {
                    autoClose: 1500,
                })
            )
            .then(() => {
                invoke<string[]>('get_dictionary').then((words) => {
                    setCustomWords(words ?? []);
                });
            })
            .catch((error) => {
                console.error('Error importing dictionary:', error);
                toast.error(t('Failed to update dictionary'));
            });
    };
    const handleImportDictionary = async () => {
        try {
            const file = await open({
                directory: false,
                multiple: false,
                title: 'Select file to import dictionary',
                filters: [
                    {
                        name: 'CSV files',
                        extensions: ['csv', 'CSV'],
                    },
                ],
            });
            if (file == null) {
                return;
            }
            await persistImportedDictionary(file as string);
        } catch (error) {
            console.error('Error importing dictionary:', error);
            toast.error(t('Failed to import dictionary'));
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
                    <Page.SecondaryButton
                        variant="outline"
                        onClick={handleAddWord}
                        disabled={!newWord.trim()}
                        data-testid="custom-dictionary-add-button"
                    >
                        {t('Add')}
                    </Page.SecondaryButton>
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

            <div className="space-y-2 w-full">
                <Typography.Title className="space-x-2">
                    <BookText className="w-4 h-4 text-zinc-400 inline-block" />
                    <span>{t('Import Dictionary')}</span>
                </Typography.Title>
                <Typography.Paragraph>
                    {t('Import a file containing a list of words')}
                </Typography.Paragraph>
                <Page.SecondaryButton
                        data-testid="custom-dictionary-import-button"
                        onClick={handleImportDictionary}
                        variant="outline"
                    >
                        Import
                    </Page.SecondaryButton>
            </div>
            <div className="space-y-2 w-full">
                <Typography.Title className="space-x-2">
                    <BookText className="w-4 h-4 text-zinc-400 inline-block" />
                    <span>{t('Export Dictionary')}</span>
                </Typography.Title>
                <Typography.Paragraph>
                    {t('Export dictionary to selected directory')}
                </Typography.Paragraph>
                <Page.SecondaryButton
                    data-testid="custom-dictionary-export-button"
                    onClick={handleExportDictionary}
                    variant="outline"
                >
                    Export
                </Page.SecondaryButton>
            </div>
        </main>
    );
};
