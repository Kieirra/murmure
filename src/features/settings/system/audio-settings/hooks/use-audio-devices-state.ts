import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface AudioDevice {
    name: string;
    is_default: boolean;
}

export const useAudioDevicesState = () => {
    const [devices, setDevices] = useState<AudioDevice[]>([]);
    const [selectedDevice, setSelectedDevice] = useState<string | null>(null);
    const [isLoading, setIsLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    // Load available microphones and selected device on mount
    useEffect(() => {
        loadAudioDevices();
    }, []);

    const loadAudioDevices = async () => {
        try {
            setIsLoading(true);
            setError(null);

            // Get available devices
            const availableDevices = await invoke<AudioDevice[]>(
                'get_available_microphones'
            );
            setDevices(availableDevices);

            // Get currently selected device
            const selected = await invoke<string | null>(
                'get_selected_microphone'
            );
            setSelectedDevice(selected);
        } catch (err) {
            const errorMessage =
                err instanceof Error ? err.message : 'Failed to load audio devices';
            setError(errorMessage);
            console.error('Error loading audio devices:', err);
        } finally {
            setIsLoading(false);
        }
    };

    const changeDevice = async (deviceName: string) => {
        try {
            setError(null);
            await invoke('set_selected_microphone', { device_name: deviceName });
            setSelectedDevice(deviceName);
        } catch (err) {
            const errorMessage =
                err instanceof Error
                    ? err.message
                    : 'Failed to change microphone';
            setError(errorMessage);
            console.error('Error changing microphone:', err);
        }
    };

    return {
        devices,
        selectedDevice,
        isLoading,
        error,
        changeDevice,
        reload: loadAudioDevices,
    };
};
