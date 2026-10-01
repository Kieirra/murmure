import { Switch } from '@/components/switch';
import clsx from 'clsx';
import { useTranslation } from '@/i18n';
import { LLMMode } from '@/features/extensions/llm-connect/hooks/use-llm-connect';
import { SUB_ITEM_KEY } from '../import-export.constants';

interface LlmConnectSubItemsProps {
    modes: LLMMode[];
    selection: Record<string, boolean>;
    onToggle: (key: string, checked: boolean) => void;
    disabled?: boolean;
}

export const LlmConnectSubItems = ({ modes, selection, onToggle, disabled }: LlmConnectSubItemsProps) => {
    const { t } = useTranslation();

    const items = [
        { key: 'connection', label: t('LLM servers (Command Mode and Prompt Mode)') },
        ...('command' in selection ? [{ key: 'command', label: t('Command Mode') }] : []),
        ...modes.map((mode, index) => ({ key: SUB_ITEM_KEY.mode(index), label: mode.name })),
    ];

    return (
        <>
            {items.map((item) => (
                <label
                    key={item.key}
                    className={clsx('flex items-center gap-2 py-1', disabled ? 'cursor-not-allowed' : 'cursor-pointer')}
                >
                    <Switch
                        checked={selection[item.key] ?? false}
                        onCheckedChange={(checked) => onToggle(item.key, checked)}
                        disabled={disabled}
                        aria-label={item.label}
                    />
                    <span className="text-sm text-muted-foreground">{item.label}</span>
                </label>
            ))}
        </>
    );
};
