import { Typography } from '@/components/typography';
import { Page } from '@/components/page';
import { useTranslation } from '@/i18n';
import { CliCommandsPanel } from '../cli-commands-panel/cli-commands-panel';

export const ShortcutsCli = () => {
    const { t } = useTranslation();

    return (
        <main>
            <div className="space-y-4">
                <Page.Header>
                    <Typography.MainTitle data-testid="shortcuts-title">{t('Shortcuts')}</Typography.MainTitle>
                </Page.Header>
                <CliCommandsPanel />
            </div>
        </main>
    );
};
