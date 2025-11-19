describe('Dictionary Tab', () => {
    it('should navigate to the dictionary tab', async () => {
        // Wait for the app to be ready
        await $('body').waitForExist();
        const dictionaryTab = await $('[data-testid="dictionary-tab"]');
        await dictionaryTab.click();
        await expect($('[data-testid="dictionary-title"]')).toBeDisplayed();
        await expect(
            await browser.checkScreen('custom-dictionary-page')
        ).toEqual(0);
    });

    it('should add a word to the dictionary', async () => {
        const input = await $('[data-testid="custom-dictionary-input"]');
        await input.setValue('test');
        const addButton = await $(
            '[data-testid="custom-dictionary-add-button"]'
        );
        await addButton.click();

        const word = await $('[data-testid="custom-dictionary-word-test"]');
        await expect(await word.getText()).toBe('test');
    });

    it('should remove a word from the dictionary', async () => {
        const removeWordButton = await $(
            '[data-testid="custom-dictionary-remove-button-test"]'
        );
        await removeWordButton.click();
        const words = await $$('[data-testid="custom-dictionary-word-test"]');
        await expect(words).toHaveLength(0);
    });
});
