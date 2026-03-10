import { Page } from '@/components/page';
import { Typography } from '@/components/typography';
import { useTranslation } from '@/i18n';
import { ExportSection } from './export-section/export-section';
import { ImportSection } from './import-section/import-section';

export const ImportExport = () => {
    const { t } = useTranslation();

    return (
        <main className="space-y-8">
            <Page.Header>
                <Typography.MainTitle data-testid="import-export-title">
                    {t('Import / Export')}
                </Typography.MainTitle>
                <Typography.Paragraph className="text-muted-foreground">
                    {t(
                        'Save or restore your Murmure configuration.'
                    )}
                </Typography.Paragraph>
            </Page.Header>

            <ExportSection />

            <div className="border-t border-border" />

            <ImportSection />
        </main>
    );
};
