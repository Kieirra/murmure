import { useTranslation } from '@/i18n';
import { Tooltip, TooltipTrigger, TooltipContent } from '@/components/tooltip';

interface RuleSummaryProps {
    trigger: string;
    replacement: string;
    name?: string;
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

export const RuleSummary = ({ trigger, replacement, name }: RuleSummaryProps) => {
    const { t } = useTranslation();

    const formattedReplacement = formatReplacement(replacement);

    if (name != null && name.length > 0) {
        return (
            <Tooltip>
                <TooltipTrigger asChild>
                    <span className="text-sm font-medium text-foreground truncate cursor-default">{name}</span>
                </TooltipTrigger>
                <TooltipContent>
                    <span className="font-medium">{trigger || t('(empty trigger)')}</span>
                    <span>{' \u2192 '}</span>
                    <span>{formattedReplacement || t('(delete)')}</span>
                </TooltipContent>
            </Tooltip>
        );
    }

    return (
        <span className="text-sm truncate">
            <span className="font-medium text-foreground">{trigger || t('(empty trigger)')}</span>
            <span className="text-muted-foreground">{' \u2192 '}</span>
            <span className="text-muted-foreground">{formattedReplacement || t('(delete)')}</span>
        </span>
    );
};
