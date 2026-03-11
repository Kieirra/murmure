import { useState } from 'react';
import { Check, ChevronsUpDown } from 'lucide-react';
import { useTranslation } from '@/i18n';
import clsx from 'clsx';
import { Button } from '@/components/button';
import { Popover, PopoverContent, PopoverTrigger } from '@/components/popover';
import { Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList } from '@/components/command';
import { OllamaModel } from '../hooks/use-llm-connect';

interface ModelComboboxProps {
    models: OllamaModel[];
    value: string;
    onValueChange: (model: string) => void;
    disabled: boolean;
    placeholder: string;
}

export const ModelCombobox = ({ models, value, onValueChange, disabled, placeholder }: ModelComboboxProps) => {
    const { t } = useTranslation();
    const [open, setOpen] = useState(false);

    return (
        <Popover open={open} onOpenChange={setOpen}>
            <PopoverTrigger asChild>
                <Button
                    variant="outline"
                    role="combobox"
                    aria-expanded={open}
                    disabled={disabled}
                    className="w-[300px] justify-between font-normal"
                >
                    <span className="truncate">{value || placeholder}</span>
                    <ChevronsUpDown className="h-4 w-4 shrink-0 opacity-50" />
                </Button>
            </PopoverTrigger>
            <PopoverContent className="w-[300px] p-0" align="start" sideOffset={4}>
                <Command>
                    <CommandInput placeholder={t('Search models...')} />
                    <CommandList>
                        <CommandEmpty>{t('No model found.')}</CommandEmpty>
                        <CommandGroup>
                            {models.map((model) => (
                                <CommandItem
                                    key={model.name}
                                    value={model.name}
                                    onSelect={() => {
                                        onValueChange(model.name);
                                        setOpen(false);
                                    }}
                                >
                                    <Check
                                        className={clsx(
                                            'mr-2 h-4 w-4',
                                            value === model.name ? 'opacity-100' : 'opacity-0'
                                        )}
                                    />
                                    {model.name}
                                </CommandItem>
                            ))}
                        </CommandGroup>
                    </CommandList>
                </Command>
            </PopoverContent>
        </Popover>
    );
};
