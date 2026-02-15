import {
    useShortcut,
    SHORTCUT_CONFIGS,
} from '../settings/shortcuts/hooks/use-shortcut';
import { AudioVisualizer } from './audio-visualizer/audio-visualizer';
import { History } from './history/history';
import { Page } from '@/components/page';
import { Typography } from '@/components/typography';
import { Statistics } from './statistics/statistics';
import { useTranslation } from '@/i18n';
import { Onboarding } from '../onboarding/onboarding';
import { RecordLabel } from '@/components/record-label';
import { CancelVisualizer } from '@/overlay/cancel-visualizer';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useRef, useState } from 'react';

export const Home = () => {
    const { shortcut: recordShortcut } = useShortcut(SHORTCUT_CONFIGS.record);
    const [isCancelled, setIsCancelled] = useState(false);
    const cancelTimerRef = useRef<number | null>(null);

    useEffect(() => {
        const unlistenPromise = listen('recording-cancelled', () => {
            if (cancelTimerRef.current) clearTimeout(cancelTimerRef.current);
            setIsCancelled(true);
            cancelTimerRef.current = setTimeout(() => {
                setIsCancelled(false);
                cancelTimerRef.current = null;
            }, 700);
        });
        return () => { unlistenPromise.then((unlisten) => unlisten()); };
    }, []);

    const { t } = useTranslation();
    return (
        <main className="space-y-4 relative">
            <Page.Header>
                <Typography.MainTitle className="pb-4" data-testid="home-title">
                    {t('Welcome aboard!')}
                </Typography.MainTitle>
                <Statistics className="absolute -top-4 -right-4" />
                <Onboarding recordShortcut={recordShortcut} />
            </Page.Header>

            <div className="space-y-4">
                <div className="space-y-2 flex flex-col items-center">
                    <Typography.Title>{t('Live input')}</Typography.Title>
                    <div className="rounded-md border border-zinc-700 p-2 space-y-4 relative">
                        {isCancelled ? (
                            <CancelVisualizer bars={34} rows={21} pixelWidth={12} pixelHeight={6} />
                        ) : (
                            <AudioVisualizer bars={34} rows={21} />
                        )}
                        <RecordLabel />
                    </div>
                </div>

                <div className="flex justify-center">
                    <History />
                </div>
            </div>
        </main>
    );
};
