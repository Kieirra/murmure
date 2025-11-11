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
import { toast } from 'sonner';
import { formatTime } from './history.helpers';
import { useHistoryState } from './hooks/use-history-state';
import { InfoIcon, Trash2 } from 'lucide-react';
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/tooltip';
import { useTranslation } from '@/i18n';

interface HistoryProps {}

export const History = ({}: HistoryProps) => {
    const { history } = useHistoryState();
    const { t } = useTranslation(['history', 'common']);

    const handleClearHistory = async () => {
        try {
            await invoke('clear_history');
            toast.success(t('common:messages.historyCleared'), {
                duration: 1500,
                closeButton: true,
            });
        } catch (error) {
            toast.error(t('common:messages.clearHistoryFailed'), {
                duration: 2000,
                closeButton: true,
            });
            console.error('Clear history error:', error);
        }
    };

    return (
        <div className="space-y-2 w-full">
            <div className="flex items-center justify-between">
                <Typography.Title className="flex items-center gap-2">
                    {t('history:title')}{' '}
                    <Tooltip>
                        <TooltipTrigger asChild>
                            <InfoIcon className="size-4 inline-block text-zinc-400 cursor-pointer" />
                        </TooltipTrigger>
                        <TooltipContent>
                            <Typography.Paragraph className="text-zinc-100 text-xs">
                                {t('history:tooltip')}
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
                            <DialogTitle>{t('history:clearDialog.title')}</DialogTitle>
                            <DialogDescription>
                                {t('history:clearDialog.description')}
                            </DialogDescription>
                        </DialogHeader>
                        <DialogFooter>
                            <DialogClose asChild>
                                <Button
                                    variant="outline"
                                    className="bg-zinc-800 border border-zinc-700 hover:bg-zinc-700 hover:text-zinc-100"
                                >
                                    {t('common:buttons.cancel')}
                                </Button>
                            </DialogClose>
                            <DialogClose asChild>
                                <Button
                                    variant="destructive"
                                    onClick={handleClearHistory}
                                >
                                    {t('common:buttons.clear')}
                                </Button>
                            </DialogClose>
                        </DialogFooter>
                    </DialogContent>
                </Dialog>
            </div>
            {history.length === 0 ? (
                <Typography.Paragraph>
                    {t('history:empty')}
                </Typography.Paragraph>
            ) : (
                <div className="space-y-2">
                    {history.map((entry) => (
                        <div
                            key={entry.id}
                            className="rounded-md border border-zinc-700 p-3 hover:bg-zinc-800 cursor-pointer"
                            onClick={async () => {
                                if (!entry.text) return;
                                try {
                                    await navigator.clipboard.writeText(
                                        entry.text
                                    );
                                    toast.success(t('common:messages.copied'), {
                                        duration: 1500,
                                        closeButton: true,
                                    });
                                } catch {
                                    toast.error(t('common:messages.copyFailed'), {
                                        duration: 2000,
                                        closeButton: true,
                                    });
                                }
                            }}
                        >
                            <div className="flex items-start justify-between gap-3">
                                <Typography.Paragraph>
                                    {entry.text === '' ? (
                                        <span className="italic text-xs">
                                            {t('history:emptyTranscription')}
                                        </span>
                                    ) : (
                                        entry.text
                                    )}
                                </Typography.Paragraph>
                                <Typography.Paragraph className="text-xs block w-20 text-right">
                                    {formatTime(entry.timestamp)}
                                </Typography.Paragraph>
                            </div>
                        </div>
                    ))}
                </div>
            )}
        </div>
    );
};
