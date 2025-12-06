import { checkScreenWithWarning } from '../helpers/visual-helpers.js';

describe('Dictionary Tab', () => {
    it('should navigate to the dictionary tab', async () => {
        // Wait for the app to be ready
        await $('body').waitForExist();
        const personalizeTab = await $('[data-testid="personalize-tab"]');
        await personalizeTab.click();
        const dictionaryTab = await $('[data-testid="dictionary-tab"]');
        await dictionaryTab.click();
        await expect($('[data-testid="dictionary-title"]')).toBeDisplayed();
    });

    it('should take a screenshot of the custom dictionary page', async () => {
        await checkScreenWithWarning('custom-dictionary-page');
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
