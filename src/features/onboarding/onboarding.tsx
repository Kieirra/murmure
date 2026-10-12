import { Button } from '@/components/button';
import { RenderKeys } from '@/components/render-keys';
import clsx from 'clsx';
import { useTranslation } from '@/i18n';
import { PartyPopper, X } from 'lucide-react';
import { useOnboardingState } from './hooks/use-onboarding-state';
import { useOnboardingCalculations } from './hooks/use-onboarding-calculations';
import { useOnboardingSuccess } from './hooks/use-onboarding-success';
import { OnboardingTask } from './onboarding-task/onboarding-task';

const ONBOARDING_TASK_COUNT = 3;

export const Onboarding = ({ recordShortcut }: { recordShortcut?: string }) => {
    const { t } = useTranslation();
    const { state, loading, refresh } = useOnboardingState();
    const { doneCount, isCompleted, completeAndDismiss, dismissCongrats } = useOnboardingCalculations(state, refresh);
    const { isSuccessVisible, isSuccessLeaving, closeSuccess, skipOnboarding } = useOnboardingSuccess({
        isCompleted,
        isCongratsDismissed: state.congrats_dismissed,
        loading,
        completeAndDismiss,
        dismissCongrats,
    });

    if (isSuccessVisible) {
        return (
            <output
                className={clsx(
                    'block space-y-2 rounded-md border border-emerald-500/50 bg-emerald-500/10 p-2.5',
                    'motion-safe:transition-[opacity,scale] motion-safe:duration-300',
                    'motion-safe:starting:scale-95 motion-safe:starting:opacity-0',
                    isSuccessLeaving && 'scale-95 opacity-0'
                )}
                data-testid="onboarding-congrats"
            >
                <div className="flex items-center gap-2">
                    <PartyPopper className="size-4 shrink-0 text-emerald-400" />
                    <span className="text-sm font-medium text-foreground">{t('Well done!')}</span>
                    <Button
                        variant="subtle"
                        size="icon"
                        onClick={closeSuccess}
                        aria-label={t('Close')}
                        className="ml-auto size-6 shrink-0 text-muted-foreground hover:text-foreground"
                        data-testid="onboarding-congrats-dismiss"
                    >
                        <X className="size-3.5" />
                    </Button>
                </div>
                <div className="h-1 rounded-full bg-emerald-500" />
                <p className="text-xs text-muted-foreground">{t("You're all set to use Murmure everywhere.")}</p>
            </output>
        );
    }

    if (isCompleted) {
        return null;
    }

    return (
        <div className="rounded-md border border-sky-500/50 bg-surface p-2.5 space-y-2" data-testid="onboarding-todo">
            <div className="flex items-center gap-2">
                <span className="text-sm font-medium text-foreground">{t('Getting started')}</span>
                <span className="ml-auto text-xs text-muted-foreground" data-testid="onboarding-todo-count">
                    {doneCount}/{ONBOARDING_TASK_COUNT}
                </span>
                <Button
                    variant="subtle"
                    size="icon"
                    onClick={skipOnboarding}
                    aria-label={t('Cancel')}
                    className="size-6 shrink-0 text-muted-foreground hover:text-foreground"
                    data-testid="onboarding-todo-dismiss"
                >
                    <X className="size-3.5" />
                </Button>
            </div>
            <progress
                aria-label={t('Getting started')}
                value={doneCount}
                max={ONBOARDING_TASK_COUNT}
                className={clsx(
                    'block h-1 w-full appearance-none overflow-hidden rounded-full border-0 bg-field',
                    '[&::-webkit-progress-bar]:bg-field',
                    '[&::-webkit-progress-value]:rounded-full [&::-webkit-progress-value]:bg-sky-500',
                    '[&::-webkit-progress-value]:transition-[width] [&::-webkit-progress-value]:duration-300',
                    '[&::-moz-progress-bar]:rounded-full [&::-moz-progress-bar]:bg-sky-500'
                )}
                data-testid="onboarding-todo-progress"
            />
            <ul className="text-xs">
                <OnboardingTask
                    done={state.used_home_shortcut}
                    label={t('Try dictation')}
                    description={t(
                        'Murmure use the default microphone to record your voice. Make sure your microphone is well set up.'
                    )}
                    dataTestId="onboarding-task-dictation"
                >
                    {recordShortcut != null && (
                        <RenderKeys
                            keyString={recordShortcut}
                            className={clsx('ml-auto shrink-0', recordShortcut.split('+').length > 2 && '[&_kbd]:px-1')}
                        />
                    )}
                </OnboardingTask>
                <OnboardingTask
                    done={state.transcribed_outside_app}
                    label={t('Dictate in another app')}
                    description={t(
                        'Place your cursor in any textbox of any software and try to transcribe your voice.'
                    )}
                    dataTestId="onboarding-task-outside-app"
                />
                <OnboardingTask
                    done={state.added_dictionary_word}
                    label={t('Add a word to the dictionary')}
                    description={t(
                        'Go to Personalize > Dictionary and add a word to make it available for future transcriptions.'
                    )}
                    to="/personalize/custom-dictionary"
                    dataTestId="onboarding-task-dictionary"
                />
            </ul>
        </div>
    );
};
