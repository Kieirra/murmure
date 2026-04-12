import { invoke } from '@tauri-apps/api/core';
import { useState, useEffect } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';
import { AppSettings } from '@/features/settings/settings.types';

export const useOverlayState = () => {
    const [overlayMode, setOverlayMode] = useState<'hidden' | 'recording' | 'always'>('recording');
    const [overlayPosition, setOverlayPosition] = useState<'top' | 'bottom'>('bottom');
    const [streamingPreview, setStreamingPreview] = useState(false);
    const [overlaySize, setOverlaySize] = useState<'small' | 'medium' | 'large'>('small');
    const [streamingTextWidth, setStreamingTextWidth] = useState(450);
    const [streamingFontSize, setStreamingFontSize] = useState(11);
    const [streamingMaxLines, setStreamingMaxLines] = useState(5);
    const { t } = useTranslation();
    const showSaveError = () => toast.error(t('Failed to save overlay settings'));

    useEffect(() => {
        invoke<AppSettings>('get_all_settings').then((settings) => {
            const mode = settings.overlay_mode;
            if (mode === 'hidden' || mode === 'recording' || mode === 'always') setOverlayMode(mode);
            const position = settings.overlay_position;
            if (position === 'top' || position === 'bottom') setOverlayPosition(position);
            if (typeof settings.streaming_preview === 'boolean') setStreamingPreview(settings.streaming_preview);
            const size = settings.overlay_size;
            if (size === 'small' || size === 'medium' || size === 'large') setOverlaySize(size);
            if (typeof settings.streaming_text_width === 'number') setStreamingTextWidth(settings.streaming_text_width);
            if (typeof settings.streaming_font_size === 'number') setStreamingFontSize(settings.streaming_font_size);
            if (typeof settings.streaming_max_lines === 'number') setStreamingMaxLines(settings.streaming_max_lines);
        });
    }, []);

    return {
        overlayMode,
        setOverlayMode: (mode: 'hidden' | 'recording' | 'always') => {
            setOverlayMode(mode);
            invoke('set_overlay_mode', { mode }).catch(showSaveError);
        },
        overlayPosition,
        setOverlayPosition: (position: 'top' | 'bottom') => {
            setOverlayPosition(position);
            invoke('set_overlay_position', { position }).catch(showSaveError);
        },
        streamingPreview,
        setStreamingPreview: (enabled: boolean) => {
            setStreamingPreview(enabled);
            invoke('set_streaming_preview', { enabled }).catch(showSaveError);
        },
        overlaySize,
        setOverlaySize: (size: 'small' | 'medium' | 'large') => {
            setOverlaySize(size);
            invoke('set_overlay_size', { size }).catch(showSaveError);
        },
        streamingTextWidth,
        streamingFontSize,
        streamingMaxLines,
        setStreamingTextSettings: (textWidth: number, fontSize: number, maxLines: number) => {
            setStreamingTextWidth(textWidth);
            setStreamingFontSize(fontSize);
            setStreamingMaxLines(maxLines);
            invoke('set_streaming_text_settings', { textWidth, fontSize, maxLines }).catch(showSaveError);
        },
    };
};
