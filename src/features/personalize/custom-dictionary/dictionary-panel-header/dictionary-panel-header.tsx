import { useState } from 'react';
import { Download, Trash2, Upload } from 'lucide-react';
import { invoke } from '@tauri-apps/api/core';
import { open, save } from '@tauri-apps/plugin-dialog';
import { toast } from 'react-toastify';
import { Button } from '@/components/button';
import { Page } from '@/components/page';
import { useTranslation } from '@/i18n';
import {
    Dialog,
    DialogClose,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
} from '@/components/dialog';
import { DictionaryCounter } from '../dictionary-counter/dictionary-counter';

interface DictionaryPanelHeaderProps {
    wordCount: number;
    onWordsChanged: (words: string[]) => void;
    onClear: () => void;
}

export const DictionaryPanelHeader = ({ wordCount, onWordsChanged, onClear }: DictionaryPanelHeaderProps) => {
    const { t } = useTranslation();
    const [clearDialogOpen, setClearDialogOpen] = useState(false);

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
                        name: t('Text file (.txt)'),
                        extensions: ['txt', 'TXT'],
                    },
                ],
                defaultPath: 'murmure-dictionary.txt',
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
                        name: t('Dictionary file (.txt, .csv)'),
                        extensions: ['txt', 'TXT', 'csv', 'CSV'],
                    },
                    {
                        name: t('All files'),
                        extensions: ['*'],
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
        <div className="flex items-start justify-between gap-4 rounded-t-[5px] border-b border-surface-border bg-surface-header px-3 py-1.5">
            <div className="flex-1 space-y-1 py-1.5">
                <DictionaryCounter wordCount={wordCount} />
            </div>
            <div className="flex shrink-0 items-center gap-1">
                <Button
                    variant="subtle"
                    size="sm"
                    onClick={handleImportDictionary}
                    data-testid="custom-dictionary-import-button"
                >
                    <Download />
                    {t('Import')}
                </Button>
                <Button
                    variant="subtle"
                    size="sm"
                    onClick={handleExportDictionary}
                    data-testid="custom-dictionary-export-button"
                >
                    <Upload />
                    {t('Export')}
                </Button>
                <Button
                    variant="subtle"
                    size="sm"
                    disabled={wordCount === 0}
                    onClick={() => setClearDialogOpen(true)}
                    className="text-red-400 hover:text-red-300"
                    data-testid="custom-dictionary-clear-button"
                >
                    <Trash2 />
                    {t('Clear')}
                </Button>
            </div>
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
                            <Page.SecondaryButton>{t('Cancel')}</Page.SecondaryButton>
                        </DialogClose>
                        <Button variant="destructive" onClick={handleClearDictionary}>
                            {t('Clear')}
                        </Button>
                    </DialogFooter>
                </DialogContent>
            </Dialog>
        </div>
    );
};
