let congratsPending = false;

export function setOnboardingCongratsPending(value: boolean) {
    congratsPending = value;
}

export function isOnboardingCongratsPending(): boolean {
    return congratsPending;
}
