import { checkScreenWithWarning } from '../helpers/visual-helpers.js';

describe('LLM Connect', () => {
    it('should navigate to LLM Connect settings', async () => {
        // Wait for the app to be ready
        await $('body').waitForExist();
        const llmConnectTab = await $('[data-testid="llm-connect-tab"]');
        await llmConnectTab.click();

        const header = await $('h1');
        await expect(header).toBeDisplayed();
    });

    it('should take a screenshot of the LLM Connect page', async () => {
        await checkScreenWithWarning('llm-connect-page');
    });

    it('should enable LLM Connect and show settings', async () => {
        const switchElement = await $('[data-testid="llm-connect-switch"]');
        await switchElement.click();
        await expect(
            $('[data-testid="llm-connect-url-input"]')
        ).toBeDisplayed();
        await expect(
            $('[data-testid="llm-connect-model-select"]')
        ).toBeDisplayed();
        await expect(
            $('[data-testid="llm-connect-prompt-textarea"]')
        ).toBeDisplayed();
        await checkScreenWithWarning('llm-connect-enabled', 1);
    });
});
