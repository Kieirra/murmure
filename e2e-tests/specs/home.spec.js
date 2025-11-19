describe('Dictionary Tab', () => {
    it('should navigate to homepage', async () => {
        // Wait for the app to be ready
        await $('body').waitForExist();
        const homeTab = await $('[data-testid="home-tab"]');
        await homeTab.click();
        await expect($('[data-testid="home-title"]')).toBeDisplayed();
        await expect(await browser.checkScreen('home-page')).toEqual(0);
    });
});
