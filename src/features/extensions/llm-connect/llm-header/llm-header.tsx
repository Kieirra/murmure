import { useTranslation } from '@/i18n';
import { Typography } from '@/components/typography';
import { Page } from '@/components/page';
import { Sparkles } from 'lucide-react';

export const LLMHeader = () => {
    const { t } = useTranslation();

    return (
        <Page.Header>
            <Typography.MainTitle className="flex items-center gap-2">
                <Sparkles className="w-6 h-6 text-sky-400" />
                {t('Prompt Mode')}
            </Typography.MainTitle>
            <Typography.Paragraph>{t('Rewrite each dictation with a language model.')}</Typography.Paragraph>
        </Page.Header>
    );
};
