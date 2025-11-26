import { checkScreenWithWarning } from '../helpers/visual-helpers.js';

describe('Shortcuts Tab', () => {
    it('should navigate to the shortcuts tab', async () => {
        // Wait for the app to be ready
        await $('body').waitForExist();
        const shortcutsTab = await $('[data-testid="shortcuts-tab"]');
        await shortcutsTab.click();
        await expect($('[data-testid="shortcuts-title"]')).toBeDisplayed();

        // Check for shortcut buttons
        const pushToTalkButton = await $('[data-testid="push-to-talk-button"]');
        await expect(pushToTalkButton).toBeDisplayed();

        const pasteTranscriptButton = await $(
            '[data-testid="paste-transcript-button"]'
        );
        await expect(pasteTranscriptButton).toBeDisplayed();
    });

    it('should take a screenshot of the shortcuts page', async () => {
        await checkScreenWithWarning('shortcuts-page');
    });
});
