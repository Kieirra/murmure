import { CodeXml, FileCode2 } from 'lucide-react';
import { ExternalLink } from '@/components/external-link';
import { SettingsUI } from '@/components/settings-ui';
import { useLlmExtensionsEnabled } from '@/features/extensions/llm-connect/hooks/use-llm-extensions-enabled';
import { useTranslation } from '@/i18n';
import { EndpointDoc } from './endpoint-doc/endpoint-doc';
import { ENDPOINTS } from './local-api-endpoints.helpers';

interface LocalApiEndpointsProps {
    port: number;
    isServerRunning: boolean;
}

export const LocalApiEndpoints = ({ port, isServerRunning }: LocalApiEndpointsProps) => {
    const { t } = useTranslation();
    const availability = useLlmExtensionsEnabled();

    if (!availability.isLoaded) {
        return null;
    }

    return (
        <SettingsUI.Section title={t('Endpoints')} icon={CodeXml}>
            <div className="p-4 space-y-4">
                {ENDPOINTS.map((definition) => (
                    <EndpointDoc
                        key={definition.path}
                        definition={definition}
                        port={port}
                        isServerRunning={isServerRunning}
                        availability={availability}
                    />
                ))}
                <div className="flex items-center gap-1 text-xs">
                    <FileCode2 className="w-4 h-4 text-muted-foreground" />
                    {t('View')}{' '}
                    <ExternalLink href="https://github.com/Kieirra/murmure/blob/main/docs/features/api.md">
                        {t('API documentation')}
                    </ExternalLink>
                </div>
            </div>
        </SettingsUI.Section>
    );
};
