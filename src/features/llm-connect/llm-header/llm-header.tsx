import { useTranslation } from '@/i18n';
import { Typography } from '@/components/typography';
import { Page } from '@/components/page';
import { RenderKeys } from '@/components/render-keys';
import { Cloud } from 'lucide-react';
import {
    useShortcut,
    SHORTCUT_CONFIGS,
} from '@/features/settings/shortcuts/hooks/use-shortcut';
import { LLMProvider } from '../hooks/use-llm-connect';

interface LLMHeaderProps {
    activeProvider: LLMProvider;
}

export const LLMHeader = ({ activeProvider }: LLMHeaderProps) => {
    const { t } = useTranslation();
    const { shortcut: llmShortcut } = useShortcut(SHORTCUT_CONFIGS.llm);
    const { shortcut: commandShortcut } = useShortcut(SHORTCUT_CONFIGS.command);

    return (
        <Page.Header>
            <div className="flex flex-col">
                <Typography.MainTitle className="flex items-center">
                    {t('LLM Connect')}
                    {activeProvider === 'remote' && (
                        <Cloud className="w-4 h-4 ml-2 text-sky-400" />
                    )}
                </Typography.MainTitle>
                <Typography.Paragraph className="text-zinc-400 mb-2">
                    {t('Configure your LLM prompts and use the shortcut')}{' '}
                    <RenderKeys keyString={llmShortcut} className="mr-1" />
                    {t(
                        'to record your voice. Your transcription will be automatically processed by the LLM.'
                    )}
                </Typography.Paragraph>
                <Typography.Paragraph className="text-zinc-400">
                    {t('Or you can select text and use the shortcut')}{' '}
                    <RenderKeys
                        keyString={commandShortcut}
                        className="mr-1"
                    />
                    {t(
                        'to run a command on a selected text (eg. translate it to French).'
                    )}
                </Typography.Paragraph>
            </div>
        </Page.Header>
    );
};
