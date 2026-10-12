import { Check, Copy } from 'lucide-react';
import { Button } from '@/components/button';
import { Typography } from '@/components/typography';
import { useCopyFeedback } from '@/components/hooks/use-copy-feedback';
import { useTranslation } from '@/i18n';

interface CliCommandRowProps {
    label: string;
    command: string;
}

export const CliCommandRow = ({ label, command }: CliCommandRowProps) => {
    const { t } = useTranslation();
    const { copied, handleCopy } = useCopyFeedback(command);

    return (
        <div className="flex items-center justify-between gap-4 px-4 py-3">
            <div className="flex flex-col gap-1 min-w-0">
                <Typography.Title className="text-sm">{t(label)}</Typography.Title>
                <code className="font-mono text-xs text-muted-foreground truncate">{command}</code>
            </div>
            <Button
                variant="subtle"
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
