import { useState } from 'react';
import { Highlighter, MessageCircleQuestionMark, Zap } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { Tabs } from '@/components/tabs';
import { PALETTES } from '@/features/home/audio-visualizer/audio-pixel/audio-pixel.helpers';
import { CommandPreview, type CommandPreviewVariant } from '../command-preview/command-preview';
import { LLMEnableFooter } from '@/features/extensions/llm-connect/llm-enable-footer/llm-enable-footer';

interface CommandModeCtaProps {
    onEnable: () => void;
    needsSetup: boolean;
}

export const CommandModeCta = ({ onEnable, needsSetup }: CommandModeCtaProps) => {
    const { t } = useTranslation();
    const [variant, setVariant] = useState<CommandPreviewVariant>('selection');

    return (
        <section data-testid="command-mode-cta" className="flex flex-col items-center text-center gap-12 pt-6 pb-10">
            <LLMEnableFooter
                icon={Zap}
                label={needsSetup ? t('Set up Command Mode') : t('Enable Command Mode')}
                needsSetup={needsSetup}
                onEnable={onEnable}
                testId="command-mode-cta-enable"
                colorScheme="command"
            />

            <Tabs
                value={variant}
                onValueChange={setVariant}
                tabs={[
                    { value: 'selection', label: t('With selected text'), icon: Highlighter },
                    { value: 'question', label: t('Without selection'), icon: MessageCircleQuestionMark },
                ]}
                activeColor={PALETTES.command.accent}
                className="w-full"
            >
                <CommandPreview variant={variant} />
            </Tabs>
        </section>
    );
};
