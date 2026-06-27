import { Mic } from 'lucide-react';
import { useTranslation } from '@/i18n';

export const HistoryEmptyState = () => {
    const { t } = useTranslation();

    return (
        <div className="flex flex-col items-center gap-3 pt-16 pb-6 text-center">
            <Mic className="w-8 h-8 text-muted-foreground" />
            <p className="text-sm text-muted-foreground">
                {t('Start speaking to see your transcriptions appear here.')}
            </p>
        </div>
    );
};
