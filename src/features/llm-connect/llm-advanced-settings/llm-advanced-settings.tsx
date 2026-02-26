import { useTranslation } from '@/i18n';
import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { Input } from '@/components/input';
import { Page } from '@/components/page';

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
                    <Page.SecondaryButton onClick={onTestConnection} size="sm">
                        {t('Test Connection')}
                    </Page.SecondaryButton>
                </div>
            </SettingsUI.Item>

            <SettingsUI.Separator />

            <SettingsUI.Item>
                <SettingsUI.Description>
                    <Typography.Title>{t('Tutorial')}</Typography.Title>
                </SettingsUI.Description>

                <div className="flex items-center gap-3">
                    <Page.SecondaryButton onClick={onInstallModel} size="sm">
                        {t('Install another model')}
                    </Page.SecondaryButton>
                    <Page.SecondaryButton
                        onClick={onResetOnboarding}
                        variant="ghost"
                        size="sm"
                        className="text-zinc-500 hover:text-zinc-300"
                    >
                        {t('Reset Tutorial')}
                    </Page.SecondaryButton>
                </div>
            </SettingsUI.Item>
        </SettingsUI.Container>
    );
};
