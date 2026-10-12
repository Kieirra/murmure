import { OnboardingState } from './hooks/use-onboarding-state';

export type OnboardingSuccessPhase = 'idle' | 'in-progress' | 'success' | 'leaving' | 'done';

export function isOnboardingCompleted(state: OnboardingState): boolean {
    return state.used_home_shortcut && state.transcribed_outside_app && state.added_dictionary_word;
}

export function getNextOnboardingSuccessPhase(
    phase: OnboardingSuccessPhase,
    isCompleted: boolean
): OnboardingSuccessPhase {
    if (phase === 'idle' && !isCompleted) {
        return 'in-progress';
    }
    if (phase === 'in-progress' && isCompleted) {
        return 'success';
    }
    return phase;
}
