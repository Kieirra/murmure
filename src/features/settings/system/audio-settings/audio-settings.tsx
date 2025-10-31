import { useAudioDevicesState } from './hooks/use-audio-devices-state';
import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { Mic } from 'lucide-react';
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from '@/components/select';

export const AudioSettings = () => {
    const { devices, selectedDevice, isLoading, error, changeDevice } =
        useAudioDevicesState();

    if (isLoading) {
        return (
            <SettingsUI.Item>
                <SettingsUI.Description>
                    <Typography.Title className="flex items-center gap-2">
                        <Mic className="w-4 h-4 text-zinc-400" />
                        Input Microphone
                    </Typography.Title>
                    <Typography.Paragraph>
                        Loading available microphones...
                    </Typography.Paragraph>
                </SettingsUI.Description>
            </SettingsUI.Item>
        );
    }

    if (error) {
        return (
            <SettingsUI.Item>
                <SettingsUI.Description>
                    <Typography.Title className="flex items-center gap-2">
                        <Mic className="w-4 h-4 text-zinc-400" />
                        Input Microphone
                    </Typography.Title>
                    <Typography.Paragraph className="text-red-500">
                        Error: {error}
                    </Typography.Paragraph>
                </SettingsUI.Description>
            </SettingsUI.Item>
        );
    }

    if (devices.length === 0) {
        return (
            <SettingsUI.Item>
                <SettingsUI.Description>
                    <Typography.Title className="flex items-center gap-2">
                        <Mic className="w-4 h-4 text-zinc-400" />
                        Input Microphone
                    </Typography.Title>
                    <Typography.Paragraph>
                        No input devices found
                    </Typography.Paragraph>
                </SettingsUI.Description>
            </SettingsUI.Item>
        );
    }

    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title className="flex items-center gap-2">
                    <Mic className="w-4 h-4 text-zinc-400" />
                    Input Microphone
                </Typography.Title>
                <Typography.Paragraph>
                    Select which microphone to use for transcription
                </Typography.Paragraph>
            </SettingsUI.Description>
            <Select value={selectedDevice || ''} onValueChange={changeDevice}>
                <SelectTrigger className="w-48">
                    <SelectValue placeholder="Select a microphone" />
                </SelectTrigger>
                <SelectContent>
                    {devices.map((device) => (
                        <SelectItem key={device.name} value={device.name}>
                            {device.name}
                            {device.is_default && ' (Default)'}
                        </SelectItem>
                    ))}
                </SelectContent>
            </Select>
        </SettingsUI.Item>
    );
};
