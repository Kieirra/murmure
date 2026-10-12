import { Sparkles } from 'lucide-react';
import { SettingsUI } from '@/components/settings-ui';
import { useTranslation } from '@/i18n';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useState, useEffect } from 'react';
import { PALETTES } from '@/features/home/audio-visualizer/audio-pixel/audio-pixel.helpers';
import { LlmTriggerItem } from './llm-trigger-item/llm-trigger-item';
import type { LLMConnectSettings } from '@/features/extensions/llm-connect/hooks/use-llm-connect';

export const LlmConnectTriggers = () => {
    const { t } = useTranslation();
    const [settings, setSettings] = useState<LLMConnectSettings | null>(null);

    useEffect(() => {
        invoke<LLMConnectSettings>('get_llm_connect_settings')
            .then(setSettings)
            .catch((err) => console.error('Failed to load LLM Connect settings:', err));
    }, []);

    useEffect(() => {
        let mounted = true;
        let unlisten: (() => void) | null = null;

        listen<LLMConnectSettings>('llm-settings-updated', (event) => {
            setSettings(event.payload);
        }).then((fn) => {
            if (mounted) {
                unlisten = fn;
            } else {
                fn();
            }
        });

        return () => {
            mounted = false;
            unlisten?.();
        };
    }, []);

    if (!settings?.onboarding_completed || !settings.enabled || settings.modes.length === 0) {
        return null;
    }

    return (
        <SettingsUI.Section
            title={t('Prompt Mode Triggers')}
            icon={Sparkles}
            iconColor={PALETTES.llm.accent}
            titleTestId="llm-connect-triggers-title"
        >
            {settings.modes.map((mode, index) => (
                <div key={mode.name}>
                    {index > 0 && <SettingsUI.Separator />}
                    <LlmTriggerItem index={index} mode={mode} />
                </div>
            ))}
        </SettingsUI.Section>
    );
};
