import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { ClipboardPaste } from 'lucide-react';
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from '@/components/select';
import { useTranslation } from '@/i18n';
import {
    PasteMethod,
    usePasteMethodState,
} from './hooks/use-paste-method-state';

const PASTE_METHODS: { key: PasteMethod; label: string }[] = [
    { key: 'ctrl_v', label: 'Standard (Ctrl+V)' },
    { key: 'ctrl_shift_v', label: 'Terminal (Ctrl+Shift+V)' },
    { key: 'direct', label: 'Direct (type text)' },
];

export const PasteMethodSettings = () => {
    const { t } = useTranslation();
    const { pasteMethod, setPasteMethod } = usePasteMethodState();

    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title className="flex items-center gap-2">
                    <ClipboardPaste className="w-4 h-4 text-zinc-400" />
                    {t('Text insertion mode')}
                </Typography.Title>
                <Typography.Paragraph>
                    {t(
                        'Choose how transcriptions are inserted into applications.'
                    )}
                    <ul className="list-disc pl-6 text-xs">
                        <li>
                            <span className="font-bold text-sky-400">
                                {t('Standard: ')}
                            </span>
                            {t(
                                'Fast and default option. Works in most applications, but not in terminals.'
                            )}
                        </li>
                        <li>
                            <span className="font-bold text-sky-400">
                                {t('Terminal: ')}
                            </span>
                            {t(
                                'Designed for terminal applications. May conflict with some software (e.g. LibreOffice, VS Code).'
                            )}
                        </li>
                        <li>
                            <span className="font-bold text-sky-400">
                                {t('Direct: ')}
                            </span>
                            {t(
                                'Types text character by character. Works everywhere, but is the slowest option.'
                            )}
                        </li>
                    </ul>
                </Typography.Paragraph>
            </SettingsUI.Description>
            <Select value={pasteMethod} onValueChange={setPasteMethod}>
                <SelectTrigger
                    className="w-[200px]"
                    data-testid="paste-method-select"
                >
                    <SelectValue />
                </SelectTrigger>
                <SelectContent>
                    {PASTE_METHODS.map((method) => (
                        <SelectItem key={method.key} value={method.key}>
                            {t(method.label)}
                        </SelectItem>
                    ))}
                </SelectContent>
            </Select>
        </SettingsUI.Item>
    );
};
