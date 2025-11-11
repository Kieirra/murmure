import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { Languages } from 'lucide-react';
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from '@/components/select';
import { useTranslation, i18n } from '@/i18n';
import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { toast } from 'sonner';

const SUPPORTED_LANGUAGES = [
    { code: 'en', label: 'English' },
    { code: 'fr', label: 'Français' },
];

export const LanguageSettings = () => {
    const { t } = useTranslation(['common', 'settings']);
    const [currentLang, setCurrentLang] = useState<string>('en');
    const loadLanguageFailed = t('common:messages.loadLanguageFailed');

    useEffect(() => {
        const loadLanguage = async () => {
            try {
                const lang = await invoke<string>('get_current_language');
                setCurrentLang(lang);
                i18n.changeLanguage(lang);
            } catch (error) {
                console.error('Failed to load language:', error);
                toast.error(loadLanguageFailed, {
                    duration: 30000,
                    closeButton: true,
                });
            }
        };
        loadLanguage();
    }, []);

    const handleLanguageChange = async (lang: string) => {
        try {
            await invoke('set_current_language', { lang });
            setCurrentLang(lang);
            i18n.changeLanguage(lang);
        } catch (error) {
            console.error('Failed to save language:', error);
            toast.error(t('common:messages.saveLanguageFailed'), {
                duration: 2000,
                closeButton: true,
            });
        }
    };

    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title className="flex items-center gap-2">
                    <Languages className="w-4 h-4 text-zinc-400" />
                    {t('language.label')}
                </Typography.Title>
                <Typography.Paragraph>
                    {t('settings:system.language.description')}
                </Typography.Paragraph>
            </SettingsUI.Description>
            <Select value={currentLang} onValueChange={handleLanguageChange}>
                <SelectTrigger className="w-[180px]">
                    <SelectValue />
                </SelectTrigger>
                <SelectContent>
                    {SUPPORTED_LANGUAGES.map((lang) => (
                        <SelectItem key={lang.code} value={lang.code}>
                            {lang.label}
                        </SelectItem>
                    ))}
                </SelectContent>
            </Select>
        </SettingsUI.Item>
    );
};
