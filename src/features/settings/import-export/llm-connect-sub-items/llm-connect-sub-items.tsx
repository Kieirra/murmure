import { Switch } from '@/components/switch';
import clsx from 'clsx';
import { useTranslation } from '@/i18n';
import { LLMMode } from '@/features/personalize/llm-connect/hooks/use-llm-connect';
import { SUB_ITEM_KEY } from '../import-export.constants';

interface LlmConnectSubItemsProps {
    modes: LLMMode[];
    selection: Record<string, boolean>;
    onToggle: (key: string, checked: boolean) => void;
    disabled?: boolean;
}

export const LlmConnectSubItems = ({ modes, selection, onToggle, disabled }: LlmConnectSubItemsProps) => {
    const { t } = useTranslation();

    return (
        <>
            <label className={clsx('flex items-center gap-2 py-1', disabled ? 'cursor-not-allowed' : 'cursor-pointer')}>
                <Switch
                    checked={selection['connection'] ?? false}
                    onCheckedChange={(checked) => onToggle('connection', checked)}
                    disabled={disabled}
                    aria-label={t('Connection Settings')}
                />
                <span className="text-sm text-muted-foreground">{t('Connection Settings')}</span>
            </label>
            {modes.map((mode, index) => (
                <label
                    key={mode.name}
                    className={clsx('flex items-center gap-2 py-1', disabled ? 'cursor-not-allowed' : 'cursor-pointer')}
                >
                    <Switch
                        checked={selection[SUB_ITEM_KEY.mode(index)] ?? false}
                        onCheckedChange={(checked) => onToggle(SUB_ITEM_KEY.mode(index), checked)}
                        disabled={disabled}
                        aria-label={mode.name}
                    />
                    <span className="text-sm text-muted-foreground">{mode.name}</span>
                </label>
            ))}
        </>
    );
};
