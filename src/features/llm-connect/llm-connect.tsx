import { useTranslation } from '@/i18n';

export const LLMConnect = () => {
    const { t } = useTranslation();
    return (
        <div className="flex flex-col gap-4 p-4">
            <h1 className="text-2xl font-bold">{t('LLM Connect')}</h1>
        </div>
    );
};
