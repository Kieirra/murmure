import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { Switch } from '@/components/switch';
import { Clipboard } from 'lucide-react';
import { useCopyToClipboardState } from './hooks/use-copy-to-clipboard-state';

export const CopyToClipboardSettings = () => {
    const { copyToClipboard, setCopyToClipboard } = useCopyToClipboardState();

    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title className="flex items-center gap-2">
                    <Clipboard className="w-4 h-4 text-zinc-400" />
                    Copy to Clipboard
                </Typography.Title>
                <Typography.Paragraph>
                    Automatically copy transcriptions to the clipboard when recording finishes.
                </Typography.Paragraph>
            </SettingsUI.Description>
            <Switch checked={copyToClipboard} onCheckedChange={setCopyToClipboard} />
        </SettingsUI.Item>
    );
};