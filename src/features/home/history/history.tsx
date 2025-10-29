import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Typography } from '@/components/typography';
import { Button } from '@/components/button';
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
} from '@/components/dialog';
import { toast } from 'sonner';
import { formatTime } from './history.helpers';
import { useHistoryState } from './hooks/use-history-state';

interface HistoryProps {}

export const History = ({}: HistoryProps) => {
    const { history } = useHistoryState();
    const [showClearDialog, setShowClearDialog] = useState(false);
    const [isClearing, setIsClearing] = useState(false);

    const handleClearHistory = async () => {
        setIsClearing(true);
        try {
            await invoke('clear_history');
            toast.success('History cleared', {
                duration: 1500,
                closeButton: true,
            });
            setShowClearDialog(false);
        } catch (error) {
            toast.error('Failed to clear history', {
                duration: 2000,
                closeButton: true,
            });
            console.error('Clear history error:', error);
        } finally {
            setIsClearing(false);
        }
    };

    return (
        <div className="space-y-2 w-full">
            <div className="flex items-center justify-between">
                <Typography.Title>
                    Recent activity{' '}
                    <span className="text-[10px] text-zinc-400">
                        (Only the last 5 transcriptions are kept; older text and
                        audio files are deleted)
                    </span>
                </Typography.Title>
                {history.length > 0 && (
                    <Button
                        variant="destructive"
                        size="sm"
                        onClick={() => setShowClearDialog(true)}
                        disabled={isClearing}
                    >
                        Clear
                    </Button>
                )}
            </div>
            {history.length === 0 ? (
                <Typography.Paragraph>
                    No transcriptions yet
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
                                    await navigator.clipboard.writeText(entry.text);
                                    toast.success('Copied to clipboard', {
                                        duration: 1500,
                                        closeButton: true,
                                    });
                                } catch {
                                    toast.error('Failed to copy', {
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
                                            (Empty transcription)
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

            <Dialog open={showClearDialog} onOpenChange={setShowClearDialog}>
                <DialogContent>
                    <DialogHeader>
                        <DialogTitle>Clear History</DialogTitle>
                        <DialogDescription>
                            Are you sure you want to clear all transcription history? This action cannot be undone.
                        </DialogDescription>
                    </DialogHeader>
                    <DialogFooter>
                        <Button
                            variant="outline"
                            onClick={() => setShowClearDialog(false)}
                            disabled={isClearing}
                        >
                            Cancel
                        </Button>
                        <Button
                            variant="destructive"
                            onClick={handleClearHistory}
                            disabled={isClearing}
                        >
                            {isClearing ? 'Clearing...' : 'Clear'}
                        </Button>
                    </DialogFooter>
                </DialogContent>
            </Dialog>
        </div>
    );
};
