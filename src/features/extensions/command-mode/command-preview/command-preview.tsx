import { Mic } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { getResultPanelPalette } from '@/overlay/result-panel/result-panel.helpers';
import { PreviewCell } from '@/features/extensions/extension-demo/preview-cell/preview-cell';
import { PreviewArrow } from '@/features/extensions/extension-demo/preview-arrow/preview-arrow';
import { ResultCardPreview } from '../result-card-preview/result-card-preview';
import { buildSelectionHighlight } from './command-preview.helpers';

export type CommandPreviewVariant = 'selection' | 'question';

interface CommandPreviewProps {
    variant: CommandPreviewVariant;
}

export const CommandPreview = ({ variant }: CommandPreviewProps) => {
    const { t } = useTranslation();
    const palette = getResultPanelPalette('command');
    const isSelection = variant === 'selection';

    return (
        <div className="grid grid-cols-1 gap-4 text-left md:grid-cols-[1fr_auto_1fr_auto_1fr]">
            {isSelection && (
                <>
                    <PreviewCell label={t('Selected text')} centered>
                        <p className="text-sm leading-relaxed">
                            <span
                                className="text-foreground"
                                style={{ backgroundColor: buildSelectionHighlight(palette.center) }}
                            >
                                {t('Send me the file now.')}
                            </span>
                        </p>
                    </PreviewCell>

                    <PreviewArrow />
                </>
            )}

            <PreviewCell label={t('You say')} centered>
                <div className="flex items-center gap-2">
                    <Mic className="w-4 h-4 shrink-0 motion-safe:animate-pulse" style={{ color: palette.accent }} />
                    <span className="text-sm italic text-foreground">
                        {isSelection ? t('"Make it more polite"') : t('"How do you say "see you soon" in German?"')}
                    </span>
                </div>
            </PreviewCell>

            <PreviewArrow />

            {isSelection ? (
                <PreviewCell label={t('Replaced by')} labelStyle={{ color: palette.accent }}>
                    <p
                        className="border-l-2 pl-3 text-sm leading-relaxed text-foreground"
                        style={{ borderColor: palette.accent }}
                    >
                        {t('Could you send me the file when you have a moment?')}
                    </p>
                </PreviewCell>
            ) : (
                <div className="grid md:col-span-3">
                    <ResultCardPreview size="large" surface="field">
                        {t(
                            '"Bis bald" in everyday speech. In a more formal message, "Auf baldiges Wiedersehen". And "Bis später" for "see you later".'
                        )}
                    </ResultCardPreview>
                </div>
            )}
        </div>
    );
};
