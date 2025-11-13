import {
    isOnboardingCongratsPending,
    setOnboardingCongratsPending,
} from '../store/onboarding-session';
import { invoke } from '@tauri-apps/api/core';

interface OnboardingState {
    used_home_shortcut: boolean;
    transcribed_outside_app: boolean;
    added_dictionary_word: boolean;
}

export const useOnboardingCalculations = (
    state: OnboardingState,
    refresh: () => void
) => {
    const doneCount =
        Number(state.used_home_shortcut) +
        Number(state.transcribed_outside_app) +
        Number(state.added_dictionary_word);

    const isCompleted =
        state.used_home_shortcut &&
        state.transcribed_outside_app &&
        state.added_dictionary_word;

    const showCongrats = isOnboardingCongratsPending();

    const completeAndDismiss = () => {
        Promise.all([
            invoke('set_onboarding_used_home_shortcut'),
            invoke('set_onboarding_transcribed_outside_app'),
            invoke('set_onboarding_added_dictionary_word'),
        ])
            .then(() => {
                setOnboardingCongratsPending(true);
                refresh();
            })
            .catch(() => {});
    };

    return {
        doneCount,
        isCompleted,
        showCongrats,
        completeAndDismiss,
    };
};
