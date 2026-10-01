import { type ReactNode } from 'react';
import { useTranslation } from '@/i18n';
import {
    buildResultPanelGradient,
    getResultPanelPalette,
    RESULT_PANEL_SEGMENT_MASK,
} from '@/overlay/result-panel/result-panel.helpers';

interface ResultCardPreviewProps {
    children: ReactNode;
}

export const ResultCardPreview = ({ children }: ResultCardPreviewProps) => {
    const { t } = useTranslation();
    const palette = getResultPanelPalette('command');

    return (
        <div className="relative overflow-hidden rounded-t-lg border border-neutral-800 bg-black px-2.5 pt-1.5 pb-3">
            <div className="truncate pb-0.5 pr-5 font-sans text-[10px] font-normal" style={{ color: palette.accent }}>
                {t('Command')}
            </div>
            <div className="font-sans text-xs leading-relaxed text-white">{children}</div>
            <div
                className="absolute bottom-0 left-0 right-0 h-1 bg-neutral-800"
                style={{
                    maskImage: RESULT_PANEL_SEGMENT_MASK,
                    WebkitMaskImage: RESULT_PANEL_SEGMENT_MASK,
                }}
            >
                <div
                    className="h-full w-[60%]"
                    style={{
                        backgroundImage: buildResultPanelGradient('command'),
                        backgroundSize: '166.67% 100%',
                        backgroundRepeat: 'no-repeat',
                    }}
                />
            </div>
        </div>
    );
};
