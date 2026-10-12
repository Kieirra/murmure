import { Check, Copy } from 'lucide-react';
import { Button } from '@/components/button';
import { useCopyFeedback } from '@/components/hooks/use-copy-feedback';
import { useTranslation } from '@/i18n';

interface CommandPillProps {
    command: string;
}

export const CommandPill = ({ command }: CommandPillProps) => {
    const { t } = useTranslation();
    const { copied, handleCopy } = useCopyFeedback(command);

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
