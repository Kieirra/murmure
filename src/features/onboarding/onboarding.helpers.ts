import { OnboardingState } from './hooks/use-onboarding-state';

export function isOnboardingCompleted(state: OnboardingState): boolean {
    return (
        state.used_home_shortcut &&
        state.transcribed_outside_app &&
        state.added_dictionary_word
    );
}
