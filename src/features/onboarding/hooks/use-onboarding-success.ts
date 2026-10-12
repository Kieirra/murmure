import { useEffect, useState } from 'react';
import { getNextOnboardingSuccessPhase, OnboardingSuccessPhase } from '../onboarding.helpers';

const SUCCESS_DURATION_MS = 6000;
const EXIT_DURATION_MS = 300;

interface UseOnboardingSuccessParams {
    isCompleted: boolean;
    isCongratsDismissed: boolean;
    loading: boolean;
    completeAndDismiss: () => Promise<void>;
    dismissCongrats: () => Promise<void>;
}

export const useOnboardingSuccess = ({
    isCompleted,
    isCongratsDismissed,
    loading,
    completeAndDismiss,
    dismissCongrats,
}: UseOnboardingSuccessParams) => {
    const [phase, setPhase] = useState<OnboardingSuccessPhase>('idle');

    useEffect(() => {
        if (loading) {
            return;
        }
        setPhase((current) => getNextOnboardingSuccessPhase(current, isCompleted, isCongratsDismissed));
    }, [loading, isCompleted, isCongratsDismissed]);

    useEffect(() => {
        if (phase !== 'success') {
            return;
        }
        const timer = setTimeout(() => setPhase('leaving'), SUCCESS_DURATION_MS);
        return () => clearTimeout(timer);
    }, [phase]);

    useEffect(() => {
        if (phase !== 'leaving') {
            return;
        }
        const timer = setTimeout(closeSuccess, EXIT_DURATION_MS);
        return () => clearTimeout(timer);
    }, [phase]);

    const closeSuccess = () => {
        setPhase('done');
        void dismissCongrats();
    };

    const skipOnboarding = () => {
        setPhase('done');
        void completeAndDismiss();
    };

    return {
        isSuccessVisible: phase === 'success' || phase === 'leaving',
        isSuccessLeaving: phase === 'leaving',
        closeSuccess,
        skipOnboarding,
    };
};
