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
import { useMicState } from './hooks/use-mic-state';

export const MicSettings = () => {
    const { t } = useTranslation();
    const { currentMic, setMic, micList, isLoading } = useMicState();

    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title className="flex items-center gap-2">
                    <Mic className="w-4 h-4 text-zinc-400" />
                    {t('Microphone')}
                </Typography.Title>
                <Typography.Paragraph>
                    {t('Choose your preferred input device for recording.')}
                </Typography.Paragraph>
            </SettingsUI.Description>
            <div className={isLoading ? 'opacity-50' : ''}>
                <Select
                    value={currentMic}
                    onValueChange={setMic}
                    disabled={isLoading}
                >
                    <SelectTrigger
                        className="w-[240px]"
                        data-testid="mic-select"
                    >
                        <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                        {micList.map((mic) => (
                            <SelectItem key={mic.id} value={mic.id}>
                                {mic.label}
                            </SelectItem>
                        ))}
                    </SelectContent>
                </Select>
            </div>
        </SettingsUI.Item>
    );
};
