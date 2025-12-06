import { checkScreenWithWarning } from '../helpers/visual-helpers.js';

describe('LLM Connect', () => {
    it('should navigate to LLM Connect settings', async () => {
        // Wait for the app to be ready
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

    // it('should test ollama connection', async () => {
    //     await checkScreenWithWarning('llm-connect-install');
    //     const testBtn = await $('[data-testid="llm-connect-test-button"]');
    //     await testBtn.click();

    //     await setTimeout(500);
    //     const nextBtn = await $('[data-testid="llm-connect-next-button"]');
    //     await nextBtn.click();
    // });

    // it('should select model', async () => {
    //     await checkScreenWithWarning('llm-connect-model');
    //     const modelBtn = await $(
    //         '[data-testid="llm-connect-model-card-button-recommended"]'
    //     );
    //     await modelBtn.click();
    //     await setTimeout(500);

    //     const nextBtn = await $('[data-testid="llm-connect-next-button"]');
    //     await nextBtn.click();
    // });

    // it('should validate configuration', async () => {
    //     await checkScreenWithWarning('llm-connect-success');
    //     const nextBtn = await $('[data-testid="llm-connect-success-button"]');
    //     await nextBtn.click();
    // });

    // it('should enable LLM Connect and show settings', async () => {
    //     await checkScreenWithWarning('llm-connect-page');
    //     await expect(
    //         $('[data-testid="llm-connect-url-input"]')
    //     ).toBeDisplayed();
    //     await expect(
    //         $('[data-testid="llm-connect-model-select"]')
    //     ).toBeDisplayed();
    //     await expect(
    //         $('[data-testid="llm-connect-prompt-textarea"]')
    //     ).toBeDisplayed();
    //     await checkScreenWithWarning('llm-connect-enabled', 1);
    // });
});
