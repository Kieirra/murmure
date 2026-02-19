import { useEffect, useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';
import {
    FormattingSettings,
    FormattingRule,
    MatchMode,
    defaultFormattingSettings,
    migrateRule,
} from '../types';

export const useFormattingRules = () => {
    const [settings, setSettings] = useState<FormattingSettings>(
        defaultFormattingSettings
    );
    const [isLoading, setIsLoading] = useState(true);
    const { t } = useTranslation();

    const loadSettings = useCallback(async () => {
        try {
            const loaded = await invoke<FormattingSettings>(
                'get_formatting_settings'
            );
            const migratedSettings = {
                ...loaded,
                rules: loaded.rules.map((rule) =>
                    migrateRule(rule as unknown as Record<string, unknown>)
                ),
            };
            setSettings(migratedSettings);
        } catch (error) {
            console.error('Failed to load formatting settings:', error);
        } finally {
            setIsLoading(false);
        }
    }, []);

    useEffect(() => {
        loadSettings();
    }, [loadSettings]);

    const saveSettings = useCallback(
        async (newSettings: FormattingSettings) => {
            setSettings(newSettings);
            try {
                await invoke('set_formatting_settings', {
                    settings: newSettings,
                });
            } catch (error) {
                console.error('Failed to save formatting settings:', error);
                toast.error(t('Failed to save formatting rules'));
            }
        },
        [t]
    );

    const updateBuiltInOption = useCallback(
        async (
            key: keyof FormattingSettings['built_in'],
            value: boolean | string | number
        ) => {
            const newSettings = {
                ...settings,
                built_in: {
                    ...settings.built_in,
                    [key]: value,
                },
            };
            await saveSettings(newSettings);
        },
        [settings, saveSettings]
    );

    const addRule = useCallback(
        async (trigger: string, replacement: string, matchMode: MatchMode) => {
            const newRule: FormattingRule = {
                id: crypto.randomUUID(),
                trigger,
                replacement,
                enabled: true,
                match_mode: matchMode,
            };
            const newSettings = {
                ...settings,
                rules: [...settings.rules, newRule],
            };
            await saveSettings(newSettings);
        },
        [settings, saveSettings]
    );

    const updateRule = useCallback(
        async (id: string, updates: Partial<Omit<FormattingRule, 'id'>>) => {
            const newSettings = {
                ...settings,
                rules: settings.rules.map((rule) =>
                    rule.id === id ? { ...rule, ...updates } : rule
                ),
            };
            await saveSettings(newSettings);
        },
        [settings, saveSettings]
    );

    const deleteRule = useCallback(
        async (id: string) => {
            const newSettings = {
                ...settings,
                rules: settings.rules.filter((rule) => rule.id !== id),
            };
            await saveSettings(newSettings);
        },
        [settings, saveSettings]
    );

    const duplicateRule = useCallback(
        async (id: string) => {
            const ruleToDuplicate = settings.rules.find(
                (rule) => rule.id === id
            );
            if (!ruleToDuplicate) return;

            const newRule: FormattingRule = {
                ...ruleToDuplicate,
                id: crypto.randomUUID(),
            };
            const newSettings = {
                ...settings,
                rules: [...settings.rules, newRule],
            };
            await saveSettings(newSettings);
        },
        [settings, saveSettings]
    );

    return {
        settings,
        isLoading,
        updateBuiltInOption,
        addRule,
        updateRule,
        deleteRule,
        duplicateRule,
    };
};
