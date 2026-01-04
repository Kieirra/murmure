import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { Terminal } from 'lucide-react';
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from '@/components/select';
import { useTranslation } from '@/i18n';
import { useLogLevelState } from './hooks/use-log-level-state';

const LOG_LEVELS = [
    { value: 'error', label: 'Error' },
    { value: 'warn', label: 'Warning' },
    { value: 'info', label: 'Info' },
    { value: 'debug', label: 'Debug' },
    { value: 'trace', label: 'Trace' },
];

export const LogLevelSettings = () => {
    const { t } = useTranslation();
    const { logLevel, setLogLevel } = useLogLevelState();

    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title className="flex items-center gap-2">
                    <Terminal className="w-4 h-4 text-zinc-400" />
                    {t('Log Level')}
                </Typography.Title>
                <Typography.Paragraph>
                    {t('Set the verbosity of application logs.')}
                </Typography.Paragraph>
            </SettingsUI.Description>
            <Select value={logLevel} onValueChange={setLogLevel}>
                <SelectTrigger
                    className="w-[180px]"
                    data-testid="log-level-select"
                >
                    <SelectValue />
                </SelectTrigger>
                <SelectContent>
                    {LOG_LEVELS.map((level) => (
                        <SelectItem key={level.value} value={level.value}>
                            {t(level.label)}
                        </SelectItem>
                    ))}
                </SelectContent>
            </Select>
        </SettingsUI.Item>
    );
};
