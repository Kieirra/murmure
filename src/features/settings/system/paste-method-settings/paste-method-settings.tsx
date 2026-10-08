import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { ClipboardPaste } from 'lucide-react';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/select';
import { useTranslation } from '@/i18n';
import { useIsWayland } from '@/components/hooks/use-linux-session-type';
import { PasteMethod, usePasteMethodState } from './hooks/use-paste-method-state';
import { useLayoutFallback } from './hooks/use-layout-fallback';

const PASTE_METHODS: { key: PasteMethod; label: string }[] = [
    { key: 'ctrl_v', label: 'Standard (Ctrl+V)' },
    { key: 'ctrl_shift_v', label: 'Terminal (Ctrl+Shift+V)' },
    { key: 'direct', label: 'Direct (type text)' },
];

export const PasteMethodSettings = () => {
    const { t } = useTranslation();
    const { pasteMethod, setPasteMethod } = usePasteMethodState();
    const isWayland = useIsWayland();
    const { isFallback } = useLayoutFallback();

    const showFallbackBadge = isWayland && pasteMethod === 'direct' && isFallback;

    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title className="flex items-center gap-2">
                    <ClipboardPaste className="w-4 h-4 text-muted-foreground" />
                    {t('Text insertion mode')}
                </Typography.Title>
                <Typography.Paragraph>
                    {t('Choose how transcriptions are inserted into applications.')}
                    <ul className="list-disc pl-6 text-xs">
                        <li>
                            <span className="font-bold text-sky-400">{t('Standard: ')}</span>
                            {t('Fast and default option. Works in most applications, but not in terminals.')}
                        </li>
                        <li>
                            <span className="font-bold text-sky-400">{t('Terminal: ')}</span>
                            {t(
                                'Designed for terminal applications. May conflict with some software (e.g. LibreOffice).'
                            )}
                        </li>
                        <li>
                            <span className="font-bold text-sky-400">{t('Direct: ')}</span>
                            {t('Types text character by character. Works everywhere, but is the slowest option.')}
                        </li>
                    </ul>
                </Typography.Paragraph>
                {showFallbackBadge ? (
                    <p className="text-xs text-yellow-400">
                        {t(
                            'Keyboard layout could not be detected. To avoid mistyped characters, switch back to Ctrl+V.'
                        )}
                    </p>
                ) : null}
            </SettingsUI.Description>
            <Select value={pasteMethod} onValueChange={setPasteMethod}>
                <SelectTrigger className="w-[200px]" data-testid="paste-method-select">
                    <SelectValue />
                </SelectTrigger>
                <SelectContent>
                    {PASTE_METHODS.map((method) => {
                        // Direct under Wayland is flagged experimental
                        // until we have wider compositor coverage.
                        const isExperimental = isWayland && method.key === 'direct';
                        return (
                            <SelectItem key={method.key} value={method.key}>
                                {t(method.label)}
                                {isExperimental ? (
                                    <span className="ml-2 text-xs text-yellow-400">
                                        ({t('Experimental')})
                                    </span>
                                ) : null}
                            </SelectItem>
                        );
                    })}
                </SelectContent>
            </Select>
        </SettingsUI.Item>
    );
};
