import { NumberInput } from '@/components/number-input';
import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { useApiState } from './hooks/use-api-state';
import { FileCode2, Zap } from 'lucide-react';
import { Switch } from '@/components/switch';
import { ExternalLink } from '@/components/external-link';
import { useTranslation } from '@/i18n';

export const APISettings = () => {
    const { apiEnabled, setApiEnabled, apiPort, setApiPort } = useApiState();
    const { t } = useTranslation('settings');

    return (
        <>
            <SettingsUI.Item>
                <SettingsUI.Description>
                    <Typography.Title className="flex items-center gap-2">
                        <Zap className="w-4 h-4 text-zinc-400" />
                        {t('system.api.title')}
                        <code className="text-amber-300 text-[10px]">
                            {t('system.api.experimental')}
                        </code>
                    </Typography.Title>
                    <Typography.Paragraph className="space-y-2">
                        <div>
                            {t('system.api.description')}
                        </div>
                        <code className="text-xs block border p-2">
                            curl -X POST http://localhost:{apiPort}
                            /api/transcribe -F "audio=@audio.wav;type=audio/wav"
                        </code>
                        <div className="text-xs flex items-center gap-1">
                            <FileCode2 className="w-4 h-4 text-zinc-400 inline-block" />
                            {t('system.api.viewDocumentation')}{' '}
                            <ExternalLink href="https://github.com/Kieirra/murmure/blob/main/docs/API_USAGE.md">
                                {t('system.api.documentationLink')}
                            </ExternalLink>
                        </div>
                    </Typography.Paragraph>
                </SettingsUI.Description>
                <Switch checked={apiEnabled} onCheckedChange={setApiEnabled} />
            </SettingsUI.Item>
            {apiEnabled && (
                <>
                    <SettingsUI.Separator />
                    <SettingsUI.Item>
                        <SettingsUI.Description>
                            <Typography.Title>{t('system.api.port.title')}</Typography.Title>
                            <Typography.Paragraph>
                                {t('system.api.port.description')}
                            </Typography.Paragraph>
                        </SettingsUI.Description>
                        <NumberInput
                            min={1024}
                            max={65535}
                            value={apiPort}
                            onValueChange={(value) => setApiPort(value ?? 4800)}
                        />
                    </SettingsUI.Item>
                </>
            )}
        </>
    );
};
