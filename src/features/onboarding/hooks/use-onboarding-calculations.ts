import { useState, useEffect } from 'react';
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
    const [showCongrats, setShowCongrats] = useState<boolean>(() =>
        isOnboardingCongratsPending()
    );

    useEffect(() => {
        if (isOnboardingCompleted(state)) {
            const currentValue = isOnboardingCongratsPending();
            if (currentValue && !showCongrats) {
                setShowCongrats(true);
            }
        }
    }, [state]);

    const doneCount =
        Number(state.used_home_shortcut) +
        Number(state.transcribed_outside_app) +
        Number(state.added_dictionary_word);

    const isCompleted = isOnboardingCompleted(state);

    const handleDismissCongrats = () => {
        setOnboardingCongratsPending(false);
        setShowCongrats(false);
    };

    const completeAndDismiss = () => {
        Promise.all([
            invoke('set_onboarding_used_home_shortcut'),
            invoke('set_onboarding_transcribed_outside_app'),
            invoke('set_onboarding_added_dictionary_word'),
        ])
            .then(() => {
                setOnboardingCongratsPending(true);
                setShowCongrats(true);
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
        dismissCongrats: handleDismissCongrats,
    };
};
