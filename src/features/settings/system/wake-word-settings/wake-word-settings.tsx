import { invoke } from '@tauri-apps/api/core';
import { useEffect, useRef, useState } from 'react';
import { SettingsUI } from '@/components/settings-ui';
import { Switch } from '@/components/switch';
import { Typography } from '@/components/typography';
import { Input } from '@/components/input';
import { Mic } from 'lucide-react';
import { useTranslation } from '@/i18n';

export function WakeWordSettings() {
    const [enabled, setEnabled] = useState(false);
    const [wakeWord, setWakeWord] = useState('murmure');
    const [previousWord, setPreviousWord] = useState('murmure');
    const { t } = useTranslation();
    const debounceRef = useRef<ReturnType<typeof setTimeout> | null>(null);

    useEffect(() => {
        invoke<boolean>('get_wake_word_enabled').then(setEnabled);
        invoke<string>('get_wake_word').then((word) => {
            setWakeWord(word);
            setPreviousWord(word);
        });
    }, []);

    const handleToggle = (checked: boolean) => {
        if (checked && wakeWord.trim().length === 0) {
            return;
        }
        setEnabled(checked);
        invoke('set_wake_word_enabled', { enabled: checked });
    };

    const handleWordChange = (value: string) => {
        setWakeWord(value);

        if (debounceRef.current != null) {
            clearTimeout(debounceRef.current);
        }

        debounceRef.current = setTimeout(() => {
            const trimmed = value.trim();
            if (trimmed.length > 0) {
                invoke('set_wake_word', { word: trimmed });
                setPreviousWord(trimmed);
            }
        }, 500);
    };

    const handleBlur = () => {
        if (wakeWord.trim().length === 0) {
            setWakeWord(previousWord);
        }
    };

    return (
        <SettingsUI.Item className="flex-col items-start gap-4">
            <div className="flex w-full justify-between items-center">
                <SettingsUI.Description>
                    <Typography.Title className="flex items-center gap-2">
                        <Mic className="w-4 h-4 text-zinc-400" />
                        {t('Wake Word')}
                    </Typography.Title>
                    <Typography.Paragraph>
                        {t(
                            'Trigger recording by saying a wake word. Recording stops automatically after silence.'
                        )}
                    </Typography.Paragraph>
                </SettingsUI.Description>
                <Switch checked={enabled} onCheckedChange={handleToggle} />
            </div>
            {enabled && (
                <div className="w-full space-y-2">
                    <Input
                        value={wakeWord}
                        onChange={(e) => handleWordChange(e.target.value)}
                        onBlur={handleBlur}
                        placeholder="murmure"
                        maxLength={50}
                    />
                    <Typography.Paragraph className="text-amber-400 text-xs">
                        {t(
                            'Experimental — Keeps your microphone listening continuously when enabled.'
                        )}
                    </Typography.Paragraph>
                </div>
            )}
        </SettingsUI.Item>
    );
}
