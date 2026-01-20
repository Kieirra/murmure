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
