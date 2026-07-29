import { useTranslation } from '@/i18n';
import { Typography } from '@/components/typography';
import { Page } from '@/components/page';

export const LLMHeader = () => {
    const { t } = useTranslation();

    return (
        <Page.Header>
            <Typography.MainTitle>{t('LLM Connect')}</Typography.MainTitle>
        </Page.Header>
    );
};
