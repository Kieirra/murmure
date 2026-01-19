import { useTranslation } from '@/i18n';
import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { Button } from '@/components/button';
import { Input } from '@/components/input';

interface LLMAdvancedSettingsProps {
    url: string;
    onUrlChange: (url: string) => void;
    onTestConnection: () => void;
    onInstallModel: () => void;
    onResetOnboarding: () => void;
}

export const LLMAdvancedSettings = ({
    url,
    onUrlChange,
    onTestConnection,
    onInstallModel,
    onResetOnboarding,
}: LLMAdvancedSettingsProps) => {
    const { t } = useTranslation();

    return (
        <SettingsUI.Container className="mb-6">
            <SettingsUI.Item>
                <SettingsUI.Description>
                    <Typography.Title>{t('Ollama API URL')}</Typography.Title>
                </SettingsUI.Description>
                <div className="flex items-center gap-3">
                    <Input
                        value={url}
                        onChange={(e) => onUrlChange(e.target.value)}
                        className="w-[200px]"
                        placeholder="http://localhost:11434/api"
                    />
                    <Button
                        onClick={onTestConnection}
                        variant="outline"
                        size="sm"
                    >
                        {t('Test Connection')}
                    </Button>
                </div>
            </SettingsUI.Item>

            <SettingsUI.Separator />

            <SettingsUI.Item>
                <SettingsUI.Description>
                    <Typography.Title>{t('Tutorial')}</Typography.Title>
                </SettingsUI.Description>

                <div className="flex items-center gap-3">
                    <Button
                        onClick={onInstallModel}
                        variant="outline"
                        size="sm"
                    >
                        {t('Install another model')}
                    </Button>
                    <Button
                        onClick={onResetOnboarding}
                        variant="ghost"
                        size="sm"
                        className="text-zinc-500 hover:text-zinc-300"
                    >
                        {t('Reset Tutorial')}
                    </Button>
                </div>
            </SettingsUI.Item>
        </SettingsUI.Container>
    );
};
