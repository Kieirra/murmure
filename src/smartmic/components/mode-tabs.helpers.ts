import type { ViewMode } from '../types';
import type { StringKey } from '../i18n/strings';

export const TABS: { mode: ViewMode; labelKey: StringKey }[] = [
    { mode: 'remote', labelKey: 'tabs.remote' },
    { mode: 'transcription', labelKey: 'tabs.transcription' },
    { mode: 'translation', labelKey: 'tabs.translation' },
];
