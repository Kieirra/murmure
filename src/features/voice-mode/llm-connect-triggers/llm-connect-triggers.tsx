import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { useTranslation } from '@/i18n';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useState, useEffect } from 'react';
import { VoiceTriggerItem } from '../voice-trigger-item/voice-trigger-item';
import { useLlmWakeWord } from '../hooks/use-llm-wake-word';
import type {
    LLMConnectSettings,
    LLMMode,
} from '@/features/llm-connect/hooks/use-llm-connect';

const LlmTriggerItem = ({
    index,
    mode,
}: {
    index: number;
    mode: LLMMode;
}) => {
    const {
        wakeWord,
        setWakeWord,
        handleBlur,
        isEnabled,
        toggleEnabled,
        defaultWord,
        resetToDefault,
    } = useLlmWakeWord({ index, modeName: mode.name });

    return (
        <VoiceTriggerItem
            title={mode.name}
            description={`Slot ${index + 1} - ${mode.shortcut}`}
            wakeWord={wakeWord}
            onWakeWordChange={setWakeWord}
            onBlur={handleBlur}
            placeholder={defaultWord}
            dataTestId={`wake-word-llm-mode-${index}-input`}
            isEnabled={isEnabled}
            onToggleEnabled={toggleEnabled}
            defaultWord={defaultWord}
            onReset={resetToDefault}
        />
    );
};

export const LlmConnectTriggers = () => {
    const { t } = useTranslation();
    const [settings, setSettings] = useState<LLMConnectSettings | null>(null);

    useEffect(() => {
        invoke<LLMConnectSettings>('get_llm_connect_settings')
            .then(setSettings)
            .catch((err) =>
                console.error('Failed to load LLM Connect settings:', err)
            );
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

    if (
        settings == null ||
        !settings.onboarding_completed ||
        settings.modes.length === 0
    ) {
        return null;
    }

    return (
        <section>
            <Typography.Title
                data-testid="llm-connect-triggers-title"
                className="p-2 font-semibold text-sky-400!"
            >
                {t('LLM Connect Triggers')}
            </Typography.Title>
            <SettingsUI.Container>
                {settings.modes.map((mode, index) => (
                    <div key={index}>
                        {index > 0 && <SettingsUI.Separator />}
                        <LlmTriggerItem index={index} mode={mode} />
                    </div>
                ))}
            </SettingsUI.Container>
        </section>
    );
};
