import { useState } from 'react';
import clsx from 'clsx';
import { ArrowDown, ArrowRight } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { PALETTES } from '@/features/home/audio-visualizer/audio-pixel/audio-pixel.helpers';
import { TRANSLATION_PREVIEW_PAIR } from './llm-rewrite-preview.constants';

type PreviewPairId = 'correct' | 'translation' | 'medical';

interface PreviewPair {
    id: PreviewPairId;
    label: string;
    raw: string;
    result: string;
}

export const LLMRewritePreview = () => {
    const { t } = useTranslation();
    const [activePairId, setActivePairId] = useState<PreviewPairId>('translation');
    const palette = PALETTES.llm;

    const pairs: PreviewPair[] = [
        {
            id: 'translation',
            label: t('Translation'),
            raw: TRANSLATION_PREVIEW_PAIR.raw,
            result: TRANSLATION_PREVIEW_PAIR.result,
        },
        {
            id: 'correct',
            label: t('Correct'),
            raw: t("uh so tomorrow's meeting is moved to three, no, four, tell the team"),
            result: t("Tomorrow's meeting is moved to 4 pm. Please let the team know."),
        },
        {
            id: 'medical',
            label: t('Medical'),
            raw: t('patient on paracetamol one gram three times a day since monday'),
            result: t('Patient on paracetamol 1 g, 3 times a day since Monday.'),
        },
    ];
    const activePair = pairs.find((pair) => pair.id === activePairId) ?? pairs[0];

    return (
        <div className="w-full overflow-hidden rounded-lg border border-border bg-black text-left">
            <div className="flex gap-4 border-b border-border px-4">
                {pairs.map((pair) => {
                    const isActive = pair.id === activePair.id;
                    return (
                        <button
                            key={pair.id}
                            type="button"
                            aria-pressed={isActive}
                            onClick={() => setActivePairId(pair.id)}
                            className={clsx(
                                '-mb-px cursor-pointer border-b-2 pt-3 pb-2 text-sm transition-colors',
                                isActive && 'text-foreground',
                                !isActive && 'border-transparent text-muted-foreground hover:text-foreground'
                            )}
                            style={isActive ? { borderColor: palette.accent } : undefined}
                        >
                            {pair.label}
                        </button>
                    );
                })}
            </div>

            <div className="grid grid-cols-1 md:grid-cols-[1fr_auto_1fr] items-center gap-3 p-4">
                <div className="space-y-2">
                    <p className="text-[11px] uppercase tracking-wider text-muted-foreground">{t('You dictate')}</p>
                    <p className="min-h-[4.5rem] text-sm italic leading-relaxed text-muted-foreground">
                        {activePair.raw}
                    </p>
                </div>

                <div className="flex justify-center text-muted-foreground">
                    <ArrowDown className="w-4 h-4 md:hidden" />
                    <ArrowRight className="hidden w-4 h-4 md:block" />
                </div>

                <div key={activePair.id} className="space-y-2 motion-safe:animate-in motion-safe:fade-in">
                    <p className="text-[11px] uppercase tracking-wider text-foreground">{t('Murmure writes')}</p>
                    <p
                        className="min-h-[4.5rem] border-l-2 pl-3 text-sm leading-relaxed text-foreground"
                        style={{ borderColor: palette.accent }}
                    >
                        {activePair.result}
                    </p>
                </div>
            </div>
        </div>
    );
};
