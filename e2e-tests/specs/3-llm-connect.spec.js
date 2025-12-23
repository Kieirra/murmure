import { checkScreenWithWarning } from '../helpers/visual-helpers.js';

describe('LLM Connect', () => {
    it('should navigate to LLM Connect settings', async () => {
        await $('body').waitForExist();
        const personalizeTab = await $('[data-testid="personalize-tab"]');
        await personalizeTab.click();
        const llmConnectTab = await $('[data-testid="llm-connect-tab"]');
        await llmConnectTab.click();

        const header = await $('h1');
        await expect(header).toBeDisplayed();
    });

    it('should start configuration', async () => {
        await checkScreenWithWarning('llm-connect-intro');
        const startBtn = await $('[data-testid="llm-connect-start-button"]');
        await startBtn.click();
    });
});
