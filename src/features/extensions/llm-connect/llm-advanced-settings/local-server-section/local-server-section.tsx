import { useTranslation } from '@/i18n';
import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { Input } from '@/components/input';
import { Page } from '@/components/page';
import { Server } from 'lucide-react';
import { ConnectionStatus } from '../../hooks/use-llm-connect';
import { ConnectionButton } from '../connection-button/connection-button';

interface LocalServerSectionProps {
    localUrl: string;
    onLocalUrlChange: (value: string) => void;
    onLocalUrlBlur: () => void;
    isTestingLocal: boolean;
    localConnectionStatus: ConnectionStatus;
    onTestLocal: () => void;
    showInstallModel: boolean;
    onInstallModel: () => void;
}

export const LocalServerSection = ({
    localUrl,
    onLocalUrlChange,
    onLocalUrlBlur,
    isTestingLocal,
    localConnectionStatus,
    onTestLocal,
    showInstallModel,
    onInstallModel,
}: LocalServerSectionProps) => {
    const { t } = useTranslation();

    return (
        <SettingsUI.Section title={t('Ollama')} icon={Server}>
            <SettingsUI.Item>
                <SettingsUI.Description>
                    <Typography.Title>{t('Server URL')}</Typography.Title>
                </SettingsUI.Description>
                <div className="flex items-center gap-3">
                    <Input
                        value={localUrl}
                        onChange={(e) => onLocalUrlChange(e.target.value)}
                        onBlur={onLocalUrlBlur}
                        className="w-70"
                        placeholder="http://localhost:11434/api"
                    />
                    <ConnectionButton isTesting={isTestingLocal} status={localConnectionStatus} onClick={onTestLocal} />
                </div>
            </SettingsUI.Item>
            {showInstallModel && (
                <>
                    <SettingsUI.Separator />
                    <SettingsUI.Item>
                        <SettingsUI.Description>
                            <Typography.Title>{t('Models')}</Typography.Title>
                        </SettingsUI.Description>
                        <Page.SecondaryButton onClick={onInstallModel} size="sm">
                            {t('Install another model')}
                        </Page.SecondaryButton>
                    </SettingsUI.Item>
                </>
            )}
        </SettingsUI.Section>
    );
};
