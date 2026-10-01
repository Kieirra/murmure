import { Keyboard, Mic } from 'lucide-react';
import clsx from 'clsx';
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
        <div
            className={clsx(
                'flex flex-col min-[920px]:flex-row min-[920px]:items-center min-[920px]:justify-between gap-x-4 gap-y-2 w-full',
                'rounded-md border border-border bg-black/30 p-3'
            )}
        >
            <div className="flex items-center gap-2">
                <Keyboard className="w-4 h-4 shrink-0 text-sky-400" />
                <span className="text-sm font-medium text-foreground">{t('Shortcut')}</span>
                {shortcut.length > 0 ? (
                    <RenderKeys keyString={shortcut} />
                ) : (
                    <span className="text-sm text-muted-foreground">{t('No shortcut set.')}</span>
                )}
            </div>
            {isVoiceModeEnabled === true && (
                <div className="flex items-center gap-2">
                    <Mic className="w-4 h-4 shrink-0 text-sky-400" />
                    <span className="text-sm font-medium text-foreground">{t('Voice keyword')}</span>
                    <span className="text-sm text-foreground">
                        {wakeWord.trim().length > 0 ? `"${wakeWord}"` : t('Off')}
                    </span>
                </div>
            )}
        </div>
    );
};
