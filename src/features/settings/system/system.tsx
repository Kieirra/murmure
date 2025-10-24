import { Switch } from '../../../components/switch';
import { Eye, Power, Ruler, Zap } from 'lucide-react';
import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { useStartOnBootState } from './hooks/use-auto-start-state';
import { Page } from '@/components/page';
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from '@/components/select';
import { useOverlayState } from './hooks/use-overlay-state';
import { useApiState } from './hooks/use-api-state';
import { Input } from '@/components/input';

export const System = () => {
    const { startOnBoot, setStartOnBoot } = useStartOnBootState();
    const { overlayMode, setOverlayMode, overlayPosition, setOverlayPosition } =
        useOverlayState();
    const { apiEnabled, setApiEnabled, apiPort, setApiPort } = useApiState();

    return (
        <main>
            <div className="space-y-8">
                <Page.Header>
                    <Typography.MainTitle>System</Typography.MainTitle>
                    <Typography.Paragraph className="text-zinc-400">
                        Adjust system preferences to control Murmure's behavior
                        at startup and more.
                    </Typography.Paragraph>
                </Page.Header>

                <div className="flex justify-center">
                    <SettingsUI.Container>
                        <SettingsUI.Item>
                            <SettingsUI.Description>
                                <Typography.Title className="flex items-center gap-2">
                                    <Power className="w-4 h-4 text-zinc-400" />
                                    Start on boot
                                </Typography.Title>
                                <Typography.Paragraph>
                                    If enabled, Murmure will start automatically
                                    when your system starts.
                                </Typography.Paragraph>
                            </SettingsUI.Description>
                            <Switch
                                checked={startOnBoot}
                                onCheckedChange={setStartOnBoot}
                            />
                        </SettingsUI.Item>
                        <SettingsUI.Separator />
                        <SettingsUI.Item>
                            <SettingsUI.Description>
                                <Typography.Title className="flex items-center gap-2">
                                    <Eye className="w-4 h-4 text-zinc-400" />
                                    Overlay visibility
                                </Typography.Title>
                                <Typography.Paragraph>
                                    Choose when to show the recording overlay.
                                </Typography.Paragraph>
                            </SettingsUI.Description>

                            <div className="flex gap-2">
                                <Select
                                    value={overlayMode}
                                    onValueChange={setOverlayMode}
                                >
                                    <SelectTrigger className="w-[150px]">
                                        <SelectValue placeholder="Select a mode" />
                                    </SelectTrigger>
                                    <SelectContent>
                                        <SelectItem value="hidden">
                                            Hidden
                                        </SelectItem>
                                        <SelectItem value="recording">
                                            While recording
                                        </SelectItem>
                                        <SelectItem value="always">
                                            Always
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
                                    Overlay position
                                </Typography.Title>
                                <Typography.Paragraph>
                                    Choose whether the overlay appears at the
                                    top or bottom.
                                </Typography.Paragraph>
                            </SettingsUI.Description>
                            <div className="flex gap-2">
                                <Select
                                    value={overlayPosition}
                                    onValueChange={setOverlayPosition}
                                >
                                    <SelectTrigger className="w-[150px]">
                                        <SelectValue placeholder="Select a position" />
                                    </SelectTrigger>
                                    <SelectContent>
                                        <SelectItem value="top">Top</SelectItem>
                                        <SelectItem value="bottom">
                                            Bottom
                                        </SelectItem>
                                    </SelectContent>
                                </Select>
                            </div>
                        </SettingsUI.Item>
                        <SettingsUI.Separator />
                        <SettingsUI.Item>
                            <SettingsUI.Description>
                                <Typography.Title className="flex items-center gap-2">
                                    <Zap className="w-4 h-4 text-zinc-400" />
                                    Local HTTP API (Experimental)
                                </Typography.Title>
                                <Typography.Paragraph>
                                    Enable a local HTTP API for transcribing audio files
                                    from other applications. Access it at
                                    http://localhost:{apiPort}/api/transcribe
                                </Typography.Paragraph>
                            </SettingsUI.Description>
                            <Switch
                                checked={apiEnabled}
                                onCheckedChange={setApiEnabled}
                            />
                        </SettingsUI.Item>
                        {apiEnabled && (
                            <>
                                <SettingsUI.Separator />
                                <SettingsUI.Item>
                                    <SettingsUI.Description>
                                        <Typography.Title>API Port</Typography.Title>
                                        <Typography.Paragraph>
                                            Set the port number for the HTTP API
                                            (1024-65535)
                                        </Typography.Paragraph>
                                    </SettingsUI.Description>
                                    <Input
                                        type="number"
                                        min={1024}
                                        max={65535}
                                        value={apiPort}
                                        onChange={(e) =>
                                            setApiPort(parseInt(e.target.value, 10))
                                        }
                                        className="w-32"
                                    />
                                </SettingsUI.Item>
                            </>
                        )}
                    </SettingsUI.Container>
                </div>
            </div>
        </main>
    );
};
