import { useState } from 'react';
import { Check, ChevronsUpDown, Plus } from 'lucide-react';
import { useTranslation } from '@/i18n';
import clsx from 'clsx';
import { Button } from '@/components/button';
import { Popover, PopoverContent, PopoverTrigger } from '@/components/popover';
import { Command, CommandGroup, CommandInput, CommandItem, CommandList } from '@/components/command';
import { OllamaModel } from '../hooks/use-llm-connect';
import { filterModels, shouldOfferCustomModel } from './model-combobox.helpers';

interface ModelComboboxProps {
    models: OllamaModel[];
    value: string;
    onValueChange: (model: string) => void;
    placeholder: string;
}

export const ModelCombobox = ({ models, value, onValueChange, placeholder }: ModelComboboxProps) => {
    const { t } = useTranslation();
    const [open, setOpen] = useState(false);
    const [query, setQuery] = useState('');

    const trimmedQuery = query.trim();
    const filteredModels = filterModels(models, query);
    const offerCustom = shouldOfferCustomModel(models, query);

    const handleOpenChange = (nextOpen: boolean) => {
        setOpen(nextOpen);
        setQuery('');
    };

    return (
        <Popover open={open} onOpenChange={handleOpenChange}>
            <PopoverTrigger asChild>
                <Button
                    variant="outline"
                    role="combobox"
                    aria-expanded={open}
                    className="w-[300px] justify-between font-normal dark:bg-black/30 dark:hover:bg-black/50"
                >
                    <span className="truncate">{value || placeholder}</span>
                    <ChevronsUpDown className="h-4 w-4 shrink-0 opacity-50" />
                </Button>
            </PopoverTrigger>
            <PopoverContent className="w-[300px] p-0" align="start" sideOffset={4}>
                <Command shouldFilter={false}>
                    <CommandInput
                        value={query}
                        onValueChange={setQuery}
                        placeholder={t('Search or type a model name')}
                        aria-label={t('Model name')}
                    />
                    <CommandList>
                        {offerCustom && (
                            <CommandGroup>
                                <CommandItem
                                    value={trimmedQuery}
                                    className="cursor-pointer"
                                    onSelect={() => {
                                        onValueChange(trimmedQuery);
                                        setOpen(false);
                                    }}
                                >
                                    <Plus className="mr-2 h-4 w-4" />
                                    <span className="truncate">{t('Use "{{model}}"', { model: trimmedQuery })}</span>
                                </CommandItem>
                            </CommandGroup>
                        )}
                        <CommandGroup>
                            {filteredModels.map((model) => (
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
                        {filteredModels.length === 0 && !offerCustom && (
                            <div className="px-3 py-6 text-center text-sm">
                                {t('Type the exact model name, for example claude-haiku-4-5.')}
                            </div>
                        )}
                    </CommandList>
                </Command>
            </PopoverContent>
        </Popover>
    );
};
