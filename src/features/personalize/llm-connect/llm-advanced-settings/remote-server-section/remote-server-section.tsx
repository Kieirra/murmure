import { useTranslation } from '@/i18n';
import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { Input } from '@/components/input';
import { Cloud, Eye, EyeOff, AlertTriangle, AlertCircle } from 'lucide-react';
import { ConnectionStatus } from '../../hooks/use-llm-connect';
import { DEFAULT_REMOTE_URL_PLACEHOLDER } from '../../llm-connect.constants';
import { isInsecureRemoteUrl } from '../../llm-connect.helpers';
import { ConnectionButton } from '../connection-button/connection-button';

interface RemoteServerSectionProps {
    localRemoteUrl: string;
    onRemoteUrlChange: (value: string) => void;
    onRemoteUrlBlur: () => void;
    isTesting: boolean;
    remoteConnectionStatus: ConnectionStatus;
    onTestRemote: () => void;
    remoteModelCount: number | null;
    remoteError: string | null;
    apiKeyValue: string;
    onApiKeyChange: (value: string) => void;
    onApiKeyBlur: () => void;
    showApiKey: boolean;
    onToggleShowApiKey: () => void;
}

export const RemoteServerSection = ({
    localRemoteUrl,
    onRemoteUrlChange,
    onRemoteUrlBlur,
    isTesting,
    remoteConnectionStatus,
    onTestRemote,
    remoteModelCount,
    remoteError,
    apiKeyValue,
    onApiKeyChange,
    onApiKeyBlur,
    showApiKey,
    onToggleShowApiKey,
}: RemoteServerSectionProps) => {
    const { t } = useTranslation();

    const connectedLabel =
        remoteModelCount === null ? undefined : t('Connected — {{count}} models', { count: remoteModelCount });

    return (
        <section>
            <Typography.Title className="p-2 font-semibold text-sky-400! flex items-center gap-2">
                <Cloud className="w-4 h-4" />
                {t('Remote Server (OpenAI-compatible)')}
            </Typography.Title>
            <SettingsUI.Container>
                <SettingsUI.Item>
                    <SettingsUI.Description>
                        <Typography.Title>{t('Server URL')}</Typography.Title>
                    </SettingsUI.Description>
                    <div className="flex items-center gap-3">
                        <Input
                            value={localRemoteUrl}
                            onChange={(e) => onRemoteUrlChange(e.target.value)}
                            onBlur={onRemoteUrlBlur}
                            className="w-70"
                            placeholder={DEFAULT_REMOTE_URL_PLACEHOLDER}
                        />
                        <ConnectionButton
                            isTesting={isTesting}
                            status={remoteConnectionStatus}
                            onClick={onTestRemote}
                            disabled={localRemoteUrl.length === 0}
                            connectedLabel={connectedLabel}
                        />
                    </div>
                </SettingsUI.Item>
                <SettingsUI.Separator />
                <SettingsUI.Item>
                    <SettingsUI.Description>
                        <Typography.Title>{t('API Key')}</Typography.Title>
                        <Typography.Paragraph>
                            {t("Leave empty if your server doesn't require authentication.")}
                        </Typography.Paragraph>
                    </SettingsUI.Description>
                    <div className="relative w-70">
                        <Input
                            type={showApiKey ? 'text' : 'password'}
                            value={apiKeyValue}
                            onChange={(e) => onApiKeyChange(e.target.value)}
                            onBlur={onApiKeyBlur}
                            placeholder="sk-..."
                            className="w-full pr-10"
                        />
                        <button
                            type="button"
                            onClick={onToggleShowApiKey}
                            className="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
                        >
                            {showApiKey ? <EyeOff className="w-4 h-4" /> : <Eye className="w-4 h-4" />}
                        </button>
                    </div>
                </SettingsUI.Item>

                {(remoteError || localRemoteUrl.length > 0 || isInsecureRemoteUrl(localRemoteUrl)) && (
                    <div className="px-4 pb-3 flex flex-col gap-1">
                        {remoteError && (
                            <div className="flex items-center gap-1.5 text-xs text-red-400">
                                <AlertCircle className="w-3 h-3 shrink-0" />
                                {remoteError}
                            </div>
                        )}
                        {localRemoteUrl.length > 0 && (
                            <div className="flex items-center gap-1.5 text-xs text-amber-500">
                                <AlertTriangle className="w-3 h-3 shrink-0" />
                                {t(
                                    'Your transcriptions are sent to a third-party server and are not protected by local privacy.'
                                )}
                            </div>
                        )}
                        {isInsecureRemoteUrl(localRemoteUrl) && (
                            <div className="flex items-center gap-1.5 text-xs text-amber-500">
                                <AlertTriangle className="w-3 h-3 shrink-0" />
                                {t('This connection is not encrypted. Your data could be intercepted.')}
                            </div>
                        )}
                    </div>
                )}
            </SettingsUI.Container>
        </section>
    );
};
