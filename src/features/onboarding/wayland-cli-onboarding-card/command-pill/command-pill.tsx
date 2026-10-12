import { useEffect, useRef, useState } from 'react';
import { Check, Copy } from 'lucide-react';
import { Button } from '@/components/button';
import { useTranslation } from '@/i18n';
import {
    copyCommandToClipboard,
    COPIED_FEEDBACK_DURATION_MS,
} from '@/features/settings/shortcuts/shortcuts-cli/cli-commands-panel/cli-command-row/cli-command-row.helpers';

interface CommandPillProps {
    command: string;
}

export const CommandPill = ({ command }: CommandPillProps) => {
    const { t } = useTranslation();
    const [copied, setCopied] = useState(false);
    const timeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);

    useEffect(() => {
        return () => {
            if (timeoutRef.current !== null) {
                clearTimeout(timeoutRef.current);
            }
        };
    }, []);

    const handleCopy = async () => {
        try {
            await copyCommandToClipboard(command);
            setCopied(true);
            if (timeoutRef.current !== null) {
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
        <span className="inline-flex items-center gap-1 rounded-md border border-input bg-field pl-2">
            <code className="font-mono text-xs text-foreground">{command}</code>
            <Button
                type="button"
                variant="subtle"
                size="icon-sm"
                onClick={handleCopy}
                aria-label={t('Copy command to clipboard')}
                aria-live="polite"
                className="size-6"
                data-testid={`cli-copy-${command}`}
            >
                {copied ? <Check className="size-3.5 text-emerald-400" /> : <Copy className="size-3.5" />}
            </Button>
        </span>
    );
};
