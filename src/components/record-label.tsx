import { RenderKeys } from '@/components/render-keys.tsx';
import { Typography } from '@/components/typography.tsx';
import { useTranslation } from '@/i18n';
import { useRecordModeState } from '@/features/settings/system/record-mode-settings/hooks/use-record-mode-state.ts';
import {
    useShortcut,
    SHORTCUT_CONFIGS,
} from '@/features/settings/shortcuts/hooks/use-shortcut.ts';

export const RecordLabel = () => {
    const { recordMode } = useRecordModeState();
    const { shortcut: recordShortcut } = useShortcut(SHORTCUT_CONFIGS.record);
    const { t } = useTranslation();

    return (
        <Typography.Paragraph className="text-xs absolute bottom-2 left-2">
            {recordMode === 'push_to_talk' ? (
                <>
                    {t('Hold')} <RenderKeys keyString={recordShortcut} />
                    {t(' to record')}
                </>
            ) : (
                <>
                    {t('Toggle')} <RenderKeys keyString={recordShortcut} />
                    {t(' to start/stop recording')}
                </>
            )}
        </Typography.Paragraph>
    );
};
