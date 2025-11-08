import { RenderKeys } from '@/components/render-keys';
import { useRecordShortcutState } from '../settings/shortcuts/hooks/use-record-shortcut-state';
import { AudioVisualizer } from './audio-visualizer/audio-visualizer';
import { History } from './history/history';
import { Page } from '@/components/page';
import { Typography } from '@/components/typography';
import { Statistics } from './statistics/statistics';

export const Home = () => {
    const { recordShortcut } = useRecordShortcutState();
    return (
        <main className="space-y-8 relative">
            <Page.Header>
                <Typography.MainTitle className="pb-4">
                    Welcome aboard!
                </Typography.MainTitle>
                <Statistics className="absolute -top-4 -right-4" />
                <Typography.Paragraph className="text-zinc-400">
                    Murmure use default microphone to record your voice.
                </Typography.Paragraph>
            </Page.Header>

            <div className="space-y-4">
                <div className="space-y-2 flex flex-col items-center">
                    <Typography.Title>Live input</Typography.Title>
                    <div className="rounded-md border border-zinc-700 p-2 space-y-4 relative">
                        <AudioVisualizer bars={34} rows={21} />
                        <Typography.Paragraph className="text-xs absolute bottom-2 left-2">
                            Hold <RenderKeys keyString={recordShortcut} /> to
                            record
                        </Typography.Paragraph>
                    </div>
                </div>

                <div className="flex justify-center">
                    <History />
                </div>
            </div>
        </main>
    );
};
