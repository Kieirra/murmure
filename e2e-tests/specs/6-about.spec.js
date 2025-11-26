import { checkScreenWithWarning } from '../helpers/visual-helpers.js';

describe('About Tab', () => {
    it('should navigate to about page', async () => {
        // Wait for the app to be ready
        await $('body').waitForExist();
        const aboutTab = await $('[data-testid="about-tab"]');
        await aboutTab.click();
        await expect($('[data-testid="about-title"]')).toBeDisplayed();
    });

    it('should take a screenshot of the about page', async () => {
        await checkScreenWithWarning('about-page');
    });
});
