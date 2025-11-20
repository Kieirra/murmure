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
import { useTranslation } from '@/i18n';
import { useLanguageState } from './hooks/use-language-state';

const SUPPORTED_LANGUAGES = [
    { code: 'default', label: 'Default' },
    { code: 'en', label: 'English' },
    { code: 'fr', label: 'Français' },
];

export const LanguageSettings = () => {
    const { t } = useTranslation();
    const { currentLang, setLanguage } = useLanguageState();

    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title className="flex items-center gap-2">
                    <Languages className="w-4 h-4 text-zinc-400" />
                    {t('Language')}
                </Typography.Title>
                <Typography.Paragraph>
                    {t('Choose your preferred language for the interface.')}
                </Typography.Paragraph>
            </SettingsUI.Description>
            <Select value={currentLang} onValueChange={setLanguage}>
                <SelectTrigger className="w-[180px]" data-testid="language-select">
                    <SelectValue />
                </SelectTrigger>
                <SelectContent>
                    {SUPPORTED_LANGUAGES.map((lang) => (
                        <SelectItem key={lang.code} value={lang.code}>
                            {t(lang.label)}
                        </SelectItem>
                    ))}
                </SelectContent>
            </Select>
        </SettingsUI.Item>
    );
};
