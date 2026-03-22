import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { Mic } from 'lucide-react';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/select';
import { useTranslation } from '@/i18n';
import {
    RecordMode,
    useRecordModeState,
} from '@/features/settings/system/record-mode-settings/hooks/use-record-mode-state.ts';
import { useIsWayland } from '@/hooks/use-is-wayland';

const SUPPORTED_RECORD_MODE: { key: RecordMode; label: string }[] = [
    { key: 'push_to_talk', label: 'Push to talk' },
    { key: 'toggle_to_talk', label: 'Toggle to talk' },
];

export const RecordModeSettings = () => {
    const { t } = useTranslation();
    const { recordMode, setRecordMode } = useRecordModeState();
    const isWayland = useIsWayland();

    const displayedRecordMode = isWayland === true ? 'toggle_to_talk' : recordMode;

    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title className="flex items-center gap-2">
                    <Mic className="w-4 h-4 text-muted-foreground" />
                    {t('Record mode')}
                </Typography.Title>
                <Typography.Paragraph>{t('Choose how recording is triggered.')}</Typography.Paragraph>
                {isWayland === true && (
                    <Typography.Paragraph className="text-xs text-muted-foreground">
                        {t('Push-to-Talk is not supported on Wayland.')}
                    </Typography.Paragraph>
                )}
            </SettingsUI.Description>
            <Select value={displayedRecordMode} onValueChange={setRecordMode} disabled={isWayland === true}>
                <SelectTrigger className="w-[180px]" data-testid="record-mode-select">
                    <SelectValue />
                </SelectTrigger>
                <SelectContent>
                    {SUPPORTED_RECORD_MODE.map((mode) => (
                        <SelectItem key={mode.key} value={mode.key} disabled={isWayland === true && mode.key === 'push_to_talk'}>
                            {t(mode.label)}
                        </SelectItem>
                    ))}
                </SelectContent>
            </Select>
        </SettingsUI.Item>
    );
};
