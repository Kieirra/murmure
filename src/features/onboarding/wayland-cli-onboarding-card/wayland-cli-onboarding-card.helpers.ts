export const WAYLAND_CLI_SETUP_COMMAND = 'murmure --transcription';

// Paths kept in English on purpose: these are the actual labels shipped by
// each desktop environment in its default English locale. Translating them
// would mismatch what the user sees on screen.
export const DESKTOP_ENV_SHORTCUT_PATH: Record<string, string | null> = {
    gnome: 'Settings → Keyboard → Keyboard Shortcuts',
    kde: 'System Settings → Shortcuts → Custom Shortcuts',
    cinnamon: 'System Settings → Keyboard → Shortcuts → Custom Shortcuts',
    xfce: 'Settings → Keyboard → Application Shortcuts',
    mate: 'System → Preferences → Keyboard Shortcuts',
    hyprland: null,
    sway: null,
    i3: null,
    other: null,
};
