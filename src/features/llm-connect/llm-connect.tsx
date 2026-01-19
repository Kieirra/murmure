import { useTranslation } from '@/i18n';
import { useState, useEffect } from 'react';
import { useLLMConnect, LLMMode } from './hooks/use-llm-connect';
import { Button } from '@/components/button';
import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { Page } from '@/components/page';
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from '@/components/select';
import {
    RefreshCw,
    Wrench,
    Plus,
    MoreVertical,
    Pencil,
    Trash2,
} from 'lucide-react';
import { toast } from 'react-toastify';
import {
    getPresetLabel,
    getPresetTypes,
    getPromptByPreset,
    getStatusIcon,
    getStatusText,
} from './llm-connect.helpers';
import { PromptPresetType } from './llm-connect.constants';
import { Input } from '@/components/input';
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuTrigger,
} from '@/components/dropdown-menu';
import {
    Dialog,
    DialogContent,
    DialogFooter,
    DialogHeader,
    DialogTitle,
} from '@/components/dialog';
import clsx from 'clsx';
import { RenderKeys } from '@/components/render-keys';
import { LLMConnectOnboarding } from './onboarding/llm-connect-onboarding';

export const LLMConnect = () => {
    const { t, i18n } = useTranslation();
    const {
        settings,
        models,
        connectionStatus,
        isLoading,
        updateSettings,
        testConnection,
        fetchModels,
        pullModel,
    } = useLLMConnect();

    // Draft for the active prompt to allow debounced saving
    const [promptDraft, setPromptDraft] = useState('');
    const [renameDialogOpen, setRenameDialogOpen] = useState(false);
    const [modeToRename, setModeToRename] = useState<{
        index: number;
        name: string;
    } | null>(null);

    // Sync local draft when active mode changes or settings load
    const activeModeIndex = settings.active_mode_index;
    const activeMode = settings.modes[activeModeIndex];
    const [showModelSelector, setShowModelSelector] = useState(false);

    useEffect(() => {
        if (activeMode) {
            setPromptDraft(activeMode.prompt);
        }
    }, [activeMode?.prompt, activeModeIndex]);

    // Autosave Prompt Debounce
    useEffect(() => {
        const timer = setTimeout(() => {
            if (activeMode && promptDraft !== activeMode.prompt) {
                const newModes = [...settings.modes];
                newModes[activeModeIndex] = {
                    ...activeMode,
                    prompt: promptDraft,
                };
                updateSettings({ modes: newModes });
            }
        }, 1000);
        return () => clearTimeout(timer);
    }, [
        promptDraft,
        activeMode,
        activeModeIndex,
        settings.modes,
        updateSettings,
    ]);

    const handleTabChange = (index: number) => {
        // Force save current draft before switching if dirty (optional, but safer)
        if (activeMode && promptDraft !== activeMode.prompt) {
            const newModes = [...settings.modes];
            newModes[activeModeIndex] = { ...activeMode, prompt: promptDraft };
            // We await this update implicitly by updating state, but updateSettings is async.
            // However, since we update the whole modes array, it's fine.
            updateSettings({ modes: newModes, active_mode_index: index });
        } else {
            updateSettings({ active_mode_index: index });
        }
    };

    const handleAddMode = (preset?: PromptPresetType) => {
        if (settings.modes.length >= 4) return;

        let name = t('New Mode');
        let prompt = '';
        if (preset) {
            name = t(getPresetLabel(preset));
            prompt = getPromptByPreset(preset, i18n.language);
        }

        const newMode: LLMMode = {
            name,
            prompt,
            model:
                activeMode?.model || (models.length > 0 ? models[0].name : ''),
            shortcut: `Ctrl + Shift + ${settings.modes.length + 1}`,
        };

        const newModes = [...settings.modes, newMode];
        updateSettings({
            modes: newModes,
            active_mode_index: newModes.length - 1,
        });
    };

    const handleDeleteMode = (index: number) => {
        if (settings.modes.length <= 1) {
            toast.error(t('Cannot delete the last mode'));
            return;
        }

        const newModes = settings.modes.filter((_, i) => i !== index);

        let newIndex = settings.active_mode_index;
        if (index < newIndex) {
            newIndex = newIndex - 1;
        } else if (index === newIndex) {
            newIndex = Math.min(newIndex, newModes.length - 1);
        }

        const renamedModes = newModes.map((m, i) => ({
            ...m,
            shortcut: `Ctrl + Shift + ${i + 1}`,
        }));

        updateSettings({ modes: renamedModes, active_mode_index: newIndex });
    };

    const openRenameDialog = (index: number) => {
        setModeToRename({ index, name: settings.modes[index].name });
        setRenameDialogOpen(true);
    };

    const handleRenameSubmit = () => {
        if (modeToRename) {
            const newModes = [...settings.modes];
            newModes[modeToRename.index] = {
                ...newModes[modeToRename.index],
                name: modeToRename.name,
            };
            updateSettings({ modes: newModes });
            setRenameDialogOpen(false);
            setModeToRename(null);
        }
    };

    const handleModelChange = (modelName: string) => {
        if (activeMode) {
            const newModes = [...settings.modes];
            newModes[activeModeIndex] = { ...activeMode, model: modelName };
            updateSettings({ modes: newModes });
        }
    };

    const handleTestConnection = async () => {
        const result = await testConnection();
        if (result) {
            toast.success(t('Connection successful'), { autoClose: 1500 });
            await fetchModels();
        } else {
            toast.error(t('Connection failed'));
        }
    };

    const handleResetOnboarding = async () => {
        try {
            await updateSettings({ onboarding_completed: false });
        } catch {
            toast.error(t('Failed to reset onboarding'));
        }
    };

    if (!settings.modes || settings.modes.length === 0) {
        return (
            <div className="p-8 text-center text-zinc-500">
                {t('Loading modes...')}
            </div>
        );
    }

    if (!settings.onboarding_completed || showModelSelector) {
        return (
            <main>
                <LLMConnectOnboarding
                    settings={settings}
                    testConnection={testConnection}
                    pullModel={pullModel}
                    updateSettings={updateSettings}
                    initialStep={showModelSelector ? 2 : 0}
                    models={models}
                    fetchModels={fetchModels}
                    completeOnboarding={async () => {
                        await fetchModels();
                        if (showModelSelector) {
                            setShowModelSelector(false);
                            return;
                        }
                        await updateSettings({ onboarding_completed: true });
                    }}
                />
            </main>
        );
    }

    return (
        <main>
            {/* Rename Dialog */}
            <Dialog open={renameDialogOpen} onOpenChange={setRenameDialogOpen}>
                <DialogContent>
                    <DialogHeader>
                        <DialogTitle>{t('Rename Mode')}</DialogTitle>
                    </DialogHeader>
                    <div className="py-4">
                        <Input
                            value={modeToRename?.name || ''}
                            onChange={(e) =>
                                setModeToRename((prev) =>
                                    prev
                                        ? { ...prev, name: e.target.value }
                                        : null
                                )
                            }
                            placeholder={t('Mode Name')}
                        />
                    </div>
                    <DialogFooter>
                        <Button
                            variant="ghost"
                            onClick={() => setRenameDialogOpen(false)}
                        >
                            {t('Cancel')}
                        </Button>
                        <Button onClick={handleRenameSubmit}>
                            {t('Save')}
                        </Button>
                    </DialogFooter>
                </DialogContent>
            </Dialog>

            <div className="space-y-6">
                <Page.Header>
                    <div className="flex justify-between items-center w-full">
                        <div className="flex flex-col gap-2">
                            <Typography.MainTitle className="flex items-center gap-2">
                                {t('LLM Connect')}
                            </Typography.MainTitle>
                            <Typography.Paragraph className="text-zinc-400">
                                {t('Configure your LLM prompts.')}
                            </Typography.Paragraph>
                        </div>

                        {/* Connection Status Top Right */}
                        <div
                            className={clsx(
                                'flex items-center gap-2 px-3 py-1.5 rounded-full text-xs font-medium border transiton-colors',
                                connectionStatus === 'connected'
                                    ? 'bg-emerald-500/10 text-emerald-500 border-emerald-500/20'
                                    : connectionStatus === 'error'
                                      ? 'bg-red-500/10 text-red-500 border-red-500/20'
                                      : 'bg-zinc-800 text-zinc-400 border-zinc-700'
                            )}
                        >
                            {getStatusIcon(connectionStatus)}
                            {getStatusText(connectionStatus, t)}
                        </div>
                    </div>
                </Page.Header>

                {/* Tabs Header */}
                <div className="flex flex-wrap border-zinc-800 px-1 mb-0">
                    {settings.modes.map((mode, index) => (
                        <div
                            key={index}
                            className={clsx(
                                'group relative flex items-center gap-2 px-4 py-2 transition-colors cursor-pointer select-none',
                                activeModeIndex === index
                                    ? 'bg-zinc-800/80 text-sky-400 border-b-2 border-sky-500'
                                    : 'bg-zinc-900/50 text-zinc-400 hover:bg-zinc-800 hover:text-zinc-200'
                            )}
                            onClick={() => handleTabChange(index)}
                        >
                            <span className="text-sm font-medium">
                                {mode.name}
                            </span>
                            <DropdownMenu>
                                <DropdownMenuTrigger asChild>
                                    <button
                                        className={clsx(
                                            'opacity-0 group-hover:opacity-100 p-1 rounded hover:bg-zinc-700 transition-all',
                                            activeModeIndex === index &&
                                                'opacity-100'
                                        )}
                                        onClick={(e) => e.stopPropagation()}
                                    >
                                        <MoreVertical className="w-4 h-4" />
                                    </button>
                                </DropdownMenuTrigger>
                                <DropdownMenuContent align="start" className="w-40 bg-zinc-900 border-zinc-700 text-zinc-300">
                                    <DropdownMenuItem
                                        className="focus:bg-zinc-800 focus:text-zinc-200"
                                        onClick={(e) => {
                                            e.stopPropagation();
                                            openRenameDialog(index);
                                        }}
                                    >
                                        <Pencil className="w-3 h-3 mr-2" />
                                        {t('Rename')}
                                    </DropdownMenuItem>
                                    <DropdownMenuItem
                                        onClick={(e) => {
                                            e.stopPropagation();
                                            handleDeleteMode(index);
                                        }}
                                        className="text-red-400 focus:text-red-400 focus:bg-zinc-800"
                                        disabled={settings.modes.length <= 1}
                                    >
                                        <Trash2 className="w-3 h-3 mr-2" />
                                        {t('Delete')}
                                    </DropdownMenuItem>
                                </DropdownMenuContent>
                            </DropdownMenu>
                        </div>
                    ))}

                    {settings.modes.length < 4 && (
                        <DropdownMenu>
                            <DropdownMenuTrigger asChild>
                                <button className="flex items-center cursor-pointer justify-center px-3 py-2 bg-zinc-900/30 text-zinc-500 hover:text-zinc-300 hover:bg-zinc-800 transition-colors">
                                    <Plus className="w-4 h-4" />
                                </button>
                            </DropdownMenuTrigger>
                            <DropdownMenuContent className="w-40 bg-zinc-900 border-zinc-700 text-zinc-300">
                                {getPresetTypes().map((preset) => (
                                    <DropdownMenuItem
                                        key={preset}
                                        className="focus:bg-zinc-800 focus:text-zinc-200 cursor-pointer"
                                        onClick={() => handleAddMode(preset)}
                                    >
                                        {t(getPresetLabel(preset))}
                                    </DropdownMenuItem>
                                ))}
                                <DropdownMenuItem
                                    className="cursor-pointer focus:bg-zinc-800 focus:text-zinc-200"
                                    onClick={() => handleAddMode()}
                                >
                                    {t('Custom')}
                                </DropdownMenuItem>
                            </DropdownMenuContent>
                        </DropdownMenu>
                    )}
                </div>

                {/* Active Mode Content */}
                {activeMode && (
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
                                                placeholder={t(
                                                    'Select a model'
                                                )}
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
                                        onClick={handleTestConnection} // Refresh models acts as test connection too or add dedicated refresh? user asked for test connection button restoring.
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
                                        <Typography.Title>
                                            {t('Prompt')}
                                        </Typography.Title>
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
                                            setPromptDraft(
                                                e.target.value.slice(0, 4000)
                                            )
                                        }
                                        maxLength={4000}
                                        className="w-full h-[600px] px-4 py-3 bg-zinc-900/50 border border-zinc-800 rounded-lg text-sm focus:outline-none focus:ring-1 focus:ring-sky-500/50 font-mono resize-y"
                                        placeholder={t(
                                            'Enter your prompt here...'
                                        )}
                                    />
                                    <div className="absolute bottom-3 right-3 flex flex-col gap-1 items-end pointer-events-none">
                                        <span className="text-[10px] text-zinc-500 mb-1">
                                            {promptDraft.length} / 4000
                                        </span>
                                    </div>
                                </div>
                            </SettingsUI.Item>
                        </SettingsUI.Container>

                        {/* Advanced Settings: URL & Test Connection */}
                        <SettingsUI.Container className="mb-6">
                            <SettingsUI.Item>
                                <SettingsUI.Description>
                                    <Typography.Title>
                                        {t('Ollama API URL')}
                                    </Typography.Title>
                                </SettingsUI.Description>
                                <div className="flex items-center gap-3">
                                    <Input
                                        value={settings.url}
                                        onChange={(e) =>
                                            updateSettings({
                                                url: e.target.value,
                                            })
                                        }
                                        className="w-[200px]"
                                        placeholder="http://localhost:11434/api"
                                    />
                                    <Button
                                        onClick={handleTestConnection}
                                        variant="outline"
                                        size="sm"
                                    >
                                        {t('Test Connection')}
                                    </Button>
                                </div>
                            </SettingsUI.Item>

                            <SettingsUI.Separator />

                            <SettingsUI.Item>
                                <SettingsUI.Description>
                                    <Typography.Title>
                                        {t('Tutorial')}
                                    </Typography.Title>
                                </SettingsUI.Description>

                                <div className="flex items-center gap-3">
                                    <Button
                                        onClick={() => setShowModelSelector(true)}
                                        variant="outline"
                                        size="sm"
                                    >
                                        {t('Install another model')}
                                    </Button>
                                    <Button
                                        onClick={handleResetOnboarding}
                                        variant="ghost"
                                        size="sm"
                                        className="text-zinc-500 hover:text-zinc-300"
                                    >
                                        {t('Reset Tutorial')}
                                    </Button>
                                </div>
                            </SettingsUI.Item>
                        </SettingsUI.Container>
                    </div>
                )}
            </div>
        </main>
    );
};
