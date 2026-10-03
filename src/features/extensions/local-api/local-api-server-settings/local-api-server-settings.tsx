import clsx from 'clsx';
import { RotateCw, Server } from 'lucide-react';
import { Button } from '@/components/button';
import { NumberInput } from '@/components/number-input';
import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { useTranslation } from '@/i18n';
import type { LocalApiStatus } from '../local-api.types';
import { DEFAULT_API_PORT, MAX_API_PORT, MIN_API_PORT } from '../local-api.helpers';
import { getStatusView } from './local-api-server-settings.helpers';

interface LocalApiServerSettingsProps {
    port: number;
    status: LocalApiStatus | null;
    isStarting: boolean;
    onPortChange: (value: number) => void;
    onRetry: () => void;
}

export const LocalApiServerSettings = ({
    port,
    status,
    isStarting,
    onPortChange,
    onRetry,
}: LocalApiServerSettingsProps) => {
    const { t } = useTranslation();
    const view = getStatusView(status, isStarting);
    const statusPort = status?.port ?? port;

    return (
        <SettingsUI.Section title={t('Server')} icon={Server}>
            <SettingsUI.Item>
                <div role="status" aria-live="polite" className="flex items-start gap-3 min-w-0">
                    <span
                        className={clsx(
                            'mt-1.5 h-2 w-2 shrink-0 rounded-full',
                            view.tone === 'success' && 'bg-emerald-400',
                            view.tone === 'error' && 'bg-red-400',
                            view.tone === 'neutral' && 'bg-muted-foreground'
                        )}
                    />
                    <div className="space-y-1 min-w-0">
                        <Typography.Title className="text-sm" data-testid="local-api-status">
                            {t(view.label, { port: statusPort })}
                        </Typography.Title>
                        {view.hint !== undefined && <Typography.Paragraph>{t(view.hint)}</Typography.Paragraph>}
                    </div>
                </div>
                {view.canRetry && (
                    <Button variant="outline" size="sm" onClick={onRetry} data-testid="local-api-retry">
                        <RotateCw className="w-3.5 h-3.5" />
                        {t('Retry')}
                    </Button>
                )}
            </SettingsUI.Item>
            <SettingsUI.Separator />
            <SettingsUI.Item>
                <SettingsUI.Description>
                    <Typography.Title>{t('Port')}</Typography.Title>
                    <Typography.Paragraph>{t('Port used by the local API (1024 to 65535).')}</Typography.Paragraph>
                </SettingsUI.Description>
                <NumberInput
                    min={MIN_API_PORT}
                    max={MAX_API_PORT}
                    value={port}
                    onValueChange={(value) => onPortChange(value ?? DEFAULT_API_PORT)}
                    data-testid="local-api-port"
                />
            </SettingsUI.Item>
        </SettingsUI.Section>
    );
};
