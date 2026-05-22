import { AlertTriangle } from 'lucide-react';
import { useTranslation } from '@/i18n';

export const SmartMicWaylandNotice = () => {
    const { t } = useTranslation();

    return (
        <div
            data-testid="smart-mic-wayland-notice"
            className="rounded-md border border-yellow-500/30 bg-yellow-500/10 px-4 py-3 flex items-start gap-3"
        >
            <AlertTriangle className="w-4 h-4 shrink-0 text-yellow-300 mt-0.5" />
            <div className="space-y-1">
                <p className="text-sm text-yellow-300">{t('Smart Mic is not available on Wayland.')}</p>
                <p className="text-sm text-muted-foreground">{t('Switch to an X11 session to use it.')}</p>
            </div>
        </div>
    );
};
