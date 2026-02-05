import { useTranslation } from '@/i18n';
import { Typography } from '@/components/typography';
import { Page } from '@/components/page';
import clsx from 'clsx';
import { ConnectionStatus } from '../hooks/use-llm-connect';
import {
    getStatusIcon,
    getStatusText,
    getStatusColorStyles,
} from '../llm-connect.helpers';
import { RenderKeys } from '@/components/render-keys';
import {
    useShortcut,
    SHORTCUT_CONFIGS,
} from '@/features/settings/shortcuts/hooks/use-shortcut';

interface LLMHeaderProps {
    connectionStatus: ConnectionStatus;
}

export const LLMHeader = ({ connectionStatus }: LLMHeaderProps) => {
    const { t } = useTranslation();
    const { shortcut: llmShortcut } = useShortcut(SHORTCUT_CONFIGS.llm);
    const { shortcut: commandShortcut } = useShortcut(SHORTCUT_CONFIGS.command);

    return (
        <Page.Header>
            <div className="flex justify-between items-center w-full">
                <div className="flex flex-col gap-2">
                    <Typography.MainTitle className="flex items-center gap-2">
                        {t('LLM Connect')}
                    </Typography.MainTitle>
                    <Typography.Paragraph className="text-zinc-400">
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

                {/* Connection Status Top Right */}
                <div
                    className={clsx(
                        'absolute',
                        'top-8',
                        'right-8',
                        'flex',
                        'items-center',
                        'gap-2',
                        'px-3',
                        'py-1.5',
                        'rounded-full',
                        'text-xs',
                        'font-medium',
                        'border',
                        'transition-colors',
                        getStatusColorStyles(connectionStatus)
                    )}
                >
                    {getStatusIcon(connectionStatus)}
                    {getStatusText(connectionStatus, t)}
                </div>
            </div>
        </Page.Header>
    );
};
