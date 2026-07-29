import { type ReactNode } from 'react';
import { HelpCircle, type LucideIcon } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { RenderKeys } from '@/components/render-keys';
import { Tooltip, TooltipTrigger, TooltipContent } from '@/components/tooltip';
import { WorkflowCard } from '../../../workflow-card/workflow-card';

interface GestureItemProps {
    icon: LucideIcon;
    label: string;
    shortcut: string;
    benefit: string;
    steps: ReactNode[];
}

export const GestureItem = ({ icon: Icon, label, shortcut, benefit, steps }: GestureItemProps) => {
    const { t } = useTranslation();

    return (
        <div className="flex items-center gap-2">
            <Icon className="w-4 h-4 shrink-0 text-sky-400" />
            <span className="text-sm font-medium text-foreground">{label}</span>
            <Tooltip>
                <TooltipTrigger asChild>
                    <button
                        type="button"
                        className="text-muted-foreground hover:text-sky-400 transition-colors cursor-pointer"
                        aria-label={t('How does {{gesture}} work?', { gesture: label })}
                    >
                        <HelpCircle className="w-4 h-4" />
                    </button>
                </TooltipTrigger>
                <TooltipContent align="start" className="w-[320px] max-w-[90vw] border-0 p-0 text-sm">
                    <WorkflowCard icon={Icon} title={label} benefit={benefit} steps={steps} />
                </TooltipContent>
            </Tooltip>
            {shortcut.length > 0 ? (
                <RenderKeys keyString={shortcut} className="gap-0.5 text-[10px]" />
            ) : (
                <span className="text-sm text-muted-foreground">{t('No shortcut set.')}</span>
            )}
        </div>
    );
};
