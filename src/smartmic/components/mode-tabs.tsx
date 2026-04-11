import type { ViewMode } from '../types';

interface ModeTabsProps {
    activeMode: ViewMode;
    onModeChange: (mode: ViewMode) => void;
}

const tabs: { mode: ViewMode; label: string }[] = [
    { mode: 'remote', label: 'Remote' },
    { mode: 'transcription', label: 'Transcription' },
    { mode: 'translation', label: 'Translation' },
];

export const ModeTabs = ({ activeMode, onModeChange }: ModeTabsProps) => {
    return (
        <div role="tablist" className="flex h-8 border-b border-[#222] bg-[#0a0a0a] shrink-0">
            {tabs.map(({ mode, label }) => (
                <button
                    key={mode}
                    role="tab"
                    type="button"
                    aria-selected={activeMode === mode}
                    className={`flex-1 text-xs font-medium ${
                        activeMode === mode
                            ? 'text-sky-400 border-b-2 border-sky-400'
                            : 'text-[#888]'
                    }`}
                    onClick={() => onModeChange(mode)}
                >
                    {label}
                </button>
            ))}
        </div>
    );
};
