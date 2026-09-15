export const MIN_RESULT_PANEL_SECS = 2;
export const MAX_RESULT_PANEL_SECS = 15;

export const clampResultPanelDurationSecs = (secs: number) =>
    Math.min(MAX_RESULT_PANEL_SECS, Math.max(MIN_RESULT_PANEL_SECS, secs));
