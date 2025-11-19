describe('Dictionary Tab', () => {
    it('should navigate to homepage', async () => {
        // Wait for the app to be ready
        await $('body').waitForExist();
        const homeTab = await $('[data-testid="about-tab"]');
        await homeTab.click();
        await expect($('[data-testid="about-title"]')).toBeDisplayed();
        await browser.checkScreen('about-page');
    });
});
