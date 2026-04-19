import type { ViewMode } from '../smartmic.types';
import type { StringKey } from '../i18n/strings';
import { useI18n } from '../i18n/use-i18n';

const TABS: { mode: ViewMode; labelKey: StringKey }[] = [
    { mode: 'remote', labelKey: 'tabs.remote' },
    { mode: 'transcription', labelKey: 'tabs.transcription' },
    { mode: 'translation', labelKey: 'tabs.translation' },
];

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
