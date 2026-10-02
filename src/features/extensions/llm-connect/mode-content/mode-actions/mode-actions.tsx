import { type ReactNode } from 'react';
import clsx from 'clsx';
import { Mic, PenLine } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { RenderKeys } from '@/components/render-keys';
import {
    useShortcut,
    LLM_MODE_SHORTCUT_CONFIGS,
    LLM_TRANSFORM_SHORTCUT_CONFIGS,
} from '@/features/settings/shortcuts/hooks/use-shortcut';
import { type WorkflowStep } from '../../workflow-card/workflow-card';
import { GestureItem } from './gesture-item/gesture-item';

interface ModeActionsProps {
    modeIndex: number;
}

export const ModeActions = ({ modeIndex }: ModeActionsProps) => {
    const { t } = useTranslation();
    const { shortcut: dictateShortcut } = useShortcut(LLM_MODE_SHORTCUT_CONFIGS[modeIndex]);
    const { shortcut: transformShortcut } = useShortcut(LLM_TRANSFORM_SHORTCUT_CONFIGS[modeIndex]);

    const pressStep = (shortcut: string): ReactNode => {
        if (shortcut.length === 0) {
            return t('No shortcut set.');
        }

        return (
            <>
                {t('Press ')}
                <RenderKeys keyString={shortcut} />
            </>
        );
    };

    const dictateSteps: WorkflowStep[] = [
        { id: 'press', content: pressStep(dictateShortcut) },
        { id: 'speak', content: t('Speak') },
        { id: 'result', content: t('Your text is rewritten and pasted') },
    ];

    const transformSteps: WorkflowStep[] = [
        { id: 'select', content: t('Select some text') },
        { id: 'press', content: pressStep(transformShortcut) },
        { id: 'prompt', content: t('The prompt of this tab is applied') },
        { id: 'result', content: t('Your text is replaced') },
    ];

    return (
        <div
            className={clsx(
                'flex flex-col min-[920px]:flex-row min-[920px]:items-center min-[920px]:justify-between gap-x-4 gap-y-2 w-full',
                'rounded-md border border-border bg-black/30 p-3'
            )}
        >
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
        </div>
    );
};
