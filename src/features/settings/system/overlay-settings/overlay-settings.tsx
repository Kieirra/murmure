import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { Eye, Ruler } from 'lucide-react';
import { useOverlayState } from './hooks/use-overlay-state';
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from '@/components/select';
import { useTranslation } from '@/i18n';

export const OverlaySettings = () => {
    const { overlayMode, setOverlayMode, overlayPosition, setOverlayPosition } =
        useOverlayState();
    const { t } = useTranslation('settings');

    return (
        <>
            <SettingsUI.Item>
                <SettingsUI.Description>
                    <Typography.Title className="flex items-center gap-2">
                        <Eye className="w-4 h-4 text-zinc-400" />
                        {t('system.overlay.visibility.title')}
                    </Typography.Title>
                    <Typography.Paragraph>
                        {t('system.overlay.visibility.description')}
                    </Typography.Paragraph>
                </SettingsUI.Description>

                <div className="flex gap-2">
                    <Select value={overlayMode} onValueChange={setOverlayMode}>
                        <SelectTrigger className="w-[150px]">
                            <SelectValue placeholder={t('system.overlay.visibility.placeholder')} />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem value="hidden">
                                {t('system.overlay.visibility.modes.hidden')}
                            </SelectItem>
                            <SelectItem value="recording">
                                {t('system.overlay.visibility.modes.recording')}
                            </SelectItem>
                            <SelectItem value="always">
                                {t('system.overlay.visibility.modes.always')}
                            </SelectItem>
                        </SelectContent>
                    </Select>
                </div>
            </SettingsUI.Item>
            <SettingsUI.Separator />
            <SettingsUI.Item>
                <SettingsUI.Description>
                    <Typography.Title className="flex items-center gap-2">
                        <Ruler className="w-4 h-4 text-zinc-400" />
                        {t('system.overlay.position.title')}
                    </Typography.Title>
                    <Typography.Paragraph>
                        {t('system.overlay.position.description')}
                    </Typography.Paragraph>
                </SettingsUI.Description>
                <div className="flex gap-2">
                    <Select
                        value={overlayPosition}
                        onValueChange={setOverlayPosition}
                    >
                        <SelectTrigger className="w-[150px]">
                            <SelectValue placeholder={t('system.overlay.position.placeholder')} />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem value="top">
                                {t('system.overlay.position.top')}
                            </SelectItem>
                            <SelectItem value="bottom">
                                {t('system.overlay.position.bottom')}
                            </SelectItem>
                        </SelectContent>
                    </Select>
                </div>
            </SettingsUI.Item>
        </>
    );
};
