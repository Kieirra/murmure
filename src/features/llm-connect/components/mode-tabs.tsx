import { useState, useCallback } from 'react';
import { useTranslation } from '@/i18n';
import { toast } from 'react-toastify';
import clsx from 'clsx';
import { Plus, MoreVertical, Pencil, Trash2 } from 'lucide-react';
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuTrigger,
} from '@/components/dropdown-menu';
import { Button } from '@/components/button';
import { Input } from '@/components/input';
import {
    Dialog,
    DialogContent,
    DialogFooter,
    DialogHeader,
    DialogTitle,
} from '@/components/dialog';
import {
    LLMConnectSettings,
    LLMMode,
    OllamaModel,
} from '../hooks/use-llm-connect';
import {
    getPresetLabel,
    getPresetTypes,
    getPromptByPreset,
} from '../llm-connect.helpers';
import { PromptPresetType } from '../llm-connect.constants';

interface ModeTabsProps {
    modes: LLMMode[];
    activeModeIndex: number;
    models: OllamaModel[];
    updateSettings: (updates: Partial<LLMConnectSettings>) => Promise<void>;
}

export const ModeTabs = ({
    modes,
    activeModeIndex,
    models,
    updateSettings,
}: ModeTabsProps) => {
    const { t, i18n } = useTranslation();
    const [renameDialogOpen, setRenameDialogOpen] = useState(false);
    const [modeToRename, setModeToRename] = useState<{
        index: number;
        name: string;
    } | null>(null);

    const activeMode = modes[activeModeIndex];

    const handleTabChange = useCallback(
        (index: number) => {
            updateSettings({ active_mode_index: index });
        },
        [updateSettings]
    );

    const handleAddMode = useCallback(
        (preset?: PromptPresetType) => {
            if (modes.length >= 4) return;

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
                    activeMode?.model ||
                    (models.length > 0 ? models[0].name : ''),
                shortcut: `Ctrl + Shift + ${modes.length + 1}`,
            };

            const newModes = [...modes, newMode];
            updateSettings({
                modes: newModes,
                active_mode_index: newModes.length - 1,
            });
        },
        [activeMode?.model, i18n.language, models, modes, t, updateSettings]
    );

    const handleDeleteMode = useCallback(
        (index: number) => {
            if (modes.length <= 1) {
                toast.error(t('Cannot delete the last mode'));
                return;
            }

            const newModes = modes.filter((_, i) => i !== index);

            let newIndex = activeModeIndex;
            if (index < newIndex) {
                newIndex = newIndex - 1;
            } else if (index === newIndex) {
                newIndex = Math.min(newIndex, newModes.length - 1);
            }

            const renamedModes = newModes.map((m, i) => ({
                ...m,
                shortcut: `Ctrl + Shift + ${i + 1}`,
            }));

            updateSettings({
                modes: renamedModes,
                active_mode_index: newIndex,
            });
        },
        [activeModeIndex, modes, t, updateSettings]
    );

    const openRenameDialog = (index: number) => {
        setModeToRename({ index, name: modes[index].name });
        setRenameDialogOpen(true);
    };

    const handleRenameSubmit = () => {
        if (modeToRename) {
            const newModes = [...modes];
            newModes[modeToRename.index] = {
                ...newModes[modeToRename.index],
                name: modeToRename.name,
            };
            updateSettings({ modes: newModes });
            setRenameDialogOpen(false);
            setModeToRename(null);
        }
    };

    return (
        <>
            <div className="flex flex-wrap border-zinc-800 px-1 mb-0">
                {modes.map((mode, index) => (
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
                        <span className="text-sm font-medium">{mode.name}</span>
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
                            <DropdownMenuContent
                                align="start"
                                className="w-40 bg-zinc-900 border-zinc-700 text-zinc-300"
                            >
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
                                    disabled={modes.length <= 1}
                                >
                                    <Trash2 className="w-3 h-3 mr-2" />
                                    {t('Delete')}
                                </DropdownMenuItem>
                            </DropdownMenuContent>
                        </DropdownMenu>
                    </div>
                ))}

                {modes.length < 4 && (
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
        </>
    );
};
