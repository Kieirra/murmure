import { Typography } from '@/components/typography';
import { Lightbulb, RefreshCw } from 'lucide-react';
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
        <div className="flex flex-col items-center gap-3 p-6">
            <img
                src={qrCodeDataUri}
                alt="Smart Mic QR Code"
                className="w-[200px] h-[200px] rounded-lg border border-border"
            />
            <Typography.Paragraph className="text-center">
                {t('Scan this QR code with your smartphone to connect')}
            </Typography.Paragraph>
            <button
                onClick={resetTokens}
                className="flex items-center gap-1.5 px-3 py-1.5 text-xs rounded-md border border-border text-muted-foreground hover:text-foreground hover:bg-muted transition-colors"
                title={t('Reset QR code and revoke all paired devices')}
            >
                <RefreshCw className="w-3 h-3" />
                {t('Reset QR Code')}
            </button>
            <div className="mt-2 flex items-start gap-2 rounded-lg bg-linear-to-r from-cyan-900/30 to-emerald-900/30 border border-cyan-500/20 p-2.5 text-sm max-w-md">
                <Lightbulb className="w-4 h-4 mt-0.5 shrink-0 text-cyan-300" />
                <div>
                    <span className="text-xs font-medium text-cyan-300">{t('Tip')}</span>
                    <p className="mt-0.5 text-muted-foreground">
                        {t(
                            'After scanning, use "Add to Home Screen" in your browser to install it as an app, no need to rescan.'
                        )}
                    </p>
                </div>
            </div>
        </div>
    );
};
