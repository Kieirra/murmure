import { Check, Mic } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { PALETTES } from '@/features/home/audio-visualizer/audio-pixel/audio-pixel.helpers';
import { PreviewCell } from '@/features/extensions/extension-demo/preview-cell/preview-cell';
import { PreviewArrow } from '@/features/extensions/extension-demo/preview-arrow/preview-arrow';

interface VoicePreviewProps {
    recordWord: string;
    submitWord: string;
}

export const VoicePreview = ({ recordWord, submitWord }: VoicePreviewProps) => {
    const { t } = useTranslation();
    const accentColor = PALETTES.standard.accent;
    const sampleText = t('See you at noon.');

    return (
        <div className="w-full rounded-md border border-surface-border bg-surface-header p-4">
            <div className="grid grid-cols-1 gap-4 text-left lg:grid-cols-[1fr_auto_1fr_auto_1fr_auto_1fr]">
                <PreviewCell label={t('You say')} centered>
                    <span className="text-sm italic text-foreground">{t('"{{text}}"', { text: recordWord })}</span>
                </PreviewCell>

                <PreviewArrow horizontalFrom="lg" />

                <PreviewCell label={t('You dictate')} centered>
                    <div className="flex items-center gap-2">
                        <Mic className="w-4 h-4 shrink-0 motion-safe:animate-pulse" style={{ color: accentColor }} />
                        <span className="text-sm italic text-foreground">{t('"{{text}}"', { text: sampleText })}</span>
                    </div>
                </PreviewCell>

                <PreviewArrow horizontalFrom="lg" />

                <PreviewCell label={t('You say')} centered>
                    <span className="text-sm italic text-foreground">{t('"{{text}}"', { text: submitWord })}</span>
                </PreviewCell>

                <PreviewArrow horizontalFrom="lg" />

                <PreviewCell label={t('Sent')} labelStyle={{ color: accentColor }} centered>
                    <div className="flex items-center gap-2">
                        <Check className="w-4 h-4 shrink-0" style={{ color: accentColor }} />
                        <span className="text-sm text-foreground">{sampleText}</span>
                    </div>
                </PreviewCell>
            </div>
        </div>
    );
};
