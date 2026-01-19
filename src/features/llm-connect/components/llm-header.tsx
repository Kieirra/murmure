import { useTranslation } from '@/i18n';
import { Typography } from '@/components/typography';
import { Page } from '@/components/page';
import clsx from 'clsx';
import { ConnectionStatus } from '../hooks/use-llm-connect';
import { getStatusIcon, getStatusText } from '../llm-connect.helpers';

interface LLMHeaderProps {
    connectionStatus: ConnectionStatus;
}

export const LLMHeader = ({ connectionStatus }: LLMHeaderProps) => {
    const { t } = useTranslation();

    return (
        <Page.Header>
            <div className="flex justify-between items-center w-full">
                <div className="flex flex-col gap-2">
                    <Typography.MainTitle className="flex items-center gap-2">
                        {t('LLM Connect')}
                    </Typography.MainTitle>
                    <Typography.Paragraph className="text-zinc-400">
                        {t('Configure your LLM prompts.')}
                    </Typography.Paragraph>
                </div>

                {/* Connection Status Top Right */}
                <div
                    className={clsx(
                        'flex items-center gap-2 px-3 py-1.5 rounded-full text-xs font-medium border transiton-colors',
                        connectionStatus === 'connected'
                            ? 'bg-emerald-500/10 text-emerald-500 border-emerald-500/20'
                            : connectionStatus === 'error'
                              ? 'bg-red-500/10 text-red-500 border-red-500/20'
                              : 'bg-zinc-800 text-zinc-400 border-zinc-700'
                    )}
                >
                    {getStatusIcon(connectionStatus)}
                    {getStatusText(connectionStatus, t)}
                </div>
            </div>
        </Page.Header>
    );
};
