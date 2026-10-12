import { Keyboard, Mic } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { RenderKeys } from '@/components/render-keys';
import { useWakeWord, WAKE_WORD_CONFIGS } from '@/features/extensions/voice-mode/hooks/use-wake-word';
import { useWakeWordEnabled } from '@/features/extensions/voice-mode/hooks/use-wake-word-enabled';

interface CommandTriggersProps {
    shortcut: string;
}

export const CommandTriggers = ({ shortcut }: CommandTriggersProps) => {
    const { t } = useTranslation();
    const { wakeWord } = useWakeWord(WAKE_WORD_CONFIGS.command);
    const { enabled: isVoiceModeEnabled } = useWakeWordEnabled();

    return (
        <div className="flex flex-wrap items-center justify-end gap-x-4 gap-y-1 text-xs">
            <div className="flex items-center gap-1.5">
                <Keyboard className="w-3.5 h-3.5 shrink-0 text-muted-foreground" />
                <span className="text-muted-foreground">{t('Shortcut')}</span>
                {shortcut.length > 0 ? (
                    <RenderKeys keyString={shortcut} />
                ) : (
                    <span className="text-muted-foreground">{t('No shortcut set.')}</span>
                )}
            </div>
            {isVoiceModeEnabled === true && (
                <div className="flex items-center gap-1.5">
                    <Mic className="w-3.5 h-3.5 shrink-0 text-muted-foreground" />
                    <span className="text-muted-foreground">{t('Voice keyword')}</span>
                    <span className="text-foreground">{wakeWord.trim().length > 0 ? `"${wakeWord}"` : t('Off')}</span>
                </div>
            )}
        </div>
    );
};
