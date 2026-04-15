import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { Switch } from '@/components/switch';
import { Slider } from '@/components/slider';
import { Eye, Maximize2, MoveHorizontal, Rows3, Ruler, Subtitles, Type } from 'lucide-react';
import { useOverlayState } from './hooks/use-overlay-state';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/select';
import { useTranslation } from '@/i18n';
import { AudioVisualizer } from '@/features/home/audio-visualizer/audio-visualizer';
import clsx from 'clsx';
import { VISUALIZER_CONFIG } from '@/overlay/visualizer-config';

const LINE_HEIGHT_RATIO = 1.625;
const VERTICAL_PADDING_PX = 12;

export const OverlaySettings = () => {
    const {
        overlayMode,
        setOverlayMode,
        overlayPosition,
        setOverlayPosition,
        streamingPreview,
        setStreamingPreview,
        overlaySize,
        setOverlaySize,
        streamingTextWidth,
        streamingFontSize,
        streamingMaxLines,
        setStreamingTextSettings,
    } = useOverlayState();
    const { t } = useTranslation();

    return (
        <>
            <SettingsUI.Item>
                <div className="w-full space-y-2">
                    <span className="text-muted-foreground text-xs">{t('Preview')}</span>
                    <div className="flex flex-col items-center mx-auto">
                        <div
                            className={clsx(
                                'overflow-hidden flex items-center justify-center bg-black',
                                VISUALIZER_CONFIG[overlaySize].className
                            )}
                        >
                            <AudioVisualizer
                                level={0.5}
                                bars={VISUALIZER_CONFIG[overlaySize].bars}
                                rows={9}
                                audioPixelWidth={VISUALIZER_CONFIG[overlaySize].pixelWidth}
                                audioPixelHeight={VISUALIZER_CONFIG[overlaySize].pixelHeight}
                            />
                        </div>
                        {streamingPreview && (
                            <div
                                className="bg-black rounded-lg mt-0.5 px-2.5 py-1.5 leading-relaxed font-sans text-white"
                                style={{
                                    width: `${streamingTextWidth}px`,
                                    fontSize: `${streamingFontSize}px`,
                                    maxHeight: `${Math.ceil(streamingMaxLines * streamingFontSize * LINE_HEIGHT_RATIO) + VERTICAL_PADDING_PX}px`,
                                    overflow: 'hidden',
                                }}
                            >
                                So basically Murmure does real-time{' '}
                                <span className="text-cyan-400">transcription</span> as you speak, and it
                                runs entirely on your machine so nothing leaves your device. It handles
                                multiple languages, applies automatic{' '}
                                <span className="text-cyan-400">punctuation</span> and corrects words on
                                the fly. Pretty useful for writing emails or crafting{' '}
                                <span className="text-cyan-400">prompts</span> to ChatGPT or Claude Code...
                            </div>
                        )}
                    </div>
                </div>
            </SettingsUI.Item>
            <SettingsUI.Separator />
            <SettingsUI.Item>
                <SettingsUI.Description>
                    <Typography.Title className="flex items-center gap-2">
                        <Eye className="w-4 h-4 text-muted-foreground" />
                        {t('Overlay visibility')}
                    </Typography.Title>
                    <Typography.Paragraph>{t('Choose when to show the recording overlay.')}</Typography.Paragraph>
                </SettingsUI.Description>

                <div className="flex gap-2">
                    <Select value={overlayMode} onValueChange={setOverlayMode}>
                        <SelectTrigger className="w-[150px]">
                            <SelectValue placeholder={t('Select a mode')} />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem value="hidden">{t('Hidden')}</SelectItem>
                            <SelectItem value="recording">{t('While recording')}</SelectItem>
                            <SelectItem value="always">{t('Always')}</SelectItem>
                        </SelectContent>
                    </Select>
                </div>
            </SettingsUI.Item>
            <SettingsUI.Separator />
            <SettingsUI.Item>
                <SettingsUI.Description>
                    <Typography.Title className="flex items-center gap-2">
                        <Ruler className="w-4 h-4 text-muted-foreground" />
                        {t('Overlay position')}
                    </Typography.Title>
                    <Typography.Paragraph>
                        {t('Choose whether the overlay appears at the top or bottom.')}
                    </Typography.Paragraph>
                </SettingsUI.Description>
                <div className="flex gap-2">
                    <Select value={overlayPosition} onValueChange={setOverlayPosition}>
                        <SelectTrigger className="w-[150px]">
                            <SelectValue placeholder={t('Select a position')} />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem value="top">{t('Top')}</SelectItem>
                            <SelectItem value="bottom">{t('Bottom')}</SelectItem>
                        </SelectContent>
                    </Select>
                </div>
            </SettingsUI.Item>
            <SettingsUI.Separator />
            <SettingsUI.Item>
                <SettingsUI.Description>
                    <Typography.Title className="flex items-center gap-2">
                        <Maximize2 className="w-4 h-4 text-muted-foreground" />
                        {t('Overlay size')}
                    </Typography.Title>
                    <Typography.Paragraph>{t('Choose the size of the recording overlay.')}</Typography.Paragraph>
                </SettingsUI.Description>
                <div className="flex gap-2">
                    <Select value={overlaySize} onValueChange={setOverlaySize}>
                        <SelectTrigger className="w-[150px]">
                            <SelectValue placeholder={t('Select a size')} />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem value="small">{t('Small')}</SelectItem>
                            <SelectItem value="medium">{t('Medium')}</SelectItem>
                            <SelectItem value="large">{t('Large')}</SelectItem>
                        </SelectContent>
                    </Select>
                </div>
            </SettingsUI.Item>
            <SettingsUI.Separator />
            <SettingsUI.Item>
                <SettingsUI.Description>
                    <Typography.Title className="flex items-center gap-2">
                        <Subtitles className="w-4 h-4 text-muted-foreground" />
                        {t('Real-time preview')}
                    </Typography.Title>
                    <Typography.Paragraph>
                        {t('Shows live transcription in the overlay during recording')}
                    </Typography.Paragraph>
                </SettingsUI.Description>
                <Switch checked={streamingPreview} onCheckedChange={setStreamingPreview} />
            </SettingsUI.Item>
            {streamingPreview && (
                <>
                    <SettingsUI.Separator />
                    <SettingsUI.Item>
                        <SettingsUI.Description>
                            <Typography.Title className="flex items-center gap-2">
                                <MoveHorizontal className="w-4 h-4 text-muted-foreground" />
                                {t('Text width')}
                            </Typography.Title>
                            <Typography.Paragraph>
                                {t('Width of the streaming text zone in pixels')}
                            </Typography.Paragraph>
                        </SettingsUI.Description>
                        <Slider
                            value={[streamingTextWidth]}
                            onValueChange={([value]) =>
                                setStreamingTextSettings(value, streamingFontSize, streamingMaxLines)
                            }
                            min={200}
                            max={600}
                            step={50}
                            showValue
                            formatValue={(v) => `${v}px`}
                            className="w-[180px]"
                        />
                    </SettingsUI.Item>
                    <SettingsUI.Separator />
                    <SettingsUI.Item>
                        <SettingsUI.Description>
                            <Typography.Title className="flex items-center gap-2">
                                <Type className="w-4 h-4 text-muted-foreground" />
                                {t('Font size')}
                            </Typography.Title>
                            <Typography.Paragraph>{t('Size of the streaming text')}</Typography.Paragraph>
                        </SettingsUI.Description>
                        <Slider
                            value={[streamingFontSize]}
                            onValueChange={([value]) =>
                                setStreamingTextSettings(streamingTextWidth, value, streamingMaxLines)
                            }
                            min={8}
                            max={18}
                            step={1}
                            showValue
                            formatValue={(v) => `${v}px`}
                            className="w-[180px]"
                        />
                    </SettingsUI.Item>
                    <SettingsUI.Separator />
                    <SettingsUI.Item>
                        <SettingsUI.Description>
                            <Typography.Title className="flex items-center gap-2">
                                <Rows3 className="w-4 h-4 text-muted-foreground" />
                                {t('Max lines')}
                            </Typography.Title>
                            <Typography.Paragraph>{t('Maximum number of visible lines')}</Typography.Paragraph>
                        </SettingsUI.Description>
                        <Slider
                            value={[streamingMaxLines]}
                            onValueChange={([value]) =>
                                setStreamingTextSettings(streamingTextWidth, streamingFontSize, value)
                            }
                            min={1}
                            max={8}
                            step={1}
                            showValue
                            formatValue={(v) => `${v}`}
                            className="w-[180px]"
                        />
                    </SettingsUI.Item>
                </>
            )}
        </>
    );
};
