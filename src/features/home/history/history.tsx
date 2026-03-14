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
import { useHistoryState, HistoryEntry } from './hooks/use-history-state';
import { HistoryItem } from './history-item';
import { InfoIcon, Trash2 } from 'lucide-react';
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/tooltip';
import { useTranslation } from '@/i18n';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useRef } from 'react';
import { useTts } from './hooks/use-tts';

export const History = () => {
    const { history, deleteItem } = useHistoryState();
    const { t } = useTranslation();
    const { speak } = useTts();

    const historyRef = useRef(history);
    const speakRef = useRef(speak);

    useEffect(() => {
        historyRef.current = history;
        speakRef.current = speak;
    }, [history, speak]);

    useEffect(() => {
        console.log('Registering speak shortcut listener (STABLE)');
        const unlistenPromise = listen<string>('speak-shortcut-triggered', (event) => {
            const globalSelection = event.payload;
            const localSelection = window.getSelection()?.toString().trim();
            
            if (globalSelection) {
                speakRef.current(globalSelection);
            } else if (localSelection) {
                speakRef.current(localSelection);
            } else if (historyRef.current.length > 0) {
                speakRef.current(historyRef.current[0].text);
            }
        });

        return () => {
            unlistenPromise.then((unlisten) => unlisten());
        };
    }, []);

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
        <div className="space-y-2 w-full">
            <div className="flex items-center justify-between">
                <Typography.Title className="flex items-center gap-2">
                    {t('Recent activity')}{' '}
                    <Tooltip>
                        <TooltipTrigger asChild>
                            <InfoIcon className="size-4 inline-block text-muted-foreground cursor-pointer" />
                        </TooltipTrigger>
                        <TooltipContent>
                            <Typography.Paragraph className="text-foreground text-xs">
                                {t(
                                    'All audio is deleted. No telemetry, no tracking. Only the last five text transcriptions are stored on your computer.'
                                )}
                            </Typography.Paragraph>
                        </TooltipContent>
                    </Tooltip>
                </Typography.Title>
                <Dialog>
                    <DialogTrigger asChild>
                        <Trash2 className="size-4 cursor-pointer hover:text-foreground text-muted-foreground transition-colors" />
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
                                    className="bg-card border border-border hover:bg-accent hover:text-foreground"
                                >
                                    {t('Cancel')}
                                </Button>
                            </DialogClose>
                            <DialogClose asChild>
                                <Button variant="destructive" onClick={handleClearHistory}>
                                    {t('Clear')}
                                </Button>
                            </DialogClose>
                        </DialogFooter>
                    </DialogContent>
                </Dialog>
            </div>
            {history.length === 0 ? (
                <Typography.Paragraph>{t('No transcriptions yet')}</Typography.Paragraph>
            ) : (
                <div className="space-y-2">
                    {history.map((entry: HistoryEntry) => (
                        <HistoryItem key={entry.id} entry={entry} onDelete={deleteItem} />
                    ))}
                </div>
            )}
        </div>
    );
};
