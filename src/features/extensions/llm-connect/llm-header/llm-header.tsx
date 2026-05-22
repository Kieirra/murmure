import { useState, type ReactNode } from 'react';
import { ChevronDown, ChevronUp, Lightbulb, Mic, Zap } from 'lucide-react';
import clsx from 'clsx';
import { useTranslation } from '@/i18n';
import { Typography } from '@/components/typography';
import { Page } from '@/components/page';
import { RenderKeys } from '@/components/render-keys';
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '@/components/collapsible';
import { useShortcut, SHORTCUT_CONFIGS } from '@/features/settings/shortcuts/hooks/use-shortcut';

export const LLMHeader = () => {
    const { t } = useTranslation();
    const { shortcut: llmShortcut } = useShortcut(SHORTCUT_CONFIGS.llm);
    const { shortcut: commandShortcut } = useShortcut(SHORTCUT_CONFIGS.command);
    const { shortcut: llmMode1Shortcut } = useShortcut(SHORTCUT_CONFIGS.llmMode1);
    const { shortcut: llmMode2Shortcut } = useShortcut(SHORTCUT_CONFIGS.llmMode2);
    const { shortcut: llmMode3Shortcut } = useShortcut(SHORTCUT_CONFIGS.llmMode3);
    const { shortcut: llmMode4Shortcut } = useShortcut(SHORTCUT_CONFIGS.llmMode4);
    const [isOpen, setIsOpen] = useState(false);

    const promptSteps: ReactNode[] = [
        <div className="space-y-2">
            <div>{t('Pick a prompt')}</div>
            <div className="text-sm text-muted-foreground">{t('Click a tab below, or press:')}</div>
            <div className="space-y-1 pl-3 text-sm">
                <div>
                    <RenderKeys keyString={llmMode1Shortcut} />
                </div>
                <div>
                    <RenderKeys keyString={llmMode2Shortcut} />
                </div>
                <div>
                    <RenderKeys keyString={llmMode3Shortcut} />
                </div>
                <div>
                    <RenderKeys keyString={llmMode4Shortcut} />
                </div>
            </div>
        </div>,
        <>
            {t('Press ')}
            <RenderKeys keyString={llmShortcut} />
            {t(' and speak')}
        </>,
        t('Your text is rewritten and pasted'),
    ];

    const commandSteps: ReactNode[] = [
        t('Select some text'),
        <>
            {t('Press ')}
            <RenderKeys keyString={commandShortcut} />
        </>,
        <div className="space-y-1">
            <div>{t('Speak the command')}</div>
            <div className="text-sm text-muted-foreground italic">{t('e.g. "Translate to English"')}</div>
        </div>,
        t('Your text is replaced'),
    ];

    return (
        <Page.Header>
            <Typography.MainTitle>{t('LLM Connect')}</Typography.MainTitle>
            <Collapsible open={isOpen} onOpenChange={setIsOpen}>
                <CollapsibleTrigger asChild>
                    <button
                        type="button"
                        className={clsx(
                            'flex w-full items-center gap-2 rounded-md',
                            'bg-emerald-500/10 border border-emerald-500/30 px-3 py-2.5',
                            'text-sm text-left',
                            'hover:bg-emerald-500/15 transition-colors'
                        )}
                        data-testid="llm-connect-header-trigger"
                    >
                        <Lightbulb className="w-4 h-4 shrink-0 text-emerald-400" />
                        <span className="flex-1 text-foreground">{t('How does LLM Connect work?')}</span>
                        {isOpen ? (
                            <ChevronUp className="w-4 h-4 text-muted-foreground" />
                        ) : (
                            <ChevronDown className="w-4 h-4 text-muted-foreground" />
                        )}
                    </button>
                </CollapsibleTrigger>
                <CollapsibleContent>
                    <div className="mt-2 rounded-md bg-emerald-500/5 border border-emerald-500/20 p-4">
                        <div className="flex flex-col md:flex-row gap-4">
                            <WorkflowCard
                                icon={Mic}
                                title={t('Prompt')}
                                benefit={t('Speak, and the AI rewrites what you said.')}
                                steps={promptSteps}
                            />
                            <WorkflowCard
                                icon={Zap}
                                title={t('Command')}
                                benefit={t('Transform existing text with a voice instruction.')}
                                steps={commandSteps}
                            />
                        </div>
                    </div>
                </CollapsibleContent>
            </Collapsible>
        </Page.Header>
    );
};

interface WorkflowCardProps {
    icon: typeof Mic;
    title: string;
    benefit: string;
    steps: ReactNode[];
}

const WorkflowCard = ({ icon: Icon, title, benefit, steps }: WorkflowCardProps) => (
    <div className="flex-1 rounded-md border border-border bg-background/50 p-4 space-y-3">
        <div className="flex items-center gap-2">
            <Icon className="w-4 h-4 shrink-0 text-emerald-400" />
            <span className="text-base font-semibold text-foreground">{title}</span>
        </div>
        <p className="text-sm text-muted-foreground">{benefit}</p>
        <ol className="list-decimal list-outside text-sm text-foreground space-y-3 pl-5 marker:text-emerald-400 marker:font-semibold">
            {steps.map((step, i) => (
                <li key={i} className="pl-1">
                    {step}
                </li>
            ))}
        </ol>
    </div>
);
