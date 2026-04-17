import { useTranslation } from '@/i18n';
import { Typography } from '@/components/typography';
import { Page } from '@/components/page';
import { RenderKeys } from '@/components/render-keys';
import { Mic, Wand2 } from 'lucide-react';
import { useShortcut, SHORTCUT_CONFIGS } from '@/features/settings/shortcuts/hooks/use-shortcut';

export const LLMHeader = () => {
    const { t } = useTranslation();
    const { shortcut: llmShortcut } = useShortcut(SHORTCUT_CONFIGS.llm);
    const { shortcut: commandShortcut } = useShortcut(SHORTCUT_CONFIGS.command);

    return (
        <Page.Header>
            <Typography.MainTitle>{t('LLM Connect')}</Typography.MainTitle>
            <div className="bg-emerald-500/10 border border-emerald-500/30 rounded-lg p-4 text-left space-y-3">
                <div className="space-y-1">
                    <div className="flex items-center gap-2">
                        <Mic className="w-4 h-4 text-emerald-400 shrink-0" />
                        <span className="text-sm font-semibold text-foreground">{t('Prompt')}</span>
                    </div>
                    <p className="text-sm text-foreground leading-relaxed pl-6">
                        <RenderKeys keyString={llmShortcut} className="mr-1" />
                        {t('Record and process your voice with the active prompt.')}
                    </p>
                </div>
                <div className="space-y-1">
                    <div className="flex items-center gap-2">
                        <Wand2 className="w-4 h-4 text-emerald-400 shrink-0" />
                        <span className="text-sm font-semibold text-foreground">{t('Command')}</span>
                    </div>
                    <p className="text-sm text-foreground leading-relaxed pl-6">
                        {t('Select text, then press')}
                        <RenderKeys keyString={commandShortcut} className="mx-1" />
                        {t('to run a command on it.')}
                    </p>
                    <p className="text-xs text-muted-foreground pl-6 italic">
                        {t('Try: "Translate to English" or "Make it more formal"')}
                    </p>
                </div>
            </div>
        </Page.Header>
    );
};
