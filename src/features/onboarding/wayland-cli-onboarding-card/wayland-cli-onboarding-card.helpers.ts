export const WAYLAND_CLI_SETUP_COMMAND = 'murmure --transcription';

export const DEFAULT_SHORTCUT_PATH = 'Settings → Keyboard → Keyboard Shortcuts';

export const DESKTOP_ENV_SHORTCUT_PATH: Record<string, string> = {
    gnome: 'Settings → Keyboard → Keyboard Shortcuts',
    kde: 'System Settings → Shortcuts → Custom Shortcuts',
    cinnamon: 'System Settings → Keyboard → Shortcuts → Custom Shortcuts',
    xfce: 'Settings → Keyboard → Application Shortcuts',
    mate: 'System → Preferences → Keyboard Shortcuts',
};
