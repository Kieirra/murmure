import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { Switch } from '@/components/switch';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/select';
import { useHistoryPersistenceState } from './hooks/use-history-persistence-state';
import { useHistoryLimitState } from './hooks/use-history-limit-state';
import { Shield } from 'lucide-react';
import { useTranslation } from '@/i18n';

export const HistorySettings = () => {
    const { persistHistory, setPersistHistory } = useHistoryPersistenceState();
    const { historyLimit, setHistoryLimit } = useHistoryLimitState();
    const { t } = useTranslation();

    return (
        <>
            <SettingsUI.Item>
                <SettingsUI.Description>
                    <Typography.Title className="flex items-center gap-2">
                        <Shield className="w-4 h-4 text-muted-foreground" />
                        {t('History persistence')}
                    </Typography.Title>
                    <Typography.Paragraph>
                        {t('Store the last five transcriptions on disk. Disable to keep history in memory only.')}
                    </Typography.Paragraph>
                </SettingsUI.Description>
                <Switch checked={persistHistory} onCheckedChange={setPersistHistory} />
            </SettingsUI.Item>

            <SettingsUI.Item>
                <SettingsUI.Description>
                    <Typography.Title className="flex items-center gap-2">
                        <Shield className="w-4 h-4 text-muted-foreground" />
                        {t('Recent Activity Limit')}
                    </Typography.Title>
                    <Typography.Paragraph>
                        {t('Maximum number of transcriptions to keep (between 0 and 5).')}
                    </Typography.Paragraph>
                </SettingsUI.Description>
                <Select
                    value={historyLimit.toString()}
                    onValueChange={(val) => setHistoryLimit(parseInt(val, 10))}
                >
                    <SelectTrigger className="w-[100px]" data-testid="history-limit-select">
                        <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                        {[0, 1, 2, 3, 4, 5].map((num) => (
                            <SelectItem key={num} value={num.toString()}>
                                {num}
                            </SelectItem>
                        ))}
                    </SelectContent>
                </Select>
            </SettingsUI.Item>
        </>
    );
};
