import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { AlertTriangle, Mic } from 'lucide-react';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/select';
import { useTranslation } from '@/i18n';
import { useIsWayland } from '@/components/hooks/use-linux-session-type';
import {
    RecordMode,
    useRecordModeState,
} from '@/features/settings/system/record-mode-settings/hooks/use-record-mode-state.ts';

const SUPPORTED_RECORD_MODE: { key: RecordMode; label: string }[] = [
    { key: 'push_to_talk', label: 'Push to talk' },
    { key: 'toggle_to_talk', label: 'Toggle to talk' },
];

export const RecordModeSettings = () => {
    const { t } = useTranslation();
    const { recordMode, setRecordMode } = useRecordModeState();
    const isWayland = useIsWayland();

    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title className="flex items-center gap-2">
                    <Mic className="w-4 h-4 text-muted-foreground" />
                    {t('Record mode')}
                </Typography.Title>
                <Typography.Paragraph>{t('Choose how recording is triggered.')}</Typography.Paragraph>
            </SettingsUI.Description>
            <Select value={recordMode} onValueChange={setRecordMode}>
                <SelectTrigger className="w-[260px]" data-testid="record-mode-select">
                    <SelectValue />
                </SelectTrigger>
                <SelectContent>
                    {SUPPORTED_RECORD_MODE.map((mode) => {
                        const showWarning = isWayland && mode.key === 'push_to_talk';
                        return (
                            <SelectItem key={mode.key} value={mode.key}>
                                {t(mode.label)}
                                {showWarning && (
                                    <span className="ml-2 inline-flex items-center gap-1 text-xs text-yellow-300/90">
                                        <AlertTriangle className="w-3 h-3 shrink-0" />
                                        {t('may be unstable on Wayland')}
                                    </span>
                                )}
                            </SelectItem>
                        );
                    })}
                </SelectContent>
            </Select>
        </SettingsUI.Item>
    );
};
