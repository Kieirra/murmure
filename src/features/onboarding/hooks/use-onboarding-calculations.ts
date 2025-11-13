import {
    isOnboardingCongratsPending,
    setOnboardingCongratsPending,
} from '../store/onboarding-session';
import { invoke } from '@tauri-apps/api/core';
import { OnboardingState } from './use-onboarding-state';
import { isOnboardingCompleted } from '../onboarding.helpers';

export const useOnboardingCalculations = (
    state: OnboardingState,
    refresh: () => void
) => {
    const doneCount =
        Number(state.used_home_shortcut) +
        Number(state.transcribed_outside_app) +
        Number(state.added_dictionary_word);

    const isCompleted = isOnboardingCompleted(state);

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
            .catch((error) => {
                console.error('Failed to complete onboarding:', error);
            });
    };

    return {
        doneCount,
        isCompleted,
        showCongrats,
        completeAndDismiss,
    };
};
