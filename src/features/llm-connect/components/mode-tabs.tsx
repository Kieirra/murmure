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
import { Page } from '@/components/page';
import {
    DndContext,
    closestCenter,
    DragEndEvent,
    DragStartEvent,
    DragOverlay,
    PointerSensor,
    KeyboardSensor,
    useSensor,
    useSensors,
} from '@dnd-kit/core';
import {
    SortableContext,
    horizontalListSortingStrategy,
    useSortable,
    sortableKeyboardCoordinates,
} from '@dnd-kit/sortable';
import { CSS } from '@dnd-kit/utilities';
import { restrictToHorizontalAxis } from '@dnd-kit/modifiers';

interface SortableTabProps {
    mode: LLMMode;
    index: number;
    isActive: boolean;
    onTabChange: (index: number) => void;
    onOpenRenameDialog: (index: number) => void;
    onDeleteMode: (index: number) => void;
    modesLength: number;
}

const SortableTab = ({
    mode,
    index,
    isActive,
    onTabChange,
    onOpenRenameDialog,
    onDeleteMode,
    modesLength,
}: SortableTabProps) => {
    const { t } = useTranslation();
    const {
        attributes,
        listeners,
        setNodeRef,
        transform,
        transition,
        isDragging,
    } = useSortable({ id: mode.name });

    const style = {
        transform: CSS.Transform.toString(transform),
        transition,
    };

    if (isDragging) {
        return (
            <div
                ref={setNodeRef}
                style={style}
                className="border border-dashed border-zinc-700 rounded bg-zinc-800/10 px-4 py-2 min-w-[80px] h-[40px]"
            />
        );
    }

    return (
        <div
            ref={setNodeRef}
            style={style}
            {...attributes}
            {...listeners}
            onClick={() => onTabChange(index)}
            className={clsx(
                'group relative flex items-center gap-2 px-4 py-2 transition-colors cursor-grab active:cursor-grabbing select-none',
                isActive
                    ? 'bg-zinc-800/80 text-sky-400 border-b-2 border-sky-500'
                    : 'bg-zinc-900/50 text-zinc-400 hover:bg-zinc-800 hover:text-zinc-200'
            )}
        >
            <span className="text-sm font-medium">{mode.name}</span>
            <DropdownMenu>
                <DropdownMenuTrigger asChild>
                    <button
                        className={clsx(
                            'opacity-0 group-hover:opacity-100 p-1 rounded hover:bg-zinc-700 transition-all',
                            isActive && 'opacity-100'
                        )}
                        onClick={(e) => e.stopPropagation()}
                        onPointerDown={(e) => e.stopPropagation()}
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
                            onOpenRenameDialog(index);
                        }}
                    >
                        <Pencil className="w-3 h-3 mr-2" />
                        {t('Rename')}
                    </DropdownMenuItem>
                    <DropdownMenuItem
                        onClick={(e) => {
                            e.stopPropagation();
                            onDeleteMode(index);
                        }}
                        className="text-red-400 focus:text-red-400 focus:bg-zinc-800"
                        disabled={modesLength <= 1}
                    >
                        <Trash2 className="w-3 h-3 mr-2" />
                        {t('Delete')}
                    </DropdownMenuItem>
                </DropdownMenuContent>
            </DropdownMenu>
        </div>
    );
};

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
    const [activeId, setActiveId] = useState<string | null>(null);

    const activeMode = modes[activeModeIndex];

    const sensors = useSensors(
        useSensor(PointerSensor, {
            activationConstraint: { distance: 8 },
        }),
        useSensor(KeyboardSensor, {
            coordinateGetter: sortableKeyboardCoordinates,
        })
    );

    const handleDragStart = (event: DragStartEvent) => {
        setActiveId(String(event.active.id));
    };

    const handleDragEnd = (event: DragEndEvent) => {
        setActiveId(null);
        const { active, over } = event;
        if (over == null || active.id === over.id) return;

        const oldIndex = modes.findIndex((m) => m.name === String(active.id));
        const newIndex = modes.findIndex((m) => m.name === String(over.id));
        if (oldIndex === -1 || newIndex === -1) return;

        const newModes = [...modes];
        const [moved] = newModes.splice(oldIndex, 1);
        newModes.splice(newIndex, 0, moved);

        const reindexedModes = newModes.map((m, i) => ({
            ...m,
            shortcut: `Ctrl + Shift + ${i + 1}`,
        }));

        const currentActiveName = modes[activeModeIndex].name;
        const newActiveModeIndex = reindexedModes.findIndex(
            (m) => m.name === currentActiveName
        );

        updateSettings({
            modes: reindexedModes,
            active_mode_index: newActiveModeIndex,
        });
    };

    const draggedMode =
        activeId != null
            ? modes.find((m) => m.name === activeId)
            : undefined;

    const handleTabChange = useCallback(
        (index: number) => {
            updateSettings({ active_mode_index: index });
        },
        [updateSettings]
    );

    const handleAddMode = useCallback(
        (preset?: PromptPresetType) => {
            if (modes.length >= 4) return;

            let baseName = t('New Mode');
            let prompt = '';
            if (preset) {
                baseName = t(getPresetLabel(preset));
                prompt = getPromptByPreset(preset, i18n.language);
            }

            let name = baseName;
            let counter = 1;
            while (modes.some((m) => m.name === name)) {
                name = `${baseName} (${counter})`;
                counter++;
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
            const nameExists = modes.some(
                (m, i) =>
                    i !== modeToRename.index && m.name === modeToRename.name
            );

            if (nameExists) {
                toast.error(t('Mode name already exists'));
                return;
            }

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
            <DndContext
                sensors={sensors}
                collisionDetection={closestCenter}
                onDragStart={handleDragStart}
                onDragEnd={handleDragEnd}
                modifiers={[restrictToHorizontalAxis]}
            >
                <div className="flex flex-wrap border-zinc-800 px-1 mb-0">
                    <SortableContext
                        items={modes.map((m) => m.name)}
                        strategy={horizontalListSortingStrategy}
                    >
                        {modes.map((mode, index) => (
                            <SortableTab
                                key={mode.name}
                                mode={mode}
                                index={index}
                                isActive={activeModeIndex === index}
                                onTabChange={handleTabChange}
                                onOpenRenameDialog={openRenameDialog}
                                onDeleteMode={handleDeleteMode}
                                modesLength={modes.length}
                            />
                        ))}
                    </SortableContext>

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

                <DragOverlay>
                    {draggedMode != null && (
                        <div
                            className={clsx(
                                'flex items-center gap-2 px-4 py-2 select-none',
                                modes[activeModeIndex]?.name ===
                                    draggedMode.name
                                    ? 'bg-zinc-800/80 text-sky-400 border-b-2 border-sky-500'
                                    : 'bg-zinc-900/50 text-zinc-400'
                            )}
                        >
                            <span className="text-sm font-medium">
                                {draggedMode.name}
                            </span>
                            <div className="p-1">
                                <MoreVertical className="w-4 h-4" />
                            </div>
                        </div>
                    )}
                </DragOverlay>
            </DndContext>

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
                    <DialogFooter className="dark">
                        <Page.SecondaryButton
                            variant="ghost"
                            onClick={() => setRenameDialogOpen(false)}
                        >
                            {t('Cancel')}
                        </Page.SecondaryButton>
                        <Page.SecondaryButton onClick={handleRenameSubmit}>
                            {t('Save')}
                        </Page.SecondaryButton>
                    </DialogFooter>
                </DialogContent>
            </Dialog>
        </>
    );
};
