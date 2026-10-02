import { type ReactNode } from 'react';
import clsx from 'clsx';
import { useTranslation } from '@/i18n';
import {
    buildResultPanelGradient,
    getResultPanelPalette,
    RESULT_PANEL_SEGMENT_MASK,
} from '@/overlay/result-panel/result-panel.helpers';

interface ResultCardPreviewProps {
    children: ReactNode;
    size?: 'default' | 'large';
}

export const ResultCardPreview = ({ children, size = 'default' }: ResultCardPreviewProps) => {
    const { t } = useTranslation();
    const palette = getResultPanelPalette('command');
    const isLarge = size === 'large';

    return (
        <div
            className={clsx(
                'relative overflow-hidden rounded-t-lg border border-neutral-800 bg-black pb-3',
                isLarge ? 'px-3 pt-2' : 'px-2.5 pt-1.5'
            )}
        >
            <div
                className={clsx('truncate pb-0.5 pr-5 font-sans font-normal', isLarge ? 'text-xs' : 'text-[10px]')}
                style={{ color: palette.accent }}
            >
                {t('Command')}
            </div>
            <div className={clsx('font-sans leading-relaxed text-white', isLarge ? 'text-sm' : 'text-xs')}>
                {children}
            </div>
            <div
                className="absolute bottom-0 left-0 right-0 h-1 bg-neutral-800"
                style={{
                    maskImage: RESULT_PANEL_SEGMENT_MASK,
                    WebkitMaskImage: RESULT_PANEL_SEGMENT_MASK,
                }}
            >
                <div
                    className={clsx('h-full', isLarge ? 'w-full' : 'w-[60%]')}
                    style={{
                        backgroundImage: buildResultPanelGradient('command'),
                        backgroundSize: isLarge ? '100% 100%' : '166.67% 100%',
                        backgroundRepeat: 'no-repeat',
                    }}
                />
            </div>
        </div>
    );
};
