import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { Mic } from 'lucide-react';
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from '@/components/select';
import { useTranslation } from '@/i18n';
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

    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title className="flex items-center gap-2">
                    <Mic className="w-4 h-4 text-zinc-400" />
                    {t('Record mode')}
                </Typography.Title>
                <Typography.Paragraph>
                    {t('Choose how recording is triggered.')}
                </Typography.Paragraph>
            </SettingsUI.Description>
            <Select value={recordMode} onValueChange={setRecordMode}>
                <SelectTrigger
                    className="w-[180px]"
                    data-testid="record-mode-select"
                >
                    <SelectValue />
                </SelectTrigger>
                <SelectContent>
                    {SUPPORTED_RECORD_MODE.map((mode) => (
                        <SelectItem key={mode.key} value={mode.key}>
                            {t(mode.label)}
                        </SelectItem>
                    ))}
                </SelectContent>
            </Select>
        </SettingsUI.Item>
    );
};
