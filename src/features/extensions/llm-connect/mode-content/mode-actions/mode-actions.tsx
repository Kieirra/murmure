import { type ReactNode } from 'react';
import { Mic, PenLine, Zap } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { RenderKeys } from '@/components/render-keys';
import {
    useShortcut,
    SHORTCUT_CONFIGS,
    LLM_MODE_SHORTCUT_CONFIGS,
    LLM_TRANSFORM_SHORTCUT_CONFIGS,
} from '@/features/settings/shortcuts/hooks/use-shortcut';
import { GestureItem } from './gesture-item/gesture-item';

interface ModeActionsProps {
    modeIndex: number;
}

export const ModeActions = ({ modeIndex }: ModeActionsProps) => {
    const { t } = useTranslation();
    const { shortcut: dictateShortcut } = useShortcut(LLM_MODE_SHORTCUT_CONFIGS[modeIndex]);
    const { shortcut: transformShortcut } = useShortcut(LLM_TRANSFORM_SHORTCUT_CONFIGS[modeIndex]);
    const { shortcut: commandShortcut } = useShortcut(SHORTCUT_CONFIGS.command);

    const pressStep = (shortcut: string): ReactNode => (
        <>
            {t('Press ')}
            <RenderKeys keyString={shortcut} />
        </>
    );

    const dictateSteps: ReactNode[] = [pressStep(dictateShortcut), t('Speak'), t('Your text is rewritten and pasted')];

    const transformSteps: ReactNode[] = [
        t('Select some text'),
        pressStep(transformShortcut),
        t('The prompt of this tab is applied'),
        t('Your text is replaced'),
    ];

    const commandSteps: ReactNode[] = [
        t('Select some text'),
        pressStep(commandShortcut),
        <div className="space-y-1">
            <div>{t('Speak the command')}</div>
            <div className="text-sm text-muted-foreground italic">{t('e.g. "Translate to English"')}</div>
        </div>,
        t('Your text is replaced'),
    ];

    return (
        <div className="grid grid-cols-1 min-[920px]:grid-cols-[max-content_max-content] items-center gap-x-4 gap-y-2 w-full">
            <GestureItem
                icon={Mic}
                label={t('Dictate')}
                shortcut={dictateShortcut}
                benefit={t('I speak, the model rewrites my transcription.')}
                steps={dictateSteps}
            />
            <GestureItem
                icon={PenLine}
                label={t('Transform')}
                shortcut={transformShortcut}
                benefit={t('I select text, the model replaces it.')}
                steps={transformSteps}
            />
            <GestureItem
                icon={Zap}
                label={t('Command')}
                shortcut={commandShortcut}
                benefit={t('I select text and speak the instruction to apply.')}
                steps={commandSteps}
            />
        </div>
    );
};
