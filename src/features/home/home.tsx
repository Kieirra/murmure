import { useShortcut, SHORTCUT_CONFIGS } from '../settings/shortcuts/hooks/use-shortcut';
import { AudioVisualizer } from './audio-visualizer/audio-visualizer';
import { History } from './history/history';
import { Page } from '@/components/page';
import { Typography } from '@/components/typography';
import { Statistics } from './statistics/statistics';
import { useTranslation } from '@/i18n';
import { Onboarding } from '../onboarding/onboarding';
import { WaylandCliOnboardingCard } from '../onboarding/wayland-cli-onboarding-card/wayland-cli-onboarding-card';
import { WaylandClipboardFallbackCard } from '../onboarding/wayland-clipboard-fallback-card/wayland-clipboard-fallback-card';
import { useOnboardingState } from '../onboarding/hooks/use-onboarding-state';
import { isOnboardingCompleted } from '../onboarding/onboarding.helpers';
import { useIsWayland } from '@/components/hooks/use-linux-session-type';
import { RecordLabel } from '@/components/record-label';
import { MicLabel } from './mic-label/mic-label';
import { MicDisconnectedBanner } from './mic-disconnected-banner/mic-disconnected-banner';

export const Home = () => {
    const { shortcut: recordShortcut } = useShortcut(SHORTCUT_CONFIGS.record);
    const { state } = useOnboardingState();
    const isWayland = useIsWayland();

    const showStatsHeader = isOnboardingCompleted(state);

    const { t } = useTranslation();
    return (
        <main className="space-y-4 relative">
            <Page.Header>
                {showStatsHeader ? (
                    <Statistics />
                ) : (
                    <>
                        <Typography.MainTitle className="pb-4" data-testid="home-title">
                            {t('Welcome aboard!')}
                        </Typography.MainTitle>
                        {!isWayland && <Onboarding recordShortcut={recordShortcut} />}
                    </>
                )}
            </Page.Header>
            {isWayland && (
                <div className="space-y-4">
                    <WaylandCliOnboardingCard />
                    <WaylandClipboardFallbackCard />
                </div>
            )}
            <MicDisconnectedBanner />

            <div className="space-y-4">
                <div className="space-y-2 flex flex-col items-center">
                    <div className="rounded-md border border-border bg-black/30 p-2 space-y-4 relative">
                        <AudioVisualizer bars={34} rows={21} />
                        <MicLabel />
                        <RecordLabel />
                    </div>
                </div>

                <div className="flex justify-center pt-4">
                    <History />
                </div>
            </div>
        </main>
    );
};
