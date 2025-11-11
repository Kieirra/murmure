import { useMemo } from 'react';
import { Typography } from '@/components/typography';
import { useTranslation } from '@/i18n';
import { Checkbox } from '@/components/checkbox';
import clsx from 'clsx';
import { BadgeCheck, X } from 'lucide-react';
import { invoke } from '@tauri-apps/api/core';
import { useOnboardingState } from '../hooks/use-onboarding-state';
import {
    isOnboardingCongratsPending,
    setOnboardingCongratsPending,
} from '../onboarding-session';

type OnboardingItemProps = {
    done: boolean;
    label: string;
    description?: string;
    onToggle?: () => void;
};

const OnboardingItem = ({
    done,
    label,
    description,
    onToggle,
}: OnboardingItemProps) => {
    return (
        <li className="flex items-center gap-4 py-1">
            <span
                className={
                    'transition-transform duration-200 ' +
                    (done ? 'scale-100' : 'scale-75 opacity-50')
                }
            >
                <Checkbox
                    checked={done}
                    onCheckedChange={onToggle}
                    className={clsx(
                        'cursor-pointer',
                        'scale-115',
                        'data-[state=checked]:border-sky-400',
                        'data-[state=checked]:bg-sky-400',
                        'data-[state=checked]:text-white'
                    )}
                />
            </span>
            <span
                className={done ? 'text-zinc-300 line-through opacity-30!' : ''}
            >
                {label}
                {description && (
                    <Typography.Paragraph className="text-zinc-400 text-xs italic">
                        {description}
                    </Typography.Paragraph>
                )}
            </span>
        </li>
    );
};

export const OnboardingBanner = ({
    recordShortcut,
}: {
    recordShortcut?: string;
}) => {
    const { t } = useTranslation();
    const { state, refresh } = useOnboardingState();

    const completeAndDismiss = () => {
        Promise.all([
            invoke('set_onboarding_used_home_shortcut'),
            invoke('set_onboarding_transcribed_outside_app'),
            invoke('set_onboarding_added_dictionary_word'),
        ])
            .then(() => {
                setOnboardingCongratsPending(true);
                refresh();
            })
            .catch(() => {});
    };

    const doneCount = useMemo(
        () =>
            Number(state.used_home_shortcut) +
            Number(state.transcribed_outside_app) +
            Number(state.added_dictionary_word),
        [
            state.used_home_shortcut,
            state.transcribed_outside_app,
            state.added_dictionary_word,
        ]
    );

    const isCompleted =
        state.used_home_shortcut &&
        state.transcribed_outside_app &&
        state.added_dictionary_word;

    const showCongrats = isOnboardingCongratsPending();

    if (isCompleted) {
        if (!showCongrats)
            return (
                <Typography.Paragraph className="text-zinc-400">
                    {t('Murmure use default microphone to record your voice.')}
                </Typography.Paragraph>
            );
        return (
            <div className="rounded-md border border-sky-500 bg-sky-900/20 p-3 relative">
                <div className="flex items-center gap-2 justify-between">
                    <Typography.Paragraph className="text-sky-300! font-bold flex gap-2 items-center">
                        <BadgeCheck />
                        {t('You are now ready to use Murmure everywhere.')}
                    </Typography.Paragraph>
                    <button
                        type="button"
                        onClick={() => setOnboardingCongratsPending(false)}
                        aria-label={t('Close')}
                        className="text-zinc-400 hover:text-zinc-200"
                    >
                        <X className="w-4 h-4 cursor-pointer" />
                    </button>
                </div>
            </div>
        );
    }

    return (
        <div className="rounded-md border border-sky-500 bg-sky-900/20 p-4 space-y-2 relative">
            <div className="absolute top-2 right-2 flex">
                <Typography.Paragraph className="text-sky-300! font-bold">
                    {doneCount}/3
                </Typography.Paragraph>
                <button
                    type="button"
                    onClick={completeAndDismiss}
                    aria-label={t('Cancel')}
                    className=" text-zinc-400 hover:text-zinc-200 px-2 p-0.5"
                >
                    <X className="w-4 h-4 cursor-pointer" />
                </button>
            </div>
            <ul className="text-sm">
                <OnboardingItem
                    done={state.used_home_shortcut}
                    label={
                        recordShortcut != null
                            ? t(
                                  'To test transcription, press "{{recordShortcut}}", talk, then release',
                                  {
                                      recordShortcut,
                                  }
                              )
                            : t('Use the record shortcut on the Home page')
                    }
                    description={t(
                        'Murmure use the default microphone to record your voice. Make sure your microphone is well set up.'
                    )}
                />
                <OnboardingItem
                    done={state.transcribed_outside_app}
                    label={t('Use murmure in another app')}
                    description={t(
                        'Place your cursor in any textbox of any software and try to transcribe your voice.'
                    )}
                />
                <OnboardingItem
                    done={state.added_dictionary_word}
                    label={t('Add a word to the Custom Dictionary')}
                    description={t(
                        'Go to Settings > Custom Dictionary and add a word to make it available for future transcriptions.'
                    )}
                />
            </ul>
        </div>
    );
};
