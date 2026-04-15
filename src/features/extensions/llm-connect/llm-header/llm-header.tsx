import { useTranslation } from '@/i18n';
import { Typography } from '@/components/typography';
import { Page } from '@/components/page';
import { RenderKeys } from '@/components/render-keys';
import { Lightbulb } from 'lucide-react';
import { useShortcut, SHORTCUT_CONFIGS } from '@/features/settings/shortcuts/hooks/use-shortcut';
export const LLMHeader = () => {
    const { t } = useTranslation();
    const { shortcut: llmShortcut } = useShortcut(SHORTCUT_CONFIGS.llm);
    const { shortcut: commandShortcut } = useShortcut(SHORTCUT_CONFIGS.command);

    return (
        <Page.Header>
            <Typography.MainTitle>{t('LLM Connect')}</Typography.MainTitle>
            <div className="bg-emerald-500/10 border border-emerald-500/30 rounded-lg p-4 text-left space-y-3">
                <div className="flex items-center gap-2">
                    <Lightbulb className="w-4 h-4 text-emerald-400 shrink-0" />
                    <span className="text-emerald-300 font-semibold text-sm">{t('How to use')}</span>
                </div>
                <p className="text-sm text-foreground leading-relaxed">
                    <RenderKeys keyString={llmShortcut} className="mr-1" />
                    {t(' Record and process your voice with the active prompt.')}
                </p>
                <p className="text-sm text-foreground leading-relaxed">
                    <RenderKeys keyString={commandShortcut} className="mr-1" />
                    {t(' Select text, then run a command on it (translate, rephrase...).')}
                </p>
            </div>
        </Page.Header>
    );
};
