import { ArrowDown, ArrowRight, Mic } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { getResultPanelPalette } from '@/overlay/result-panel/result-panel.helpers';
import { ResultCardPreview } from '../result-card-preview/result-card-preview';
import { PreviewCell } from './preview-cell/preview-cell';

interface CommandPreviewProps {
    variant: 'selection' | 'question';
}

export const CommandPreview = ({ variant }: CommandPreviewProps) => {
    const { t } = useTranslation();
    const palette = getResultPanelPalette('command');
    const isSelection = variant === 'selection';

    const arrow = (
        <div className="flex justify-center self-center text-muted-foreground">
            <ArrowDown className="w-4 h-4 md:hidden" />
            <ArrowRight className="hidden w-4 h-4 md:block" />
        </div>
    );

    return (
        <div className="overflow-hidden rounded-lg border border-border bg-zinc-950 text-left">
            <div className="border-b border-border px-6 py-4">
                <p className="text-[11px] font-semibold uppercase tracking-wider text-foreground">
                    {isSelection ? t('With selected text') : t('Without selection')}
                </p>
            </div>

            <div className="grid grid-cols-1 gap-4 p-6 md:grid-cols-[1fr_auto_1fr_auto_1fr]">
                {isSelection ? (
                    <PreviewCell label={t('Selected text')} centered>
                        <p className="text-sm leading-relaxed">
                            <span className="bg-sky-500/40 text-foreground">{t('Send me the file now.')}</span>
                        </p>
                    </PreviewCell>
                ) : (
                    <PreviewCell label={t('Nothing selected')} dashed centered />
                )}

                {arrow}

                <PreviewCell label={t('You say')} centered>
                    <div className="flex items-center gap-2">
                        <Mic className="w-4 h-4 shrink-0 motion-safe:animate-pulse" style={{ color: palette.accent }} />
                        <span className="text-sm italic text-foreground">
                            {isSelection ? t('"Make it more polite"') : t('"What does idempotent mean?"')}
                        </span>
                    </div>
                </PreviewCell>

                {arrow}

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
                    <ResultCardPreview>
                        {t(
                            'An operation is idempotent if running it several times gives the same result as running it once.'
                        )}
                    </ResultCardPreview>
                )}
            </div>
        </div>
    );
};
