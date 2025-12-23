import { invoke } from '@tauri-apps/api/core';
import { Typography } from '@/components/typography';
import { Button } from '@/components/button';
import {
    Dialog,
    DialogClose,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from '@/components/dialog';
import { toast } from 'react-toastify';
import { formatTime } from './history.helpers';
import { useHistoryState } from './hooks/use-history-state';
import { InfoIcon, Trash2 } from 'lucide-react';
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/tooltip';
import { useTranslation } from '@/i18n';

export const History = () => {
    const { history } = useHistoryState();
    const { t } = useTranslation();

    const handleClearHistory = async () => {
        try {
            await invoke('clear_history');
            toast.info(t('History cleared'));
        } catch (error) {
            toast.error(t('Failed to clear history'));
            console.error('Clear history error:', error);
        }
    };

    return (
        <div className="space-y-2 w-full mb-8">
            <div className="flex items-center justify-between">
                <Typography.Title className="flex items-center gap-2">
                    {t('Recent activity')}{' '}
                    <Tooltip>
                        <TooltipTrigger asChild>
                            <InfoIcon className="size-4 inline-block text-zinc-400 cursor-pointer" />
                        </TooltipTrigger>
                        <TooltipContent>
                            <Typography.Paragraph className="text-zinc-100 text-xs">
                                {t(
                                    'All audio is deleted. No telemetry, no tracking. Only the last five text transcriptions are stored on your computer.'
                                )}
                            </Typography.Paragraph>
                        </TooltipContent>
                    </Tooltip>
                </Typography.Title>
                <Dialog>
                    <DialogTrigger asChild>
                        <Trash2 className="size-4 cursor-pointer hover:text-zinc-100 text-zinc-400 transition-colors" />
                    </DialogTrigger>
                    <DialogContent>
                        <DialogHeader>
                            <DialogTitle>{t('Clear History')}</DialogTitle>
                            <DialogDescription>
                                {t(
                                    'Are you sure you want to clear all transcription history? This action cannot be undone.'
                                )}
                            </DialogDescription>
                        </DialogHeader>
                        <DialogFooter>
                            <DialogClose asChild>
                                <Button
                                    variant="outline"
                                    className="bg-zinc-800 border border-zinc-700 hover:bg-zinc-700 hover:text-zinc-100"
                                >
                                    {t('Cancel')}
                                </Button>
                            </DialogClose>
                            <DialogClose asChild>
                                <Button
                                    variant="destructive"
                                    onClick={handleClearHistory}
                                >
                                    {t('Clear')}
                                </Button>
                            </DialogClose>
                        </DialogFooter>
                    </DialogContent>
                </Dialog>
            </div>
            {history.length === 0 ? (
                <Typography.Paragraph>
                    {t('No transcriptions yet')}
                </Typography.Paragraph>
            ) : (
                <div className="space-y-2">
                    {history.map((entry) => (
                        <button
                            key={entry.id}
                            className="w-full text-left rounded-md border border-zinc-700 p-3 hover:bg-zinc-800 cursor-pointer"
                            onClick={async () => {
                                if (!entry.text) return;
                                try {
                                    await navigator.clipboard.writeText(
                                        entry.text
                                    );
                                    toast.info(t('Copied to clipboard'), {
                                        autoClose: 1500,
                                    });
                                } catch {
                                    toast.error(t('Failed to copy'));
                                }
                            }}
                        >
                            <div className="flex items-start justify-between gap-3">
                                <Typography.Paragraph>
                                    {entry.text === '' ? (
                                        <span className="italic text-xs">
                                            {t('(Empty transcription)')}
                                        </span>
                                    ) : (
                                        entry.text
                                    )}
                                </Typography.Paragraph>
                                <Typography.Paragraph className="text-xs block w-20 text-right">
                                    {formatTime(entry.timestamp)}
                                </Typography.Paragraph>
                            </div>
                        </button>
                    ))}
                </div>
            )}
        </div>
    );
};
