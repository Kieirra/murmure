import { useTranslation } from '@/i18n';
import {
    Dialog,
    DialogContent,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from '@/components/dialog';
import { Button } from '@/components/button';
import {
    getPresetDescription,
    getPresetLabel,
    getPresetTypes,
    getPromptByPreset,
} from '../llm-connect.helpers';
import { PromptPresetType } from '../llm-connect.constants';
import { useState } from 'react';

interface PresetSelectorProps {
    onSelect: (prompt: string) => void;
}

export const PresetSelector = ({ onSelect }: PresetSelectorProps) => {
    const { t, i18n } = useTranslation();
    const [open, setOpen] = useState(false);

    const handleSelect = (preset: PromptPresetType) => {
        onSelect(getPromptByPreset(preset, i18n.language));
        setOpen(false);
    };

    return (
        <Dialog open={open} onOpenChange={setOpen}>
            <DialogTrigger asChild>
                <Button variant="outline">{t('Import a preset')}</Button>
            </DialogTrigger>
            <DialogContent className="sm:max-w-[600px]">
                <DialogHeader>
                    <DialogTitle>{t('Select a preset')}</DialogTitle>
                </DialogHeader>
                <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 py-4">
                    {getPresetTypes().map((preset) => (
                        <button
                            key={preset}
                            className="w-full text-left flex flex-col gap-3 p-4 rounded-lg border border-zinc-700 bg-zinc-800 hover:bg-zinc-700 transition-colors cursor-pointer group"
                            onClick={() => handleSelect(preset)}
                        >
                            <div className="flex flex-col gap-1">
                                <span className="font-medium text-zinc-100">
                                    {t(getPresetLabel(preset))}
                                </span>
                                <span className="text-xs text-zinc-400 leading-relaxed">
                                    {t(getPresetDescription(preset))}
                                </span>
                            </div>
                            <Button
                                variant="outline"
                                size="sm"
                                className="w-full mt-auto bg-zinc-900 border-zinc-700 hover:bg-zinc-800 text-zinc-300 hover:text-white"
                            >
                                {t('Select')}
                            </Button>
                        </button>
                    ))}
                </div>
            </DialogContent>
        </Dialog>
    );
};
