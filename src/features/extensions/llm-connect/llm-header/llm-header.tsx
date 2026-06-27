import { type ReactNode } from 'react';
import { HelpCircle, Mic, Zap } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { Typography } from '@/components/typography';
import { Page } from '@/components/page';
import { RenderKeys } from '@/components/render-keys';
import { Tooltip, TooltipTrigger, TooltipContent } from '@/components/tooltip';
import { useShortcut, SHORTCUT_CONFIGS } from '@/features/settings/shortcuts/hooks/use-shortcut';

export const LLMHeader = () => {
    const { t } = useTranslation();
    const { shortcut: commandShortcut } = useShortcut(SHORTCUT_CONFIGS.command);
    const { shortcut: llmMode1Shortcut } = useShortcut(SHORTCUT_CONFIGS.llmMode1);
    const { shortcut: llmMode2Shortcut } = useShortcut(SHORTCUT_CONFIGS.llmMode2);
    const { shortcut: llmMode3Shortcut } = useShortcut(SHORTCUT_CONFIGS.llmMode3);
    const { shortcut: llmMode4Shortcut } = useShortcut(SHORTCUT_CONFIGS.llmMode4);

    const promptSteps: ReactNode[] = [
        <div className="space-y-2">
            <div>{t('Press one of:')}</div>
            <div className="space-y-1 pl-3 text-sm">
                {[llmMode1Shortcut, llmMode2Shortcut, llmMode3Shortcut, llmMode4Shortcut].map((shortcut, i) => (
                    <div key={i}>
                        <RenderKeys keyString={shortcut} />
                    </div>
                ))}
            </div>
            <div className="text-sm text-muted-foreground italic">{t('Each shortcut runs a different prompt.')}</div>
        </div>,
        t('Speak'),
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
            <div className="flex items-center gap-2">
                <Typography.MainTitle className="mb-0!">{t('LLM Connect')}</Typography.MainTitle>
                <Tooltip>
                    <TooltipTrigger asChild>
                        <button
                            type="button"
                            className="text-muted-foreground hover:text-sky-400 transition-colors cursor-pointer"
                            aria-label={t('How does LLM Connect work?')}
                            data-testid="llm-connect-header-trigger"
                        >
                            <HelpCircle className="w-5 h-5" />
                        </button>
                    </TooltipTrigger>
                    <TooltipContent align="start" className="w-[520px] max-w-[90vw] p-4 text-sm">
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
                    </TooltipContent>
                </Tooltip>
            </div>
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
            <Icon className="w-4 h-4 shrink-0 text-sky-400" />
            <span className="text-base font-semibold text-foreground">{title}</span>
        </div>
        <p className="text-sm text-muted-foreground">{benefit}</p>
        <ol className="list-decimal list-outside text-sm text-foreground space-y-3 pl-5 marker:text-sky-400 marker:font-semibold">
            {steps.map((step, i) => (
                <li key={i} className="pl-1">
                    {step}
                </li>
            ))}
        </ol>
    </div>
);
