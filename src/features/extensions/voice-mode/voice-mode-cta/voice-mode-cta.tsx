import { Lightbulb, Lock, Mic } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { PALETTES } from '@/components/lib/color-scheme';
import { ExtensionEnableAction } from '@/features/extensions/extension-enable-action/extension-enable-action';
import { VoicePreview } from '../voice-preview/voice-preview';

interface VoiceModeCtaProps {
    onEnable: () => void;
    recordWord: string;
    submitWord: string;
}

export const VoiceModeCta = ({ onEnable, recordWord, submitWord }: VoiceModeCtaProps) => {
    const { t } = useTranslation();

    return (
        <section data-testid="voice-mode-cta" className="flex flex-col items-center text-center gap-12 pt-6 pb-10">
            <ExtensionEnableAction
                icon={Mic}
                label={t('Enable Voice Mode')}
                onEnable={onEnable}
                testId="voice-mode-cta-enable"
                colorScheme="standard"
                hintIcon={Lock}
                hint={t('Listens on your device, nothing is saved.')}
            />

            <div className="flex w-full flex-col items-center gap-3">
                <VoicePreview recordWord={recordWord} submitWord={submitWord} />
                <div className="flex items-center gap-2 text-xs text-muted-foreground">
                    <Lightbulb className="w-3.5 h-3.5 shrink-0" style={{ color: PALETTES.standard.accent }} />
                    <span>{t('Works best in a quiet environment.')}</span>
                </div>
            </div>
        </section>
    );
};
