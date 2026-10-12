import { Button } from '@/components/button';
import { RenderKeys } from '@/components/render-keys';
import clsx from 'clsx';
import { useTranslation } from '@/i18n';
import { PartyPopper, X } from 'lucide-react';
import { useOnboardingState } from './hooks/use-onboarding-state';
import { useOnboardingCalculations } from './hooks/use-onboarding-calculations';
import { useOnboardingSuccess } from './hooks/use-onboarding-success';
import { OnboardingTask } from './onboarding-task/onboarding-task';

const blockClassName = 'rounded-md border border-sky-500/50 bg-surface p-2.5';
const successBlockClassName =
    'space-y-2 rounded-md border border-emerald-500/50 bg-emerald-500/10 p-2.5 motion-safe:transition-[opacity,scale] motion-safe:duration-300 motion-safe:starting:scale-95 motion-safe:starting:opacity-0';
const dismissButtonClassName = 'size-6 shrink-0 text-muted-foreground hover:text-foreground';

export const Onboarding = ({ recordShortcut }: { recordShortcut?: string }) => {
    const { t } = useTranslation();
    const { state, loading, refresh } = useOnboardingState();
    const { doneCount, isCompleted, completeAndDismiss, dismissCongrats } = useOnboardingCalculations(state, refresh);
    const { isSuccessVisible, isSuccessLeaving, closeSuccess, skipOnboarding } = useOnboardingSuccess({
        isCompleted,
        loading,
        completeAndDismiss,
        dismissCongrats,
    });

    if (isSuccessVisible) {
        return (
            <div
                role="status"
                className={clsx(successBlockClassName, isSuccessLeaving && 'scale-95 opacity-0')}
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
                        className={`ml-auto ${dismissButtonClassName}`}
                        data-testid="onboarding-congrats-dismiss"
                    >
                        <X className="size-3.5" />
                    </Button>
                </div>
                <div className="h-1 rounded-full bg-emerald-500" />
                <p className="text-xs text-muted-foreground">{t("You're all set to use Murmure everywhere.")}</p>
            </div>
        );
    }

    if (isCompleted) {
        return null;
    }

    return (
        <div className={`${blockClassName} space-y-2`} data-testid="onboarding-todo">
            <div className="flex items-center gap-2">
                <span className="text-sm font-medium text-foreground">{t('Getting started')}</span>
                <span className="ml-auto text-xs text-muted-foreground" data-testid="onboarding-todo-count">
                    {doneCount}/3
                </span>
                <Button
                    variant="subtle"
                    size="icon"
                    onClick={skipOnboarding}
                    aria-label={t('Cancel')}
                    className={dismissButtonClassName}
                    data-testid="onboarding-todo-dismiss"
                >
                    <X className="size-3.5" />
                </Button>
            </div>
            <div
                role="progressbar"
                aria-label={t('Getting started')}
                aria-valuemin={0}
                aria-valuemax={3}
                aria-valuenow={doneCount}
                className="h-1 overflow-hidden rounded-full bg-field"
                data-testid="onboarding-todo-progress"
            >
                <div
                    className="h-full rounded-full bg-sky-500 transition-[width] duration-300"
                    style={{ width: `${(doneCount / 3) * 100}%` }}
                />
            </div>
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
