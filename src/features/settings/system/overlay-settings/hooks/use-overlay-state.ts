import { invoke } from '@tauri-apps/api/core';
import { useState, useEffect } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';
import { AppSettings } from '@/features/settings/settings.types';

export const useOverlayState = () => {
    const [overlayMode, setOverlayMode] = useState<'hidden' | 'recording' | 'always'>('recording');
    const [overlayPosition, setOverlayPosition] = useState<'top' | 'bottom'>('bottom');
    const [streamingPreview, setStreamingPreviewState] = useState(false);
    const [overlaySize, setOverlaySizeState] = useState<'small' | 'medium' | 'large'>('small');
    const [streamingTextWidth, setStreamingTextWidthState] = useState(450);
    const [streamingFontSize, setStreamingFontSizeState] = useState(11);
    const [streamingMaxLines, setStreamingMaxLinesState] = useState(5);
    const { t } = useTranslation();

    useEffect(() => {
        invoke<AppSettings>('get_all_settings').then((settings) => {
            const mode = settings.overlay_mode;
            if (mode === 'hidden' || mode === 'recording' || mode === 'always') setOverlayMode(mode);
            const position = settings.overlay_position;
            if (position === 'top' || position === 'bottom') setOverlayPosition(position);
            if (typeof settings.streaming_preview === 'boolean') setStreamingPreviewState(settings.streaming_preview);
            const size = settings.overlay_size;
            if (size === 'small' || size === 'medium' || size === 'large') setOverlaySizeState(size);
            if (typeof settings.streaming_text_width === 'number')
                setStreamingTextWidthState(settings.streaming_text_width);
            if (typeof settings.streaming_font_size === 'number')
                setStreamingFontSizeState(settings.streaming_font_size);
            if (typeof settings.streaming_max_lines === 'number')
                setStreamingMaxLinesState(settings.streaming_max_lines);
        });
    }, []);

    return {
        setOverlayMode: (mode: 'hidden' | 'recording' | 'always') => {
            setOverlayMode(mode);
            invoke('set_overlay_mode', { mode }).catch(() => {
                toast.error(t('Failed to save overlay settings'));
            });
        },
        setOverlayPosition: (position: 'top' | 'bottom') => {
            setOverlayPosition(position);
            invoke('set_overlay_position', { position }).catch(() => {
                toast.error(t('Failed to save overlay settings'));
            });
        },
        overlayMode,
        overlayPosition,
        streamingPreview,
        setStreamingPreview: (enabled: boolean) => {
            setStreamingPreviewState(enabled);
            invoke('set_streaming_preview', { enabled }).catch(() => {
                toast.error(t('Failed to save overlay settings'));
            });
        },
        overlaySize,
        setOverlaySize: (size: 'small' | 'medium' | 'large') => {
            setOverlaySizeState(size);
            invoke('set_overlay_size', { size }).catch(() => {
                toast.error(t('Failed to save overlay settings'));
            });
        },
        streamingTextWidth,
        streamingFontSize,
        streamingMaxLines,
        setStreamingTextSettings: (textWidth: number, fontSize: number, maxLines: number) => {
            setStreamingTextWidthState(textWidth);
            setStreamingFontSizeState(fontSize);
            setStreamingMaxLinesState(maxLines);
            invoke('set_streaming_text_settings', { textWidth, fontSize, maxLines }).catch(() => {
                toast.error(t('Failed to save overlay settings'));
            });
        },
    };
};
