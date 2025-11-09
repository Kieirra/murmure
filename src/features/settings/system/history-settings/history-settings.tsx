import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { Switch } from '@/components/switch';
import { useHistoryPersistenceState } from './hooks/use-history-persistence-state';
import { Shield } from 'lucide-react';

export const HistorySettings = () => {
    const { persistHistory, setPersistHistory } = useHistoryPersistenceState();

    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title className="flex items-center gap-2">
                    <Shield className="w-4 h-4 text-zinc-400" />
                    History persistence
                </Typography.Title>
                <Typography.Paragraph>
                    Store the last five transcriptions on disk. Disable to keep
                    history in memory only.
                </Typography.Paragraph>
            </SettingsUI.Description>
            <Switch
                checked={persistHistory}
                onCheckedChange={setPersistHistory}
            />
        </SettingsUI.Item>
    );
};
