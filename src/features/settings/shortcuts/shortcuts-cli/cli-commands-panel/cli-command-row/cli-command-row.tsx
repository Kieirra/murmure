import { useState, useEffect, useRef } from 'react';
import { Check, Copy } from 'lucide-react';
import { Button } from '@/components/button';
import { Typography } from '@/components/typography';
import { useTranslation } from '@/i18n';
import { copyCommandToClipboard, COPIED_FEEDBACK_DURATION_MS } from './cli-command-row.helpers';

interface CliCommandRowProps {
    label: string;
    command: string;
}

export const CliCommandRow = ({ label, command }: CliCommandRowProps) => {
    const { t } = useTranslation();
    const [copied, setCopied] = useState(false);
    const timeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);

    useEffect(() => {
        return () => {
            if (timeoutRef.current) {
                clearTimeout(timeoutRef.current);
            }
        };
    }, []);

    const handleCopy = async () => {
        try {
            await copyCommandToClipboard(command);
            setCopied(true);
            if (timeoutRef.current) {
                clearTimeout(timeoutRef.current);
            }
            timeoutRef.current = setTimeout(() => {
                setCopied(false);
                timeoutRef.current = null;
            }, COPIED_FEEDBACK_DURATION_MS);
        } catch (err) {
            console.error('Failed to copy CLI command:', err);
        }
    };

    return (
        <div className="flex items-center justify-between gap-4 px-4 py-3">
            <div className="flex flex-col gap-1 min-w-0">
                <Typography.Title className="text-sm">{t(label)}</Typography.Title>
                <code className="font-mono text-xs text-muted-foreground truncate">{command}</code>
            </div>
            <Button
                variant="ghost"
                size="sm"
                onClick={handleCopy}
                aria-label={t('Copy command to clipboard')}
                aria-live="polite"
                className="shrink-0"
                data-testid={`cli-copy-${command}`}
            >
                {copied ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
                <span className="ml-1">{copied ? t('Copied') : t('Copy')}</span>
            </Button>
        </div>
    );
};
