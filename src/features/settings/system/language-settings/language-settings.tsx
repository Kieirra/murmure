import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { Languages, FolderOpen } from 'lucide-react';
import { Button } from '@/components/button';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/select';
import { useTranslation } from '@/i18n';
import { useLanguageState } from './hooks/use-language-state';
import { useTtsVoiceState } from './hooks/use-tts-voice-state';
import { downloadDir } from '@tauri-apps/api/path';
import { revealItemInDir } from '@tauri-apps/plugin-opener';
import { open } from '@tauri-apps/plugin-dialog';
import { useAudioExportFolderState } from './hooks/use-audio-export-folder-state';

const SUPPORTED_LANGUAGES = [
    { code: 'default', label: 'Default' },
    { code: 'en', label: 'English' },
    { code: 'fr', label: 'Français' },
];

export const LanguageSettings = () => {
    const { t } = useTranslation();
    const { currentLang, setLanguage } = useLanguageState();
    const { ttsVoice, setTtsVoice } = useTtsVoiceState();
    const { exportFolder, setExportFolder } = useAudioExportFolderState();

    // English voices provided by Kokoro
    const ENGLISH_VOICES = [
        { code: 'af_heart', label: 'American Female (Heart) - Default' },
        { code: 'af_bella', label: 'American Female (Bella)' },
        { code: 'af_nicole', label: 'American Female (Nicole)' },
        { code: 'af_sky', label: 'American Female (Sky)' },
        { code: 'af_alloy', label: 'American Female (Alloy)' },
        { code: 'af_aoede', label: 'American Female (Aoede)' },
        { code: 'am_adam', label: 'American Male (Adam)' },
        { code: 'am_michael', label: 'American Male (Michael)' },
        { code: 'am_onyx', label: 'American Male (Onyx)' },
        { code: 'bf_emma', label: 'British Female (Emma)' },
        { code: 'bf_alice', label: 'British Female (Alice)' },
        { code: 'bm_george', label: 'British Male (George)' },
        { code: 'bm_lewis', label: 'British Male (Lewis)' },
    ];
    
    // Determine the effective language (if default, rely on i18n detection)
    const showEnglishVoice = currentLang === 'default' || currentLang === 'en';
    const showFrenchVoice = currentLang === 'default' || currentLang === 'fr';

    const handleChangeFolder = async () => {
        try {
            const selected = await open({
                directory: true,
                multiple: false,
                title: t('Select Audio Export Folder'),
            });
            if (selected && typeof selected === 'string') {
                await setExportFolder(selected);
            }
        } catch (error) {
            console.error('Failed to open directory dialog:', error);
        }
    };

    const handleOpenFolder = async () => {
        try {
            const path = exportFolder ? exportFolder : await downloadDir();
            await revealItemInDir(path);
        } catch (error) {
            console.error('Failed to open directory:', error);
        }
    };

    return (
        <>
            <SettingsUI.Item>
                <SettingsUI.Description>
                <Typography.Title className="flex items-center gap-2">
                    <Languages className="w-4 h-4 text-muted-foreground" />
                    {t('Language')}
                </Typography.Title>
                <Typography.Paragraph>{t('Choose your preferred language for the interface.')}</Typography.Paragraph>
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

            {showFrenchVoice && (
                <SettingsUI.Item>
                    <SettingsUI.Description>
                        <Typography.Title className="flex items-center gap-2">
                            <Languages className="w-4 h-4 text-muted-foreground" />
                            {t('TTS Voice')}
                        </Typography.Title>
                        <Typography.Paragraph>{t('Choose the voice model for speech synthesis.')}</Typography.Paragraph>
                    </SettingsUI.Description>
                    <Select value="ff_siwis" disabled>
                        <SelectTrigger className="w-[180px]" data-testid="tts-voice-select">
                            <SelectValue />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem value="ff_siwis">French Female (Siwis)</SelectItem>
                        </SelectContent>
                    </Select>
                </SettingsUI.Item>
            )}

            {showEnglishVoice && (
                <SettingsUI.Item>
                    <SettingsUI.Description>
                        <Typography.Title className="flex items-center gap-2">
                            <Languages className="w-4 h-4 text-muted-foreground" />
                            {t('TTS Voice (English)')}
                        </Typography.Title>
                        <Typography.Paragraph>{t('Choose the English voice model for speech synthesis.')}</Typography.Paragraph>
                    </SettingsUI.Description>
                    <Select value={ttsVoice} onValueChange={setTtsVoice}>
                        <SelectTrigger className="w-[180px]" data-testid="tts-voice-select">
                            <SelectValue />
                        </SelectTrigger>
                        <SelectContent className="max-h-[250px]">
                            {ENGLISH_VOICES.map((voice) => (
                                <SelectItem key={voice.code} value={voice.code}>
                                    {voice.label}
                                </SelectItem>
                            ))}
                        </SelectContent>
                    </Select>
                </SettingsUI.Item>
            )}

            {(showFrenchVoice || showEnglishVoice) && (
                <SettingsUI.Item>
                    <SettingsUI.Description>
                        <Typography.Title className="flex items-center gap-2">
                            <FolderOpen className="w-4 h-4 text-muted-foreground" />
                            {t('Audio Export Folder')}
                        </Typography.Title>
                        <Typography.Paragraph>
                            {exportFolder 
                                ? t('Exported TTS audio files are automatically saved to: {{folder}}', { folder: exportFolder })
                                : t('Exported TTS audio files are automatically saved to your Downloads folder.')
                            }
                        </Typography.Paragraph>
                    </SettingsUI.Description>
                    <div className="flex flex-col gap-2">
                        <Button variant="outline" onClick={handleChangeFolder}>
                            {t('Change Folder')}
                        </Button>
                        <Button variant="ghost" onClick={handleOpenFolder}>
                            {t('Open Folder')}
                        </Button>
                    </div>
                </SettingsUI.Item>
            )}
        </>
    );
};
