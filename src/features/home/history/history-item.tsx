import React from 'react';
import { Typography } from '@/components/typography';
import { Button } from '@/components/button';
import { Volume2, Square, Download, Trash2 } from 'lucide-react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';
import { HistoryEntry } from './hooks/use-history-state';
import { formatTime } from './history.helpers';
import { useTts } from './hooks/use-tts';
import { downloadDir, join } from '@tauri-apps/api/path';

interface HistoryItemProps {
    entry: HistoryEntry;
    onDelete?: (id: number) => void;
}

export const HistoryItem: React.FC<HistoryItemProps> = ({ entry, onDelete }) => {
    const { t } = useTranslation();
    const { speak, stop, isSpeaking, useNeural, exportWav } = useTts();

    const handleCopy = async () => {
        if (!entry.text) return;
        try {
            await navigator.clipboard.writeText(entry.text);
            toast.info(t('Copied to clipboard'), {
                autoClose: 1500,
            });
        } catch {
            toast.error(t('Failed to copy'));
        }
    };

    const handleTts = (e: React.MouseEvent) => {
        e.stopPropagation();
        if (isSpeaking) {
            stop();
            return;
        }

        const selection = window.getSelection()?.toString().trim();
        const textToRead = selection && entry.text.includes(selection) ? selection : entry.text;

        if (textToRead) {
            speak(textToRead);
        }
    };

    const handleDownload = async (e: React.MouseEvent) => {
        e.stopPropagation();
        if (!entry.text) return;

        try {
            const dateStr = new Date().toISOString().replace(/:/g, '-').slice(0, 19);
            const fileName = `murmure-tts-${dateStr}.wav`;
            
            try {
                // Determine a unique path in the user's Downloads folder
                const dlDir = await downloadDir();
                const filePath = await join(dlDir, fileName);

                await exportWav(entry.text, filePath);
                toast.success(t('Audio saved to Downloads'));
            } catch (pathError) {
                console.error('Failed to resolve download path:', pathError);
                toast.error(t('Failed to save audio'));
            }
        } catch (error) {
            console.error('Download failed:', error);
            toast.error(t('Failed to save audio'));
        }
    };

    const handleDelete = (e: React.MouseEvent) => {
        e.stopPropagation();
        if (onDelete) {
            onDelete(entry.id);
        }
    };

    return (
        <div className="relative group">
            <button
                className="w-full text-left rounded-md border border-border p-3 hover:bg-accent cursor-pointer transition-colors"
                onClick={handleCopy}
            >
                <div className="flex items-start justify-between gap-3 pr-16">
                    <Typography.Paragraph className="break-words">
                        {entry.text === '' ? (
                            <span className="italic text-xs text-muted-foreground">{t('(Empty transcription)')}</span>
                        ) : (
                            entry.text
                        )}
                    </Typography.Paragraph>
                    <Typography.Paragraph className="text-[10px] text-muted-foreground shrink-0 whitespace-nowrap mt-1">
                        {formatTime(entry.timestamp)}
                    </Typography.Paragraph>
                </div>
            </button>
            
            <div 
                className={`absolute top-2 right-2 flex items-center gap-1 bg-background/90 backdrop-blur-sm rounded-md p-1 border border-border shadow-sm transition-opacity ${
                    isSpeaking ? 'opacity-100' : 'opacity-0 group-hover:opacity-100 focus-within:opacity-100'
                }`}
            >
                {useNeural && (
                    <Button
                        variant="ghost"
                        size="icon-sm"
                        className="text-muted-foreground hover:text-foreground hover:bg-muted h-7 w-7"
                        onClick={handleDownload}
                        title={t('Download WAV')}
                    >
                        <Download className="w-3.5 h-3.5" />
                    </Button>
                )}
                
                <Button
                    variant="ghost"
                    size="icon-sm"
                    className="text-muted-foreground hover:text-red-500 hover:bg-red-500/10 h-7 w-7 transition-colors"
                    onClick={handleDelete}
                    title={t('Delete item')}
                >
                    <Trash2 className="w-3.5 h-3.5" />
                </Button>
                
                <Button
                    variant="ghost"
                    size="icon-sm"
                    className={`h-7 w-7 transition-all ${
                        isSpeaking 
                        ? 'text-red-500 bg-red-500/10 hover:bg-red-500/20 hover:text-red-600' 
                        : 'text-muted-foreground hover:text-foreground hover:bg-muted'
                    }`}
                    onClick={handleTts}
                    title={isSpeaking ? t('Stop reading') : t('Read aloud')}
                >
                    {isSpeaking ? (
                        <Square className="w-3.5 h-3.5 fill-current" />
                    ) : (
                        <Volume2 className="w-3.5 h-3.5" />
                    )}
                </Button>
            </div>
        </div>
    );
};
