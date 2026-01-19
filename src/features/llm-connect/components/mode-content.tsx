import { useState, useEffect, useCallback } from 'react';
import { useTranslation } from '@/i18n';
import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { Button } from '@/components/button';
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from '@/components/select';
import { RefreshCw, Wrench } from 'lucide-react';
import clsx from 'clsx';
import { RenderKeys } from '@/components/render-keys';
import {
    LLMConnectSettings,
    LLMMode,
    OllamaModel,
} from '../hooks/use-llm-connect';

interface ModeContentProps {
    activeMode: LLMMode;
    activeModeIndex: number;
    modes: LLMMode[];
    models: OllamaModel[];
    isLoading: boolean;
    updateSettings: (updates: Partial<LLMConnectSettings>) => Promise<void>;
    onRefreshModels: () => void;
}

export const ModeContent = ({
    activeMode,
    activeModeIndex,
    modes,
    models,
    isLoading,
    updateSettings,
    onRefreshModels,
}: ModeContentProps) => {
    const { t } = useTranslation();
    const [promptDraft, setPromptDraft] = useState(activeMode.prompt);

    // Sync local draft when active mode changes
    useEffect(() => {
        setPromptDraft(activeMode.prompt);
    }, [activeMode.prompt, activeModeIndex]);

    // Autosave Prompt Debounce
    useEffect(() => {
        const timer = setTimeout(() => {
            if (promptDraft !== activeMode.prompt) {
                const newModes = [...modes];
                newModes[activeModeIndex] = {
                    ...activeMode,
                    prompt: promptDraft,
                };
                updateSettings({ modes: newModes });
            }
        }, 1000);
        return () => clearTimeout(timer);
    }, [promptDraft, activeMode, activeModeIndex, modes, updateSettings]);

    const handleModelChange = useCallback(
        (modelName: string) => {
            const newModes = [...modes];
            newModes[activeModeIndex] = {
                ...activeMode,
                model: modelName,
            };
            updateSettings({ modes: newModes });
        },
        [activeMode, activeModeIndex, modes, updateSettings]
    );

    return (
        <div className="flex flex-col gap-6 animate-in fade-in duration-300">
            <SettingsUI.Container>
                {/* Model */}
                <SettingsUI.Item>
                    <SettingsUI.Description>
                        <Typography.Title className="flex items-center gap-2">
                            <Wrench className="w-4 h-4 text-zinc-400" />
                            {t('Model')}
                        </Typography.Title>
                    </SettingsUI.Description>

                    <div className="flex gap-2 items-center">
                        <Select
                            value={activeMode.model}
                            onValueChange={handleModelChange}
                            disabled={models.length === 0}
                        >
                            <SelectTrigger className="w-[300px]">
                                <SelectValue
                                    placeholder={t('Select a model')}
                                />
                            </SelectTrigger>
                            <SelectContent>
                                {models.map((model) => (
                                    <SelectItem
                                        key={model.name}
                                        value={model.name}
                                    >
                                        {model.name}
                                    </SelectItem>
                                ))}
                            </SelectContent>
                        </Select>
                        <Button
                            onClick={onRefreshModels}
                            variant="ghost"
                            size="sm"
                            className="p-2"
                            title={t('Refresh Models')}
                        >
                            <RefreshCw
                                className={clsx(
                                    'w-4 h-4',
                                    isLoading && 'animate-spin'
                                )}
                            />
                        </Button>
                    </div>
                </SettingsUI.Item>

                <SettingsUI.Separator />

                {/* Prompt Editor */}
                <SettingsUI.Item className="flex-col! items-start gap-4">
                    <div className="flex w-full items-start">
                        <SettingsUI.Description className="flex-1">
                            <Typography.Title>{t('Prompt')}</Typography.Title>
                            <Typography.Paragraph>
                                {t(
                                    'Use {{TRANSCRIPT}} as the captured text and {{DICTIONARY}} as the word set defined in Settings → Custom Dictionary.'
                                )}
                            </Typography.Paragraph>
                        </SettingsUI.Description>
                        <div className="text-xs text-zinc-500 bg-zinc-900/50 px-2 rounded w-34">
                            <RenderKeys keyString={activeMode.shortcut} />
                        </div>
                    </div>

                    <div className="relative w-full">
                        <textarea
                            value={promptDraft}
                            onChange={(e) =>
                                setPromptDraft(e.target.value.slice(0, 4000))
                            }
                            maxLength={4000}
                            className="w-full h-[600px] px-4 py-3 bg-zinc-900/50 border border-zinc-800 rounded-lg text-sm focus:outline-none focus:ring-1 focus:ring-sky-500/50 font-mono resize-y"
                            placeholder={t('Enter your prompt here...')}
                        />
                        <div className="absolute bottom-3 right-3 flex flex-col gap-1 items-end pointer-events-none">
                            <span className="text-[10px] text-zinc-500 mb-1">
                                {promptDraft.length} / 4000
                            </span>
                        </div>
                    </div>
                </SettingsUI.Item>
            </SettingsUI.Container>
        </div>
    );
};
