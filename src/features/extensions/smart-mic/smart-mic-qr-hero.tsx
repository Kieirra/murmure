import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { Page } from '@/components/page';
import { Lightbulb, QrCode, RefreshCw } from 'lucide-react';
import { useTranslation } from '@/i18n';

interface SmartMicQrHeroProps {
    qrCodeDataUri: string;
    resetTokens: () => void;
}

export const SmartMicQrHero = ({ qrCodeDataUri, resetTokens }: SmartMicQrHeroProps) => {
    const { t } = useTranslation();

    if (qrCodeDataUri.length === 0) {
        return null;
    }

    return (
        <SettingsUI.Section title={t('Connect a phone')} icon={QrCode}>
            <div className="flex flex-col items-center gap-3 p-6">
                <img
                    src={qrCodeDataUri}
                    alt="Smart Mic QR Code"
                    className="w-[200px] h-[200px] rounded-lg border border-border"
                />
                <Typography.Paragraph className="text-center">
                    {t('Scan this QR code with your smartphone to connect')}
                </Typography.Paragraph>
                <Page.SecondaryButton
                    size="sm"
                    onClick={resetTokens}
                    title={t('Reset QR code and revoke all paired devices')}
                >
                    <RefreshCw className="w-3 h-3" />
                    {t('Reset QR Code')}
                </Page.SecondaryButton>
                <div className="mt-2 flex items-start gap-2 rounded-md bg-muted/40 px-2.5 py-2 text-xs text-muted-foreground max-w-md">
                    <Lightbulb className="w-4 h-4 mt-0.5 shrink-0 text-cyan-400" />
                    <span>
                        {t(
                            'After scanning, use "Add to Home Screen" in your browser to install it as an app, no need to rescan.'
                        )}
                    </span>
                </div>
            </div>
        </SettingsUI.Section>
    );
};
