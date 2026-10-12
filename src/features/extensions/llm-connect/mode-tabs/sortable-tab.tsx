import { useTranslation } from '@/i18n';
import clsx from 'clsx';
import { MoreVertical, Pencil, Trash2 } from 'lucide-react';
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '@/components/dropdown-menu';
import { TabTrigger } from '@/components/tab-trigger/tab-trigger';
import { LLMMode } from '../hooks/use-llm-connect';
import { useSortable } from '@dnd-kit/sortable';
import { CSS } from '@dnd-kit/utilities';

interface SortableTabProps {
    mode: LLMMode;
    index: number;
    isActive: boolean;
    onTabChange: (index: number) => void;
    onOpenRenameDialog: (index: number) => void;
    onDeleteMode: (index: number) => void;
    modesLength: number;
}

export const SortableTab = ({
    mode,
    index,
    isActive,
    onTabChange,
    onOpenRenameDialog,
    onDeleteMode,
    modesLength,
}: SortableTabProps) => {
    const { t } = useTranslation();
    const { attributes, listeners, setNodeRef, transform, isDragging, isSorting } = useSortable({ id: mode.name });

    const style = { transform: CSS.Transform.toString(transform) };

    if (isDragging) {
        return (
            <TabTrigger
                asChild
                isPlaceholder
                ref={setNodeRef}
                style={style}
                className={clsx('min-w-0', isSorting && 'transition-transform duration-200 ease-in-out')}
            >
                <div>
                    <span className="invisible truncate">{mode.name}</span>
                    <span className="invisible -my-0.5 shrink-0 border p-0.5">
                        <MoreVertical className="w-4 h-4" />
                    </span>
                </div>
            </TabTrigger>
        );
    }

    return (
        <TabTrigger
            ref={setNodeRef}
            style={style}
            {...attributes}
            {...listeners}
            onClick={() => onTabChange(index)}
            isActive={isActive}
            className={clsx(
                'group min-w-0 cursor-grab active:cursor-grabbing select-none',
                isSorting && 'transition-transform duration-200 ease-in-out',
                isActive && 'text-violet-300'
            )}
        >
            <span className="truncate">{mode.name}</span>
            <DropdownMenu>
                <DropdownMenuTrigger asChild>
                    <button
                        type="button"
                        className={clsx(
                            'opacity-0 group-hover:opacity-100 -my-0.5 shrink-0 p-0.5 rounded border border-transparent hover:border-border transition-all cursor-pointer',
                            isActive && 'opacity-100'
                        )}
                        onClick={(e) => e.stopPropagation()}
                        onKeyDown={(e) => e.stopPropagation()}
                        onPointerDown={(e) => e.stopPropagation()}
                    >
                        <MoreVertical className="w-4 h-4" />
                    </button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="start" className="w-40 bg-background border-border text-foreground">
                    <DropdownMenuItem
                        className="focus:bg-card focus:text-foreground"
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
                        className="text-red-400 focus:text-red-400 focus:bg-card"
                        disabled={modesLength <= 1}
                    >
                        <Trash2 className="w-3 h-3 mr-2" />
                        {t('Delete')}
                    </DropdownMenuItem>
                </DropdownMenuContent>
            </DropdownMenu>
        </TabTrigger>
    );
};
