import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { useTranslation } from '@/i18n';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useState, useEffect } from 'react';
import { LlmTriggerItem } from './llm-trigger-item/llm-trigger-item';
import type { LLMConnectSettings } from '@/features/personalize/llm-connect/hooks/use-llm-connect';

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

    if (!settings?.onboarding_completed || settings.modes.length === 0) {
        return null;
    }

    return (
        <section>
            <Typography.Title data-testid="llm-connect-triggers-title" className="p-2 font-semibold text-sky-400!">
                {t('LLM Connect Triggers')}
            </Typography.Title>
            <SettingsUI.Container>
                {settings.modes.map((mode, index) => (
                    <div key={mode.name}>
                        {index > 0 && <SettingsUI.Separator />}
                        <LlmTriggerItem index={index} mode={mode} />
                    </div>
                ))}
            </SettingsUI.Container>
        </section>
    );
};
