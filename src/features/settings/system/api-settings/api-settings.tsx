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
    const { t } = useTranslation();

    return (
        <>
            <SettingsUI.Item>
                <SettingsUI.Description>
                    <Typography.Title className="flex items-center gap-2">
                        <Zap className="w-4 h-4 text-zinc-400" />
                        {t('Local API')}
                        <code className="text-amber-300 text-[10px]">
                            {t('Experimental')}
                        </code>
                    </Typography.Title>
                    <Typography.Paragraph className="space-y-2">
                        <div>
                            {t('Allows external apps to transcribe .wav files via HTTP.')}
                        </div>
                        <code className="text-xs block border p-2">
                            curl -X POST http://localhost:{apiPort}
                            /api/transcribe -F "audio=@audio.wav;type=audio/wav"
                        </code>
                        <div className="text-xs flex items-center gap-1">
                            <FileCode2 className="w-4 h-4 text-zinc-400 inline-block" />
                            {t('View')}{' '}
                            <ExternalLink href="https://github.com/Kieirra/murmure/blob/main/docs/API_USAGE.md">
                                {t('API documentation')}
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
                            <Typography.Title>{t('API Port')}</Typography.Title>
                            <Typography.Paragraph>
                                {t('Set the port number for the HTTP API (1024-65535)')}
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
