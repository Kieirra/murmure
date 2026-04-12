import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import type { AppSettings } from '@/features/settings/settings.types';
import type { OverlaySize } from './visualizer-config';

export const useOverlayConfig = () => {
    const [overlaySize, setOverlaySize] = useState<OverlaySize>('small');
    const [streamingTextSettings, setStreamingTextSettings] = useState({ textWidth: 450, fontSize: 11, maxLines: 5 });

    useEffect(() => {
        invoke<AppSettings>('get_all_settings').then((settings) => {
            const sz = settings.overlay_size;
            if (sz === 'small' || sz === 'medium' || sz === 'large') setOverlaySize(sz);
            setStreamingTextSettings((prev) => ({
                textWidth:
                    typeof settings.streaming_text_width === 'number' ? settings.streaming_text_width : prev.textWidth,
                fontSize:
                    typeof settings.streaming_font_size === 'number' ? settings.streaming_font_size : prev.fontSize,
                maxLines:
                    typeof settings.streaming_max_lines === 'number' ? settings.streaming_max_lines : prev.maxLines,
            }));
        });
    }, []);

    useEffect(() => {
        const unlisten = listen<string>('overlay-size-changed', (event) => {
            const sz = event.payload;
            if (sz === 'small' || sz === 'medium' || sz === 'large') setOverlaySize(sz);
        });
        return () => {
            unlisten.then((u) => u());
        };
    }, []);

    useEffect(() => {
        const unlisten = listen<{ text_width: number; font_size: number; max_lines: number }>(
            'streaming-text-settings-changed',
            (event) => {
                setStreamingTextSettings({
                    textWidth: event.payload.text_width,
                    fontSize: event.payload.font_size,
                    maxLines: event.payload.max_lines,
                });
            }
        );
        return () => {
            unlisten.then((u) => u());
        };
    }, []);

    return { overlaySize, streamingTextSettings };
};
