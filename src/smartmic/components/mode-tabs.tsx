import type { ViewMode } from '../types';
import { useI18n } from '../i18n/use-i18n';
import { TABS } from './mode-tabs.helpers';

interface ModeTabsProps {
    activeMode: ViewMode;
    onModeChange: (mode: ViewMode) => void;
}

export const ModeTabs = ({ activeMode, onModeChange }: ModeTabsProps) => {
    const { t } = useI18n();
    return (
        <div role="tablist" className="flex h-11 border-b border-[#222] bg-[#0a0a0a] shrink-0">
            {TABS.map(({ mode, labelKey }) => (
                <button
                    key={mode}
                    role="tab"
                    type="button"
                    aria-selected={activeMode === mode}
                    className={`flex-1 text-sm font-medium ${
                        activeMode === mode
                            ? 'text-sky-400 border-b-2 border-sky-400'
                            : 'text-[#888]'
                    }`}
                    onClick={() => onModeChange(mode)}
                >
                    {t(labelKey)}
                </button>
            ))}
        </div>
    );
};
