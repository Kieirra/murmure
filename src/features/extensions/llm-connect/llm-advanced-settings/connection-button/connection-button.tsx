import { useTranslation } from '@/i18n';
import { Page } from '@/components/page';
import { RefreshCw, CheckCircle2 } from 'lucide-react';
import { ConnectionStatus } from '../../hooks/use-llm-connect';
import clsx from 'clsx';

interface ConnectionButtonProps {
    isTesting: boolean;
    status: ConnectionStatus;
    onClick: () => void;
    disabled?: boolean;
    connectedLabel?: string;
}

export const ConnectionButton = ({ isTesting, status, onClick, disabled, connectedLabel }: ConnectionButtonProps) => {
    const { t } = useTranslation();

    const renderContent = () => {
        if (isTesting) {
            return (
                <>
                    <RefreshCw className="w-4 h-4 animate-spin mr-2" />
                    {t('Testing...')}
                </>
            );
        }
        if (status === 'connected') {
            return (
                <>
                    <CheckCircle2 className="w-4 h-4 mr-2" />
                    {connectedLabel ?? t('Connected')}
                </>
            );
        }
        return t('Test Connection');
    };

    return (
        <Page.SecondaryButton
            onClick={onClick}
            size="sm"
            disabled={disabled || isTesting}
            className={clsx(
                'whitespace-nowrap',
                status === 'connected' && 'text-emerald-500 hover:bg-emerald-400/10 hover:text-emerald-300'
            )}
        >
            {renderContent()}
        </Page.SecondaryButton>
    );
};
