import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { setOnboardingCongratsPending } from '../store/onboarding-session';
import { isOnboardingCompleted } from '../onboarding.helpers';

export interface OnboardingState {
    used_home_shortcut: boolean;
    transcribed_outside_app: boolean;
    added_dictionary_word: boolean;
}

const initialState: OnboardingState = {
    used_home_shortcut: true,
    transcribed_outside_app: true,
    added_dictionary_word: true,
};

export const useOnboardingState = () => {
    const [state, setState] = useState<OnboardingState>(initialState);
    const [loading, setLoading] = useState<boolean>(true);

    const refresh = async () => {
        try {
            const s = await invoke<OnboardingState>('get_onboarding_state');
            const next = s ?? initialState;
            const nextCompleted = isOnboardingCompleted(next);
            setState((prev) => {
                const prevCompleted = isOnboardingCompleted(prev);
                if (!prevCompleted && nextCompleted) {
                    setOnboardingCongratsPending(true);
                }
                return next;
            });
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        refresh();
    }, []);

    useEffect(() => {
        // Keep in sync when dictionary is updated elsewhere
        const unsubs: Array<() => void> = [];

        listen('dictionary:updated', () => {
            refresh();
        }).then((un) => unsubs.push(un));
        listen('history-updated', () => {
            refresh();
        }).then((un) => unsubs.push(un));

        return () => {
            unsubs.forEach((u) => u());
        };
    }, []);

    const markUsedHomeShortcut = async () => {
        if (state.used_home_shortcut) return;
        const next = await invoke<OnboardingState>(
            'set_onboarding_used_home_shortcut'
        );
        if (isOnboardingCompleted(next)) {
            setOnboardingCongratsPending(true);
        }
        setState(next);
    };

    const markTranscribedOutsideApp = async () => {
        if (state.transcribed_outside_app) return;
        const next = await invoke<OnboardingState>(
            'set_onboarding_transcribed_outside_app'
        );
        if (isOnboardingCompleted(next)) {
            setOnboardingCongratsPending(true);
        }
        setState(next);
    };

    return {
        state,
        loading,
        refresh,
        markUsedHomeShortcut,
        markTranscribedOutsideApp,
    };
};
