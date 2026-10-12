import { useState } from 'react';
import { Check, ChevronsUpDown, Plus } from 'lucide-react';
import { useTranslation } from '@/i18n';
import clsx from 'clsx';
import { Popover, PopoverContent, PopoverTrigger } from '@/components/popover';
import { Command, CommandGroup, CommandInput, CommandItem, CommandList } from '@/components/command';
import { OllamaModel } from '../hooks/use-llm-connect';
import { filterModels, shouldOfferCustomModel } from './model-combobox.helpers';

const COMMAND_ITEM_CLASS = 'cursor-pointer data-[selected=true]:bg-sky-500/15 data-[selected=true]:text-sky-200';

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
                <button
                    type="button"
                    aria-expanded={open}
                    className="flex h-9 w-[300px] cursor-pointer items-center justify-between gap-2 whitespace-nowrap rounded-md border border-input bg-field px-3 py-2 text-sm shadow-sm focus:outline-none focus-visible:border-sky-500 focus-visible:ring-[3px] focus-visible:ring-sky-500/25 data-[state=open]:border-sky-500"
                >
                    <span className={clsx('truncate', value.length === 0 && 'text-muted-foreground')}>
                        {value || placeholder}
                    </span>
                    <ChevronsUpDown className="h-4 w-4 shrink-0 opacity-50" />
                </button>
            </PopoverTrigger>
            <PopoverContent className="w-[300px] border-input bg-field p-0" align="start" sideOffset={4}>
                <Command shouldFilter={false} className="bg-field">
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
                                    className={COMMAND_ITEM_CLASS}
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
                                    className={COMMAND_ITEM_CLASS}
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
