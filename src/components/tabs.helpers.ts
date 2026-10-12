export const getAdjacentTabIndex = (currentIndex: number, key: string, tabCount: number): number | undefined => {
    switch (key) {
        case 'ArrowRight':
            return (currentIndex + 1) % tabCount;
        case 'ArrowLeft':
            return (currentIndex - 1 + tabCount) % tabCount;
        default:
            return undefined;
    }
};
