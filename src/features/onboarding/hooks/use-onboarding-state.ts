import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { setOnboardingCongratsPending } from '../store/onboarding-session';

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
            const nextCompleted =
                next.used_home_shortcut &&
                next.transcribed_outside_app &&
                next.added_dictionary_word;
            setState((prev) => {
                const prevCompleted =
                    prev.used_home_shortcut &&
                    prev.transcribed_outside_app &&
                    prev.added_dictionary_word;
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
        const nextCompleted =
            next.used_home_shortcut &&
            next.transcribed_outside_app &&
            next.added_dictionary_word;
        if (nextCompleted) {
            setOnboardingCongratsPending(true);
        }
        setState(next);
    };

    const markTranscribedOutsideApp = async () => {
        if (state.transcribed_outside_app) return;
        const next = await invoke<OnboardingState>(
            'set_onboarding_transcribed_outside_app'
        );
        const nextCompleted =
            next.used_home_shortcut &&
            next.transcribed_outside_app &&
            next.added_dictionary_word;
        if (nextCompleted) {
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
