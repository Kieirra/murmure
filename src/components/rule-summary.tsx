import { useTranslation } from '@/i18n';

interface RuleSummaryProps {
    trigger: string;
    replacement: string;
}

const formatReplacement = (replacement: string): string => {
    if (replacement.length === 0) {
        return '';
    }
    const normalized = replacement.replaceAll('\n', '\u21B5');
    if (normalized.length > 20) {
        return `${normalized.substring(0, 20)}...`;
    }
    return normalized;
};

export const RuleSummary = ({ trigger, replacement }: RuleSummaryProps) => {
    const { t } = useTranslation();

    const formattedReplacement = formatReplacement(replacement);

    return (
        <span className="text-sm truncate">
            <span className="font-medium text-foreground">{trigger || t('(empty trigger)')}</span>
            <span className="text-muted-foreground">{' \u2192 '}</span>
            <span className="text-muted-foreground">{formattedReplacement || t('(delete)')}</span>
        </span>
    );
};
