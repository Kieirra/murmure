import { useState, useEffect, useCallback } from 'react';
import { useTranslation } from '@/i18n';
import { SettingsUI } from '@/components/settings-ui';
import { AlertTriangle } from 'lucide-react';
import { HighlightedPromptEditor } from './highlighted-prompt-editor';
import { ModeActions } from './mode-actions/mode-actions';
import { ProviderModelPicker } from '../provider-model-picker/provider-model-picker';
import clsx from 'clsx';
import { LLMConnectSettings, LLMMode, OllamaModel } from '../hooks/use-llm-connect';

interface ModeContentProps {
    activeMode: LLMMode;
    activeModeIndex: number;
    modes: LLMMode[];
    models: OllamaModel[];
    isLoading: boolean;
    updateSettings: (updates: Partial<LLMConnectSettings>) => Promise<void>;
    onRefreshModels: () => void;
    remoteModels: OllamaModel[];
    isRemoteConfigured: boolean;
    isLocalConfigured: boolean;
    onRefreshRemoteModels: () => void;
}

export const ModeContent = ({
    activeMode,
    activeModeIndex,
    modes,
    models,
    isLoading,
    updateSettings,
    onRefreshModels,
    remoteModels,
    isRemoteConfigured,
    isLocalConfigured,
    onRefreshRemoteModels,
}: ModeContentProps) => {
    const { t } = useTranslation();
    const [promptDraft, setPromptDraft] = useState(activeMode.prompt);

    const activeProvider = activeMode.provider ?? 'local';
    const isRemote = activeProvider === 'remote';
    const promptMaxLength = isRemote ? undefined : 4000;

    // Sync local draft when active mode changes
    useEffect(() => {
        setPromptDraft(activeMode.prompt);
    }, [activeMode.prompt, activeModeIndex]);

    const updateActiveMode = useCallback(
        (updates: Partial<LLMMode>) => {
            const newModes = [...modes];
            newModes[activeModeIndex] = { ...activeMode, ...updates };
            void updateSettings({ modes: newModes });
        },
        [activeMode, activeModeIndex, modes, updateSettings]
    );

    // Autosave Prompt Debounce
    useEffect(() => {
        const timer = setTimeout(() => {
            if (promptDraft !== activeMode.prompt) {
                updateActiveMode({ prompt: promptDraft });
            }
        }, 1000);
        return () => clearTimeout(timer);
    }, [promptDraft, activeMode.prompt, activeModeIndex, updateActiveMode]);

    const promptExceedsLocalLimit = activeProvider === 'local' && promptDraft.length > 4000;

    return (
        <div className="flex flex-col gap-6">
            <SettingsUI.Container>
                {/* Model */}
                <ProviderModelPicker
                    provider={activeProvider}
                    model={activeMode.model}
                    onChange={updateActiveMode}
                    models={models}
                    remoteModels={remoteModels}
                    isLoading={isLoading}
                    isLocalConfigured={isLocalConfigured}
                    isRemoteConfigured={isRemoteConfigured}
                    onRefreshModels={onRefreshModels}
                    onRefreshRemoteModels={onRefreshRemoteModels}
                />

                <SettingsUI.Separator />

                <SettingsUI.Item>
                    <ModeActions modeIndex={activeModeIndex} />
                </SettingsUI.Item>

                <SettingsUI.Separator />

                {/* Prompt Editor */}
                <SettingsUI.Item className="flex-col! items-start">
                    <div className="relative w-full">
                        <HighlightedPromptEditor
                            value={promptDraft}
                            onChange={(value) => setPromptDraft(value)}
                            maxLength={promptMaxLength}
                            placeholder={t('Enter your prompt here...')}
                            className="w-full h-[500px]"
                        />
                        <div className="absolute bottom-3 right-3 flex flex-col gap-1 items-end pointer-events-none z-20">
                            <span
                                className={clsx(
                                    'text-[10px] mb-1',
                                    promptExceedsLocalLimit ? 'text-red-400' : 'text-muted-foreground'
                                )}
                            >
                                {isRemote ? promptDraft.length : `${promptDraft.length} / 4000`}
                            </span>
                        </div>
                    </div>

                    <ul className="list-disc list-inside text-xs text-muted-foreground space-y-0.5 -mt-2">
                        <li>
                            <code>{'{{TRANSCRIPT}}'}</code>
                            {': '}
                            {t('the captured text')}
                        </li>
                        <li>
                            <code>{'{{DICTIONARY}}'}</code>
                            {': '}
                            {t('the word set defined in Personalize > Dictionary')}
                        </li>
                    </ul>

                    {promptExceedsLocalLimit && (
                        <div className="flex items-center gap-2 text-xs text-yellow-300/90">
                            <AlertTriangle className="w-3.5 h-3.5" />
                            {t(
                                'Prompt exceeds the recommended limit for local models. This may cause context overflow errors.'
                            )}
                        </div>
                    )}
                </SettingsUI.Item>
            </SettingsUI.Container>
        </div>
    );
};
