describe('System Tab', () => {
    it('should navigate to the system tab', async () => {
        // Wait for the app to be ready
        await $('body').waitForExist();
        const systemTab = await $('[data-testid="system-tab"]');
        await systemTab.click();
        await expect($('[data-testid="system-title"]')).toBeDisplayed();

        // Check for language selector
        const languageSelect = await $('[data-testid="language-select"]');
        await expect(languageSelect).toBeDisplayed();

        await expect(
            await browser.checkScreen('system-page')
        ).toEqual(0);
    });
});
