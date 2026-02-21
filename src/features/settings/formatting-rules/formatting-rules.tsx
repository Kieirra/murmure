import { useState } from 'react';
import { Page } from '@/components/page';
import { Typography } from '@/components/typography';
import { Switch } from '@/components/switch';
import { useTranslation } from '@/i18n';
import { useFormattingRules } from './hooks/use-formatting-rules';
import { RuleCard } from '../../../components/rule-card';
import { AddRuleSection } from '../../../components/add-rule-section';
import {
    Select,
    SelectTrigger,
    SelectValue,
    SelectContent,
    SelectItem,
} from '../../../components/select';
import { NumberInput } from '@/components/number-input';
import { SettingsUI } from '@/components/settings-ui';
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
    verticalListSortingStrategy,
    useSortable,
    sortableKeyboardCoordinates,
} from '@dnd-kit/sortable';
import { CSS } from '@dnd-kit/utilities';
import { restrictToVerticalAxis } from '@dnd-kit/modifiers';
import { FormattingRule } from './types';

interface SortableRuleCardProps {
    rule: FormattingRule;
    onUpdate: (id: string, updates: Partial<Omit<FormattingRule, 'id'>>) => void;
    onDelete: (id: string) => void;
    onDuplicate: (id: string) => void;
}

const SortableRuleCard = ({ rule, ...props }: SortableRuleCardProps) => {
    const {
        attributes,
        listeners,
        setNodeRef,
        transform,
        transition,
        isDragging,
    } = useSortable({ id: rule.id });

    const style = {
        transform: CSS.Transform.toString(transform),
        transition,
    };

    return (
        <div ref={setNodeRef} style={style}>
            {isDragging ? (
                <div className="border border-dashed border-zinc-700 rounded-lg h-[56px] bg-zinc-800/10" />
            ) : (
                <RuleCard
                    rule={rule}
                    {...props}
                    dragHandleProps={{ ...attributes, ...listeners }}
                />
            )}
        </div>
    );
};

