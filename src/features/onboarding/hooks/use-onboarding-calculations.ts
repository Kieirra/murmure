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
    const showCongrats = isCompleted && !state.congrats_dismissed;

    const handleDismissCongrats = async () => {
        try {
            await invoke('set_onboarding_congrats_dismissed');
            refresh();
        } catch (error) {
            console.error('Failed to dismiss congrats:', error);
        }
    };

    const completeAndDismiss = async () => {
        try {
            await Promise.all([
                invoke('set_onboarding_used_home_shortcut'),
                invoke('set_onboarding_transcribed_outside_app'),
                invoke('set_onboarding_added_dictionary_word'),
            ]);
            refresh();
        } catch (error) {
            console.error('Failed to complete onboarding:', error);
        }
    };

    return {
        doneCount,
        isCompleted,
        showCongrats,
        completeAndDismiss,
        dismissCongrats: handleDismissCongrats,
    };
};
