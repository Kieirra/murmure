import { Plug } from 'lucide-react';
import { ExtensionActiveCard } from '@/components/extension-active-card';
import { Page } from '@/components/page';
import { Typography } from '@/components/typography';
import { useTranslation } from '@/i18n';
import { useLocalApiServer } from './hooks/use-local-api-server';
import { LocalApiCta } from './local-api-cta/local-api-cta';
import { LocalApiEndpoints } from './local-api-endpoints/local-api-endpoints';
import { LocalApiServerSettings } from './local-api-server-settings/local-api-server-settings';

export const LocalApi = () => {
    const { t } = useTranslation();
    const server = useLocalApiServer();

    if (server.enabled === null) {
        return null;
    }

    const isServerRunning = !server.isStarting && server.status?.state === 'running';

    return (
        <main>
            <div className="space-y-6">
                <Page.Header>
                    <Typography.MainTitle className="flex items-center gap-2" data-testid="local-api-title">
                        <Plug className="w-6 h-6 text-sky-400" />
                        {t('Local API')}
                    </Typography.MainTitle>
                    <Typography.Paragraph>{t('Transcribe audio from your own scripts and apps.')}</Typography.Paragraph>
                </Page.Header>

                {server.enabled === false && <LocalApiCta onEnable={() => void server.saveEnabled(true)} />}

                {server.enabled === true && (
                    <>
                        <ExtensionActiveCard
                            icon={Plug}
                            label={t('Local API is active')}
                            checked
                            onCheckedChange={(checked) => void server.saveEnabled(checked)}
                            testId="local-api-toggle"
                        />
                        <LocalApiServerSettings
                            port={server.port}
                            status={server.status}
                            isStarting={server.isStarting}
                            onPortChange={(value) => void server.savePort(value)}
                            onRetry={() => void server.retry()}
                        />
                        <LocalApiEndpoints port={server.port} isServerRunning={isServerRunning} />
                    </>
                )}
            </div>
        </main>
    );
};
