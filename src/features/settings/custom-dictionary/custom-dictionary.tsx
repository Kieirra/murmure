import { useEffect, useState } from 'react';
import { Input } from '../../../components/input';
import { BookText, MoreHorizontalIcon } from 'lucide-react';
import { invoke } from '@tauri-apps/api/core';
import { toast } from 'react-toastify';
import { Page } from '@/components/page';
import { Typography } from '@/components/typography';
import { useTranslation } from '@/i18n';
import { open, save } from '@tauri-apps/plugin-dialog';
import {
    DropdownMenu,
    DropdownMenuTrigger,
    DropdownMenuContent,
    DropdownMenuGroup,
    DropdownMenuItem,
} from '@/components/dropdown-menu';

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

    const isValidWord = (word: string): boolean => {
        return word.split('').every((char) => /\p{L}/u.test(char));
    };

    const handleAddWord = () => {
        const trimmed = newWord.trim().toLowerCase();
        if (trimmed.length === 0) return;
        if (customWords.includes(trimmed)) return;
        if (!isValidWord(trimmed)) {
            toast.error(
                t(
                    'Invalid word format. Words must contain only letters (a-z, A-Z)'
                )
            );
            return;
        }
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
            const filePath = await save({
                title: t('Select file to export dictionary'),
                filters: [
                    {
                        name: 'CSV files',
                        extensions: ['csv', 'CSV'],
                    },
                ],
                defaultPath: 'murmure-dictionary.csv',
            });
            if (filePath == null) {
                return;
            }
            await invoke('export_dictionary', {
                filePath: filePath,
            });
            toast.success(t('Dictionary exported successfully'), {
                autoClose: 2000,
            });
        } catch (error) {
            toast.error(t('Failed to export dictionary') + ' : ' + error);
        }
    };

    const persistImportedDictionary = async (filePath: string) => {
        try {
            await invoke('import_dictionary', { filePath: filePath });
            const words = await invoke<string[]>('get_dictionary');
            setCustomWords(words ?? []);
            toast.info(t('Dictionary updated'), {
                autoClose: 1500,
            });
        } catch (error) {
            toast.error(t('Failed to update dictionary') + ' : ' + error);
        }
    };
    const handleImportDictionary = async () => {
        try {
            const file = await open({
                directory: false,
                multiple: false,
                title: t('Select file to import dictionary'),
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
            toast.error(t('Failed to import dictionary') + ' : ' + error);
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
                    <DropdownMenu modal={true}>
                        <DropdownMenuTrigger asChild>
                            <Page.SecondaryButton
                                variant="outline"
                                aria-label="Open menu"
                                size="icon-sm"
                            >
                                <MoreHorizontalIcon />
                            </Page.SecondaryButton>
                        </DropdownMenuTrigger>
                        <DropdownMenuContent
                            className="w-40 bg-zinc-900 border-zinc-700 text-zinc-300"
                            align="end"
                        >
                            <DropdownMenuGroup>
                                <DropdownMenuItem
                                    onSelect={handleImportDictionary}
                                    className="focus:bg-zinc-800 focus:text-zinc-200"
                                >
                                    {t('Import Dictionary')}
                                </DropdownMenuItem>
                                <DropdownMenuItem
                                    onSelect={handleExportDictionary}
                                    className="focus:bg-zinc-800 focus:text-zinc-200"
                                >
                                    {t('Export Dictionary')}
                                </DropdownMenuItem>
                            </DropdownMenuGroup>
                        </DropdownMenuContent>
                    </DropdownMenu>
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
