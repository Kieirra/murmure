import clsx from 'clsx';
import { Trans } from 'react-i18next';
import { useTranslation } from '@/i18n';
import { RenderKeys } from '@/components/render-keys';
import { AudioVisualizer } from '@/features/home/audio-visualizer/audio-visualizer';
import { VISUALIZER_CONFIG } from '@/overlay/visualizer-config';
import { ResultCardPreview } from '../result-card-preview/result-card-preview';

interface OverlayPreviewProps {
    shortcut: string;
}

export const OverlayPreview = ({ shortcut }: OverlayPreviewProps) => {
    const { t } = useTranslation();
    const config = VISUALIZER_CONFIG.medium;

    return (
        <div className="w-full rounded-lg border border-border bg-zinc-950 p-6">
            <div className="mx-auto flex w-full max-w-[450px] flex-col items-center gap-0.5">
                <div className={clsx('flex items-center justify-center overflow-hidden bg-black', config.className)}>
                    <AudioVisualizer
                        level={0.1024}
                        isProcessing={false}
                        bars={config.bars}
                        rows={9}
                        audioPixelWidth={config.pixelWidth}
                        audioPixelHeight={config.pixelHeight}
                        colorScheme="command"
                    />
                </div>

                <div className="w-full">
                    <ResultCardPreview>
                        {shortcut.length > 0 ? (
                            <Trans
                                i18nKey='Select some text and press <keys/> to transform it, for example "translate to English". With nothing selected, ask a question and the answer shows up here.'
                                components={{ keys: <RenderKeys keyString={shortcut} /> }}
                            />
                        ) : (
                            t('No shortcut set.')
                        )}
                    </ResultCardPreview>
                </div>
            </div>
        </div>
    );
};
