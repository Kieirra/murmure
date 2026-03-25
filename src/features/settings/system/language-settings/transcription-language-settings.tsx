import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { AudioLines } from 'lucide-react';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/select';
import { useTranslation } from '@/i18n';
import { useTranscriptionLanguageState } from './hooks/use-transcription-language-state';

const TRANSCRIPTION_LANGUAGES = [
    { code: 'auto', label: 'Auto' },
    { code: 'bg', label: 'Български' },
    { code: 'hr', label: 'Hrvatski' },
    { code: 'cs', label: 'Čeština' },
    { code: 'da', label: 'Dansk' },
    { code: 'nl', label: 'Nederlands' },
    { code: 'en', label: 'English' },
    { code: 'et', label: 'Eesti' },
    { code: 'fi', label: 'Suomi' },
    { code: 'fr', label: 'Français' },
    { code: 'de', label: 'Deutsch' },
    { code: 'el', label: 'Ελληνικά' },
    { code: 'hu', label: 'Magyar' },
    { code: 'it', label: 'Italiano' },
    { code: 'lv', label: 'Latviešu' },
    { code: 'lt', label: 'Lietuvių' },
    { code: 'mt', label: 'Malti' },
    { code: 'pl', label: 'Polski' },
    { code: 'pt', label: 'Português' },
    { code: 'ro', label: 'Română' },
    { code: 'sk', label: 'Slovenčina' },
    { code: 'sl', label: 'Slovenščina' },
    { code: 'es', label: 'Español' },
    { code: 'sv', label: 'Svenska' },
    { code: 'ru', label: 'Русский' },
    { code: 'uk', label: 'Українська' },
];

export const TranscriptionLanguageSettings = () => {
    const { t } = useTranslation();
    const { transcriptionLang, setTranscriptionLanguage } = useTranscriptionLanguageState();

    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title className="flex items-center gap-2">
                    <AudioLines className="w-4 h-4 text-muted-foreground" />
                    {t('Transcription language')}
                </Typography.Title>
                <Typography.Paragraph>
                    {t('Force the transcription language. Forcing a language may degrade quality for other languages.')}
                </Typography.Paragraph>
            </SettingsUI.Description>
            <Select value={transcriptionLang} onValueChange={setTranscriptionLanguage}>
                <SelectTrigger className="w-[180px]" data-testid="transcription-language-select">
                    <SelectValue />
                </SelectTrigger>
                <SelectContent>
                    {TRANSCRIPTION_LANGUAGES.map((lang) => (
                        <SelectItem key={lang.code} value={lang.code}>
                            {lang.label}
                        </SelectItem>
                    ))}
                </SelectContent>
            </Select>
        </SettingsUI.Item>
    );
};
