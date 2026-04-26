import { Typography } from '@/components/typography';
import { useTranslation } from '@/i18n';
import { BadgeCheck, Info, X } from 'lucide-react';
import { useOnboardingState } from './hooks/use-onboarding-state';
import { useOnboardingCalculations } from './hooks/use-onboarding-calculations';
import { OnboardingTask } from './onboarding-task/onboarding-task';
import { useMicState } from '@/features/settings/system/mic-settings/hooks/use-mic-state';
import { useIsWayland } from '@/components/hooks/use-linux-session-type';

const WaylandExperimentalNotice = () => {
    const { t } = useTranslation();
    return (
        <div className="flex items-start gap-2 text-muted-foreground">
            <Info className="w-4 h-4 mt-0.5 shrink-0" />
            <Typography.Paragraph>
                {t(
                    'Wayland support is experimental, expect occasional issues. Your transcription is automatically copied to the clipboard, just press Ctrl+V to paste it anywhere.'
                )}
            </Typography.Paragraph>
        </div>
    );
};

const OnboardingCompletedMessage = () => {
    const { t } = useTranslation();
    const { currentMic, micList } = useMicState();
    const selectedMic = currentMic === 'automatic' ? null : micList.find((m) => m.id === currentMic);

    return (
        <Typography.Paragraph className="text-muted-foreground">
            {selectedMic ? (
                <>
                    {t('Recording with ')}
                    <span className="text-sky-400">{selectedMic.label}</span>
                </>
            ) : (
                t('Murmure uses your default microphone to record your voice.')
            )}
        </Typography.Paragraph>
    );
};

export const Onboarding = ({ recordShortcut }: { recordShortcut?: string }) => {
    const { t } = useTranslation();
    const isWayland = useIsWayland();
    const { state, refresh } = useOnboardingState();
    const { doneCount, isCompleted, showCongrats, completeAndDismiss, dismissCongrats } = useOnboardingCalculations(
        state,
        refresh
    );

    if (isWayland) {
        return <WaylandExperimentalNotice />;
    }

    if (isCompleted) {
        if (!showCongrats) {
            return <OnboardingCompletedMessage />;
        }
        return (
            <div className="rounded-md border border-sky-500 bg-sky-900/20 p-3 relative">
                <div className="flex items-center gap-2 justify-between">
                    <Typography.Paragraph className="text-sky-300! font-bold flex gap-2 items-center">
                        <BadgeCheck />
                        {t("Perfect! You're all set to use Murmure everywhere.")}
                    </Typography.Paragraph>
                    <button
                        type="button"
                        onClick={dismissCongrats}
                        aria-label={t('Close')}
                        className="text-muted-foreground hover:text-foreground"
                    >
                        <X className="w-4 h-4 cursor-pointer" />
                    </button>
                </div>
            </div>
        );
    }

    return (
        <div className="rounded-md border border-sky-500 bg-sky-900/20 p-4 space-y-2 relative">
            <div className="absolute top-2 right-2 flex">
                <Typography.Paragraph className="text-sky-300! font-bold">{doneCount}/3</Typography.Paragraph>
                <button
                    type="button"
                    onClick={completeAndDismiss}
                    aria-label={t('Cancel')}
                    className=" text-muted-foreground hover:text-foreground px-2 p-0.5"
                >
                    <X className="w-4 h-4 cursor-pointer" />
                </button>
            </div>
            <ul className="text-sm">
                <OnboardingTask
                    done={state.used_home_shortcut}
                    label={
                        recordShortcut != null
                            ? t('To test transcription, press "{{recordShortcut}}", talk, then release', {
                                  recordShortcut,
                              })
                            : t('Use the record shortcut on the Home page')
                    }
                    description={t(
                        'Murmure use the default microphone to record your voice. Make sure your microphone is well set up.'
                    )}
                />
                <OnboardingTask
                    done={state.transcribed_outside_app}
                    label={t('Use murmure in another app')}
                    description={t(
                        'Place your cursor in any textbox of any software and try to transcribe your voice.'
                    )}
                />
                <OnboardingTask
                    done={state.added_dictionary_word}
                    label={t('Add a word to the Custom Dictionary')}
                    description={t(
                        'Go to Personalize > Custom Dictionary and add a word to make it available for future transcriptions.'
                    )}
                />
            </ul>
        </div>
    );
};
