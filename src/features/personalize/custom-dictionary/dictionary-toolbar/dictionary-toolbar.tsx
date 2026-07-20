import { useState } from 'react';
import { AlertTriangle, MoreHorizontalIcon, Trash2 } from 'lucide-react';
import { invoke } from '@tauri-apps/api/core';
import { open, save } from '@tauri-apps/plugin-dialog';
import { toast } from 'react-toastify';
import { Input } from '@/components/input';
import { Page } from '@/components/page';
import { Button } from '@/components/button';
import { ExternalLink } from '@/components/external-link';
import { InternalLink } from '@/components/internal-link';
import { useTranslation } from '@/i18n';
import {
    DropdownMenu,
    DropdownMenuTrigger,
    DropdownMenuContent,
    DropdownMenuGroup,
    DropdownMenuItem,
    DropdownMenuSeparator,
} from '@/components/dropdown-menu';
import {
    Dialog,
    DialogClose,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
} from '@/components/dialog';

interface DictionaryToolbarProps {
    wordCount: number;
    onAdd: (word: string) => void;
    onWordsChanged: (words: string[]) => void;
    onClear: () => void;
}

export const DictionaryToolbar = ({ wordCount, onAdd, onWordsChanged, onClear }: DictionaryToolbarProps) => {
    const { t } = useTranslation();
    const [newWord, setNewWord] = useState('');
    const [clearDialogOpen, setClearDialogOpen] = useState(false);
    const containsDigit = /\d/.test(newWord);
    const hasMultipleSpaces =
        newWord
            .trim()
            .split('')
            .filter((c) => c === ' ').length > 1;

    const handleAddWord = () => {
        onAdd(newWord);
        setNewWord('');
    };

    const handleKeyDown = (e: React.KeyboardEvent) => {
        if (e.key === 'Enter') {
            e.preventDefault();
            handleAddWord();
        }
    };

    const handleClearDictionary = () => {
        onClear();
        setClearDialogOpen(false);
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
                filePath,
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
            await invoke('import_dictionary', { filePath });
            const words = await invoke<string[]>('get_dictionary');
            onWordsChanged(words ?? []);
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
        <>
            <div className="flex items-center gap-2">
                <Input
                    type="text"
                    value={newWord}
                    onChange={(e) => setNewWord(e.target.value)}
                    onKeyDown={handleKeyDown}
                    placeholder={t('Add a word')}
                    data-testid="custom-dictionary-input"
                />
                <Page.PrimaryButton
                    size="default"
                    className="!px-4"
                    onClick={handleAddWord}
                    disabled={!newWord.trim()}
                    data-testid="custom-dictionary-add-button"
                >
                    {t('Add')}
                </Page.PrimaryButton>
                <DropdownMenu modal={true}>
                    <DropdownMenuTrigger asChild>
                        <Page.SecondaryButton variant="outline" aria-label="Open menu" size="icon-sm">
                            <MoreHorizontalIcon />
                        </Page.SecondaryButton>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent className="w-40 bg-background border-border text-foreground" align="end">
                        <DropdownMenuGroup>
                            <DropdownMenuItem
                                onSelect={handleImportDictionary}
                                className="focus:bg-card focus:text-foreground"
                            >
                                {t('Import Dictionary')}
                            </DropdownMenuItem>
                            <DropdownMenuItem
                                onSelect={handleExportDictionary}
                                className="focus:bg-card focus:text-foreground"
                            >
                                {t('Export Dictionary')}
                            </DropdownMenuItem>
                            <DropdownMenuSeparator className="bg-accent" />
                            <DropdownMenuItem
                                disabled={wordCount === 0}
                                onSelect={() => setClearDialogOpen(true)}
                                className="focus:bg-card text-red-400 focus:text-red-300"
                            >
                                <Trash2 className="w-4 h-4" />
                                {t('Clear Dictionary')}
                            </DropdownMenuItem>
                        </DropdownMenuGroup>
                    </DropdownMenuContent>
                </DropdownMenu>
                <Dialog open={clearDialogOpen} onOpenChange={setClearDialogOpen}>
                    <DialogContent>
                        <DialogHeader>
                            <DialogTitle>{t('Clear Dictionary')}</DialogTitle>
                            <DialogDescription>
                                {t(
                                    'Are you sure you want to remove all words from the dictionary? This action cannot be undone.'
                                )}
                            </DialogDescription>
                        </DialogHeader>
                        <DialogFooter>
                            <DialogClose asChild>
                                <Button
                                    variant="outline"
                                    className="bg-card border border-border hover:bg-accent hover:text-foreground"
                                >
                                    {t('Cancel')}
                                </Button>
                            </DialogClose>
                            <Button variant="destructive" onClick={handleClearDictionary}>
                                {t('Clear')}
                            </Button>
                        </DialogFooter>
                    </DialogContent>
                </Dialog>
            </div>
            <p className="text-xs text-muted-foreground">
                {t('Letters, accents, punctuation and hyphens are supported. Use one space for two-word terms.')}
            </p>
            {containsDigit && (
                <div
                    className="flex items-start gap-1.5 text-xs text-yellow-300/90"
                    data-testid="custom-dictionary-number-warning"
                >
                    <AlertTriangle className="w-3 h-3 shrink-0 mt-0.5" />
                    <span>
                        {t('Numbers are not supported in the dictionary. Use')}{' '}
                        <InternalLink to="/personalize/formatting-rules" hash="custom-rules">
                            {t('Formatting Rules')}
                        </InternalLink>{' '}
                        {t('to handle words with digits.')}{' '}
                        <ExternalLink href="https://docs.murmure.app/features/formatting-rules/">
                            {t('Learn more')}
                        </ExternalLink>
                    </span>
                </div>
            )}
            {hasMultipleSpaces && (
                <div
                    className="flex items-start gap-1.5 text-xs text-yellow-300/90"
                    data-testid="custom-dictionary-space-warning"
                >
                    <AlertTriangle className="w-3 h-3 shrink-0 mt-0.5" />
                    <span>{t('Only single words or two-word pairs are allowed.')}</span>
                </div>
            )}
        </>
    );
};
