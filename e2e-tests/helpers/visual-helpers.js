/**
 * Performs a visual regression check on the content area only (excluding sidebar)
 * and logs a warning if differences are detected, but does not fail the test.
 *
 * @param {string} screenName - Name of the screen to check
 * @param {number} threshold - Maximum acceptable difference percentage (default: 0)
 * @returns {Promise<number>} The difference percentage
 */
export async function checkScreenWithWarning(screenName, threshold = 0) {
    const element = await $('[data-testid="murmure-content"]');
    const diff = await browser.checkElement(element, screenName, {});

    if (diff > threshold) {
        console.warn(
            `⚠️  Visual regression detected for "${screenName}": ${diff}% difference (threshold: ${threshold}%)`
        );
    }

    return diff;
}
