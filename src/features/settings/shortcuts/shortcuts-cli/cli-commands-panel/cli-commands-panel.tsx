import { Info, Terminal } from 'lucide-react';
import { Trans } from 'react-i18next';
import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { ExternalLink } from '@/components/external-link';
import { useTranslation } from '@/i18n';
import { CliCommandRow } from './cli-command-row/cli-command-row';
import { CLI_COMMANDS, CLI_DOC_URL } from './cli-commands-panel.helpers';

export const CliCommandsPanel = () => {
    const { t } = useTranslation();

    return (
        <div className="space-y-6">
            <div className="w-full flex items-start gap-3 rounded-md bg-muted/40 p-4">
                <Info className="w-4 h-4 mt-0.5 shrink-0 text-sky-400" />
                <div className="space-y-2">
                    <Typography.Title className="font-semibold text-sm">
                        {t('Shortcuts are managed by your system')}
                    </Typography.Title>
                    <Typography.Paragraph className="text-foreground text-xs">
                        {t(
                            'Shortcuts are managed by your operating system. Bind these commands in your keyboard settings.'
                        )}
                    </Typography.Paragraph>
                    <Typography.Paragraph className="text-foreground text-xs">
                        <Trans
                            i18nKey="Once <cmd>murmure --transcription</cmd> works, every other command below binds the same way."
                            components={{
                                cmd: <code className="font-mono text-[11px] px-1 py-0.5 bg-surface-header rounded" />,
                            }}
                        />
                    </Typography.Paragraph>
                    <ExternalLink href={CLI_DOC_URL} className="text-xs font-semibold">
                        {t('Setup guide')}
                    </ExternalLink>
                </div>
            </div>

            <SettingsUI.Section title={t('Available commands')} icon={Terminal} titleTestId="cli-commands-title">
                {CLI_COMMANDS.map((cmd, idx) => (
                    <div key={cmd.id}>
                        <CliCommandRow label={cmd.label} command={cmd.command} />
                        {idx < CLI_COMMANDS.length - 1 && <SettingsUI.Separator />}
                    </div>
                ))}
            </SettingsUI.Section>
        </div>
    );
};
