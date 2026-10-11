import clsx from 'clsx';
import { Trans } from 'react-i18next';
import { useTranslation } from '@/i18n';
import { RenderKeys } from '@/components/render-keys';
import { SettingsUI } from '@/components/settings-ui';
import { AudioVisualizer } from '@/features/home/audio-visualizer/audio-visualizer';
import { VISUALIZER_CONFIG } from '@/overlay/visualizer-config';
import { ResultCardPreview } from '../result-card-preview/result-card-preview';
import { CommandTriggers } from '../command-triggers/command-triggers';

interface OverlayPreviewProps {
    shortcut: string;
}

export const OverlayPreview = ({ shortcut }: OverlayPreviewProps) => {
    const { t } = useTranslation();
    const config = VISUALIZER_CONFIG.medium;

    return (
        <SettingsUI.Section title={t('How to use it')} aside={<CommandTriggers shortcut={shortcut} />}>
            <div className="rounded-b-[5px] bg-surface-header p-6">
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
        </SettingsUI.Section>
    );
};
