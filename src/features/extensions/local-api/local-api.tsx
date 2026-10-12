import { Plug } from 'lucide-react';
import { ExtensionCtaLayout } from '@/features/extensions/extension-cta-layout/extension-cta-layout';
import { ExtensionHeader } from '@/features/extensions/extension-header/extension-header';
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

    if (server.enabled === true) {
        return (
            <ExtensionCtaLayout colorScheme="standard" haloIntensity={12}>
                <ExtensionHeader
                    icon={Plug}
                    title={t('Local API')}
                    colorScheme="standard"
                    titleTestId="local-api-title"
                    toggle={{
                        checked: server.enabled,
                        onCheckedChange: (checked) => void server.saveEnabled(checked),
                        ariaLabel: t('Enable Local API'),
                        testId: 'local-api-toggle',
                    }}
                />
                <LocalApiServerSettings
                    port={server.port}
                    status={server.status}
                    isStarting={server.isStarting}
                    onPortChange={(value) => void server.savePort(value)}
                    onRetry={() => void server.retry()}
                />
                <LocalApiEndpoints port={server.port} isServerRunning={isServerRunning} />
            </ExtensionCtaLayout>
        );
    }

    return (
        <ExtensionCtaLayout colorScheme="standard">
            <ExtensionHeader
                icon={Plug}
                title={t('Local API')}
                colorScheme="standard"
                titleTestId="local-api-title"
                subtitle={t('Transcribe audio from your own scripts and apps.')}
            />
            <LocalApiCta onEnable={() => void server.saveEnabled(true)} />
        </ExtensionCtaLayout>
    );
};
