import { AlignLeft, Settings2 } from 'lucide-react';
import { Page } from '@/components/page';
import { Typography } from '@/components/typography';
import { Switch } from '@/components/switch';
import { useTranslation } from '@/i18n';
import { useFormattingRules } from './hooks/use-formatting-rules';
import { RuleCard } from '../../../components/rule-card';
import { AddRuleSection } from '../../../components/add-rule-section';

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
    } = useFormattingRules();

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
        <main className="space-y-8">
            <Page.Header>
                <Typography.MainTitle data-testid="formatting-rules-title">
                    {t('Formatting Rules')}
                </Typography.MainTitle>
                <Typography.Paragraph className="text-zinc-400">
                    {t(
                        'Define rules to automatically clean, format, and enhance your transcriptions. These rules apply to all new transcriptions.'
                    )}
                </Typography.Paragraph>
            </Page.Header>

            {/* Built-in Options Section */}
            <div className="space-y-4">
                <div className="flex items-center gap-2">
                    <Settings2 className="w-4 h-4 text-zinc-400" />
                    <Typography.Title>{t('Built-in Options')}</Typography.Title>
                </div>

                <div className="space-y-3">
                    {/* Space before punctuation option */}
                    <div className="flex items-center justify-between p-4 bg-zinc-800/50 border border-zinc-700 rounded-lg">
                        <div className="flex-1">
                            <p className="text-sm font-medium text-white">
                                {t('Add space before ? and !')}
                            </p>
                            <p className="text-xs text-zinc-400 mt-1">
                                {t(
                                    'Automatically adds a space before question marks and exclamation points if missing. Example: "Hello?" → "Hello ?"'
                                )}
                            </p>
                        </div>
                        <Switch
                            checked={settings.built_in.space_before_punctuation}
                            onCheckedChange={(checked) =>
                                updateBuiltInOption(
                                    'space_before_punctuation',
                                    checked
                                )
                            }
                            data-testid="option-space-before-punctuation"
                        />
                    </div>

                    {/* Trailing space option */}
                    <div className="flex items-center justify-between p-4 bg-zinc-800/50 border border-zinc-700 rounded-lg">
                        <div className="flex-1">
                            <p className="text-sm font-medium text-white">
                                {t('Add space at end of transcription')}
                            </p>
                            <p className="text-xs text-zinc-400 mt-1">
                                {t(
                                    'Ensures each transcription ends with a space. Prevents consecutive transcriptions from "sticking" together.'
                                )}
                            </p>
                        </div>
                        <Switch
                            checked={settings.built_in.trailing_space}
                            onCheckedChange={(checked) =>
                                updateBuiltInOption('trailing_space', checked)
                            }
                            data-testid="option-trailing-space"
                        />
                    </div>
                </div>
            </div>

            {/* Custom Rules Section */}
            <div className="space-y-4">
                <div className="flex items-center gap-2">
                    <AlignLeft className="w-4 h-4 text-zinc-400" />
                    <Typography.Title>
                        {t('Custom Formatting Rules')}
                    </Typography.Title>
                </div>

                <Typography.Paragraph>
                    {t(
                        'Create find-and-replace rules to transform specific words or phrases in your transcriptions.'
                    )}
                </Typography.Paragraph>

                {/* Existing rules */}
                {settings.rules.length > 0 && (
                    <div className="space-y-3">
                        {settings.rules.map((rule) => (
                            <RuleCard
                                key={rule.id}
                                rule={rule}
                                onUpdate={updateRule}
                                onDelete={deleteRule}
                                onDuplicate={duplicateRule}
                            />
                        ))}
                    </div>
                )}

                {/* Add new rule section */}
                <AddRuleSection onAdd={addRule} />
            </div>
        </main>
    );
};
