import { useShortcut, SHORTCUT_CONFIGS } from '../settings/shortcuts/hooks/use-shortcut';
import { AudioVisualizer } from './audio-visualizer/audio-visualizer';
import { History } from './history/history';
import { Page } from '@/components/page';
import { Statistics } from './statistics/statistics';
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

    const showOnboarding = !isOnboardingCompleted(state) && !isWayland;

    return (
        <main className="space-y-4 relative">
            <Page.Header className="space-y-4">
                <Statistics />
                {showOnboarding && <Onboarding recordShortcut={recordShortcut} />}
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
                    <div className="w-full rounded-md border border-border bg-background p-2 space-y-4 relative">
                        <AudioVisualizer className="h-44" />
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
