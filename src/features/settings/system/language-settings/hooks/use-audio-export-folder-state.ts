import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';

export const useAudioExportFolderState = () => {
    const [exportFolder, setExportFolder] = useState<string>('');

    useEffect(() => {
        invoke<string>('get_audio_export_folder').then((folder) => {
            setExportFolder(folder);
        });
    }, []);

    const handleSetExportFolder = async (folder: string) => {
        try {
            setExportFolder(folder);
            await invoke('set_audio_export_folder', { path: folder });
        } catch {
            // Revert on error
            invoke<string>('get_audio_export_folder').then(setExportFolder);
        }
    };

    return {
        exportFolder,
        setExportFolder: handleSetExportFolder,
    };
};
