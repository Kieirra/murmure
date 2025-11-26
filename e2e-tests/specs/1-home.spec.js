import { checkScreenWithWarning } from '../helpers/visual-helpers.js';

describe('Home Tab', () => {
    it('should navigate to homepage', async () => {
        // Wait for the app to be ready
        await $('body').waitForExist();
        const homeTab = await $('[data-testid="home-tab"]');
        await homeTab.click();
        await expect($('[data-testid="home-title"]')).toBeDisplayed();
    });

    it('should take a screenshot of the home page', async () => {
        await checkScreenWithWarning('home-page');
    });
});
