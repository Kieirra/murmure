describe('About Tab', () => {
    it('should navigate to about page', async () => {
        // Wait for the app to be ready
        await $('body').waitForExist();
        const aboutTab = await $('[data-testid="about-tab"]');
        await aboutTab.click();
        await expect($('[data-testid="about-title"]')).toBeDisplayed();
        await expect(await browser.checkScreen('about-page')).toEqual(0);
    });
});
