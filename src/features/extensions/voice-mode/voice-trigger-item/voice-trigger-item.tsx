import { Typography } from '@/components/typography';
import { Input } from '@/components/input';
import { Button } from '@/components/button';
import { Switch } from '@/components/switch';
import { SettingsUI } from '@/components/settings-ui';
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/tooltip';
import { RotateCcw } from 'lucide-react';
import { useTranslation } from '@/i18n';

interface VoiceTriggerItemProps {
    title: string;
    description: string;
    wakeWord: string;
    onWakeWordChange: (value: string) => void;
    onBlur: () => void;
    placeholder: string;
    dataTestId: string;
    isEnabled: boolean;
    onToggleEnabled: () => void;
    defaultWord: string;
    onReset: () => void;
}

export const VoiceTriggerItem = ({
    title,
    description,
    wakeWord,
    onWakeWordChange,
    onBlur,
    placeholder,
    dataTestId,
    isEnabled,
    onToggleEnabled,
    defaultWord,
    onReset,
}: VoiceTriggerItemProps) => {
    const { t } = useTranslation();
    const isDefault = wakeWord === defaultWord;

    return (
        <SettingsUI.Item>
            <div className="w-2/5 shrink-0 space-y-0.5">
                <Typography.Title>{title}</Typography.Title>
                <Typography.Paragraph>{description}</Typography.Paragraph>
            </div>
            <div className="w-3/5 pl-4 flex items-center gap-2">
                <Input
                    value={wakeWord}
                    onChange={(e) => onWakeWordChange(e.target.value)}
                    onBlur={onBlur}
                    placeholder={placeholder}
                    maxLength={50}
                    disabled={!isEnabled}
                    aria-label={`Trigger word for ${title}`}
                    data-testid={dataTestId}
                    className="flex-1"
                />
                <Tooltip>
                    <TooltipTrigger asChild>
                        <Button
                            variant="ghost"
                            size="icon"
                            onClick={onReset}
                            disabled={!isEnabled || isDefault}
                            className="shrink-0 h-8 w-8"
                            data-testid={`${dataTestId}-reset`}
                        >
                            <RotateCcw className="w-3.5 h-3.5" />
                        </Button>
                    </TooltipTrigger>
                    <TooltipContent>{t('Reset to default')}</TooltipContent>
                </Tooltip>
                <Switch checked={isEnabled} onCheckedChange={onToggleEnabled} data-testid={`${dataTestId}-toggle`} />
            </div>
        </SettingsUI.Item>
    );
};