export const FormattingRules = () => {
    const { t } = useTranslation();
    const {
        settings,
        isLoading,
        updateBuiltInOption,
        addRule,
        updateRule,
        deleteRule,
        duplicateRule,
        reorderRules,
    } = useFormattingRules();

    const [activeId, setActiveId] = useState<string | null>(null);

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
        if (over != null && active.id !== over.id) {
            reorderRules(String(active.id), String(over.id));
        }
    };

    const activeRule = activeId != null
        ? settings.rules.find((rule) => rule.id === activeId)
        : undefined;

    if (isLoading) {
        return (
            <main className="space-y-8">
                <Page.Header>
                    <Typography.MainTitle data-testid="formatting-rules-title">
                        {t('Formatting Rules')}
                    </Typography.MainTitle>
                </Page.Header>
                <div className="text-zinc-400">{t('Loading...')}</div>
            </main>
        );
    }

    return (
        <main className="space-y-6">
            <Page.Header>
                <Typography.MainTitle data-testid="formatting-rules-title">
                    {t('Formatting Rules')}
                </Typography.MainTitle>
                <Typography.Paragraph className="text-zinc-400">
                    {t('Clean automatically your transcriptions')}
                </Typography.Paragraph>
            </Page.Header>

            <div>
                <div className="space-y-3">
                    <SettingsUI.Container>
                        <SettingsUI.Item>
                            <SettingsUI.Description className="w-[600px]">
                                <Typography.Title>
                                    {t('Short text correction')}
                                </Typography.Title>
                                <Typography.Paragraph>
                                    {t(
                                        'Automatically removes capitalization and trailing punctuation for short transcriptions (1-2 words). Useful when correcting a single word mid-sentence.'
                                    )}
                                    <br />
                                    <span className="text-xs italic text-zinc-500">
                                        {t(
                                            'Example: "Hello." → "hello"'
                                        )}
                                    </span>
                                </Typography.Paragraph>
                            </SettingsUI.Description>
                            <Switch
                                checked={
                                    settings.built_in.short_text_correction
                                }
                                onCheckedChange={(checked) =>
                                    updateBuiltInOption(
                                        'short_text_correction',
                                        checked
                                    )
                                }
                                data-testid="option-short-text-correction"
                            />
                        </SettingsUI.Item>
                    </SettingsUI.Container>

                    <SettingsUI.Container>
                        <SettingsUI.Item>
                            <SettingsUI.Description className="w-[600px]">
                                <Typography.Title>
                                    {t('Add space before ? and !')}
                                </Typography.Title>
                                <Typography.Paragraph>
                                    {t(
                                        'Automatically adds a space before question marks and exclamation points if missing.'
                                    )}
                                    <br />
                                    <span className="text-xs italic text-zinc-500">
                                        {t('Example: "Hello?" → "Hello ?"')}
                                    </span>
                                </Typography.Paragraph>
                            </SettingsUI.Description>
                            <Switch
                                checked={
                                    settings.built_in.space_before_punctuation
                                }
                                onCheckedChange={(checked) =>
                                    updateBuiltInOption(
                                        'space_before_punctuation',
                                        checked
                                    )
                                }
                                data-testid="option-space-before-punctuation"
                            />
                        </SettingsUI.Item>
                    </SettingsUI.Container>

                    <SettingsUI.Container>
                        <SettingsUI.Item>
                            <SettingsUI.Description className="w-[600px]">
                                <Typography.Title>
                                    {t('Add space at end of transcription')}
                                </Typography.Title>
                                <Typography.Paragraph>
                                    {t(
                                        'Ensures each transcription ends with a space. Prevents consecutive transcriptions from "sticking" together.'
                                    )}
                                </Typography.Paragraph>
                            </SettingsUI.Description>
                            <Switch
                                checked={settings.built_in.trailing_space}
                                onCheckedChange={(checked) =>
                                    updateBuiltInOption(
                                        'trailing_space',
                                        checked
                                    )
                                }
                                data-testid="option-trailing-space"
                            />
                        </SettingsUI.Item>
                    </SettingsUI.Container>

                    <SettingsUI.Container>
                        <SettingsUI.Item>
                            <SettingsUI.Description className="w-[600px]">
                                <Typography.Title>
                                    {t('Convert text numbers to digits')}
                                </Typography.Title>
                                <Typography.Paragraph>
                                    {t(
                                        'Automatically converts numbers written in letters to digits.'
                                    )}
                                    <br />
                                    <span className="text-xs italic text-zinc-500">
                                        {t(
                                            'Example: "one" → "1", "twenty-three" → "23"'
                                        )}
                                    </span>
                                </Typography.Paragraph>
                            </SettingsUI.Description>
                            <Switch
                                checked={settings.built_in.convert_text_numbers}
                                onCheckedChange={(checked) =>
                                    updateBuiltInOption(
                                        'convert_text_numbers',
                                        checked
                                    )
                                }
                                data-testid="option-convert-text-numbers"
                            />
                        </SettingsUI.Item>
                        {settings.built_in.convert_text_numbers && (
                            <>
                                <SettingsUI.Separator />
                                <SettingsUI.Item>
                                    <SettingsUI.Description className="flex-1">
                                        <Typography.Title>
                                            {t(
                                                'Language for number conversion'
                                            )}
                                        </Typography.Title>
                                        <Typography.Paragraph>
                                            {t(
                                                'Choose the language for text-to-number conversion'
                                            )}
                                        </Typography.Paragraph>
                                    </SettingsUI.Description>
                                    <Select
                                        value={
                                            settings.built_in
                                                .text_numbers_language
                                        }
                                        onValueChange={(value) =>
                                            updateBuiltInOption(
                                                'text_numbers_language',
                                                value
                                            )
                                        }
                                    >
                                        <SelectTrigger className="w-[180px]">
                                            <SelectValue
                                                placeholder={t(
                                                    'Select language'
                                                )}
                                            />
                                        </SelectTrigger>
                                        <SelectContent>
                                            <SelectItem value="en">
                                                English
                                            </SelectItem>
                                            <SelectItem value="fr">
                                                Français
                                            </SelectItem>
                                            <SelectItem value="de">
                                                Deutsch
                                            </SelectItem>
                                            <SelectItem value="it">
                                                Italiano
                                            </SelectItem>
                                            <SelectItem value="es">
                                                Español
                                            </SelectItem>
                                            <SelectItem value="nl">
                                                Nederlands
                                            </SelectItem>
                                            <SelectItem value="pt">
                                                Português
                                            </SelectItem>
                                        </SelectContent>
                                    </Select>
                                </SettingsUI.Item>
                                <SettingsUI.Separator />
                                <SettingsUI.Item>
                                    <SettingsUI.Description className="w-[600px]">
                                        <Typography.Title>
                                            {t('Conversion threshold')}
                                        </Typography.Title>
                                        <Typography.Paragraph>
                                            {t(
                                                'Do not convert numbers that are strictly below this threshold.'
                                            )}
                                        </Typography.Paragraph>
                                    </SettingsUI.Description>
                                    <div className="w-[120px]">
                                        <NumberInput
                                            value={
                                                settings.built_in
                                                    .text_numbers_threshold
                                            }
                                            onValueChange={(value) =>
                                                updateBuiltInOption(
                                                    'text_numbers_threshold',
                                                    value ?? 10
                                                )
                                            }
                                            min={0}
                                            max={50}
                                        />
                                    </div>
                                </SettingsUI.Item>
                            </>
                        )}
                    </SettingsUI.Container>
                </div>
            </div>

            <hr />

            <div className="space-y-4">
                {settings.rules.length > 0 && (
                    <DndContext
                        sensors={sensors}
                        collisionDetection={closestCenter}
                        onDragStart={handleDragStart}
                        onDragEnd={handleDragEnd}
                        modifiers={[restrictToVerticalAxis]}
                    >
                        <SortableContext
                            items={settings.rules.map((rule) => rule.id)}
                            strategy={verticalListSortingStrategy}
                        >
                            <div className="space-y-3">
                                {settings.rules.map((rule) => (
                                    <SortableRuleCard
                                        key={rule.id}
                                        rule={rule}
                                        onUpdate={updateRule}
                                        onDelete={deleteRule}
                                        onDuplicate={duplicateRule}
                                    />
                                ))}
                            </div>
                        </SortableContext>
                        <DragOverlay>
                            {activeRule != null ? (
                                <RuleCard
                                    rule={activeRule}
                                    onUpdate={updateRule}
                                    onDelete={deleteRule}
                                    onDuplicate={duplicateRule}
                                />
                            ) : null}
                        </DragOverlay>
                    </DndContext>
                )}
                <AddRuleSection onAdd={addRule} />
                <div className="h-8" />
            </div>
        </main>
    );
};
