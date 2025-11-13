/**
 * Manages onboarding session state to show congrats message when tutorial is first completed.
 * State resets on app restart (in-memory, not persisted).
 */

let congratsPending = false;

export function setOnboardingCongratsPending(value: boolean) {
    congratsPending = value;
}

export function isOnboardingCongratsPending(): boolean {
    return congratsPending;
}
