import { useTranslation } from '@/i18n';
import clsx from 'clsx';
import { MoreVertical, Pencil, Trash2 } from 'lucide-react';
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuTrigger,
} from '@/components/dropdown-menu';
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
    const {
        attributes,
        listeners,
        setNodeRef,
        transform,
        isDragging,
        isSorting,
    } = useSortable({ id: mode.name });

    const style = { transform: CSS.Transform.toString(transform) };

    if (isDragging) {
        return (
            <div
                ref={setNodeRef}
                style={style}
                className={clsx(
                    'border border-dashed border-zinc-700 rounded bg-zinc-800/10 px-4 py-2 min-w-[80px] h-[40px]',
                    isSorting && 'transition-transform duration-200 ease-in-out'
                )}
            />
        );
    }

    return (
        <button
            ref={setNodeRef}
            style={style}
            {...attributes}
            {...listeners}
            onClick={() => onTabChange(index)}
            className={clsx(
                'group relative flex items-center gap-2 px-4 py-2 transition-colors cursor-grab active:cursor-grabbing select-none',
                isSorting && 'transition-transform duration-200 ease-in-out',
                isActive
                    ? 'bg-zinc-800/80 text-sky-400 border-b-2 border-sky-500'
                    : 'bg-zinc-900/50 text-zinc-400 hover:bg-zinc-800 hover:text-zinc-200'
            )}
        >
            <span className="text-sm font-medium">{mode.name}</span>
            <DropdownMenu>
                <DropdownMenuTrigger asChild>
                    <button
                        type="button"
                        className={clsx(
                            'opacity-0 group-hover:opacity-100 p-1 rounded hover:bg-zinc-700 transition-all cursor-pointer',
                            isActive && 'opacity-100'
                        )}
                        onClick={(e) => e.stopPropagation()}
                        onKeyDown={(e) => e.stopPropagation()}
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
        </button>
    );
};
