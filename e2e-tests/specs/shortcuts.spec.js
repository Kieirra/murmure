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
        
        const pasteTranscriptButton = await $('[data-testid="paste-transcript-button"]');
        await expect(pasteTranscriptButton).toBeDisplayed();
        
        await expect(
            await browser.checkScreen('shortcuts-page')
        ).toEqual(0);
    });
});
