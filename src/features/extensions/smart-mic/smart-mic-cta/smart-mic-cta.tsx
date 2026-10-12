import { useState } from 'react';
import { Languages, Lock, Mic, Smartphone } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { Tabs } from '@/components/tabs';
import { PALETTES } from '@/components/lib/color-scheme';
import { ExtensionEnableAction } from '@/features/extensions/extension-enable-action/extension-enable-action';
import { SmartMicPreview, type SmartMicPreviewVariant } from '../smart-mic-preview/smart-mic-preview';

interface SmartMicCtaProps {
    onEnable: () => void;
    // When true, renders the "Enable Smart Mic" button in a disabled state and
    // ignores clicks. Used on Wayland where the feature cannot start; the rest
    // of the CTA (value cards, footer) stays identical so the user still sees
    // what Smart Mic offers.
    disabled?: boolean;
}

export const SmartMicCta = ({ onEnable, disabled = false }: SmartMicCtaProps) => {
    const { t } = useTranslation();
    const [variant, setVariant] = useState<SmartMicPreviewVariant>('mic');

    return (
        <section data-testid="smart-mic-cta" className="flex flex-col items-center text-center gap-12 pt-6 pb-10">
            <ExtensionEnableAction
                icon={Smartphone}
                label={t('Enable Smart Mic')}
                onEnable={onEnable}
                testId="smart-mic-cta-enable"
                colorScheme="standard"
                hintIcon={Lock}
                hint={t('On local WiFi only.')}
                disabled={disabled}
            />

            <div className="flex w-full flex-col items-center gap-3">
                <Tabs
                    value={variant}
                    onValueChange={setVariant}
                    tabs={[
                        { value: 'mic', label: t('Wireless mic'), icon: Mic },
                        { value: 'translation', label: t('Live translation'), icon: Languages },
                    ]}
                    activeColor={PALETTES.standard.accent}
                    className="w-full"
                >
                    <SmartMicPreview variant={variant} />
                </Tabs>
                {variant === 'translation' && (
                    <p className="text-xs text-muted-foreground">{t('Requires Prompt Mode')}</p>
                )}
            </div>
        </section>
    );
};
