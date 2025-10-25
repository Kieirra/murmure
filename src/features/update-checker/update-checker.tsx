import { useEffect, useRef, useState } from 'react';
import { check } from '@tauri-apps/plugin-updater';
import { invoke } from '@tauri-apps/api/core';
import { RefreshCcw } from 'lucide-react';

type UpdateCheckerProps = {
    className?: string;
};

export const UpdateChecker = ({ className = '' }: UpdateCheckerProps) => {
    const [isChecking, setIsChecking] = useState(false);
    const [updateAvailable, setUpdateAvailable] = useState(false);
    const [isInstalling, setIsInstalling] = useState(false);
    const [downloadProgress, setDownloadProgress] = useState(0);
    const [showUpToDate, setShowUpToDate] = useState(false);
    const [linuxTarget, setLinuxTarget] = useState<string | null>(null);

    const upToDateTimeoutRef = useRef<
        ReturnType<typeof setTimeout> | undefined
    >(undefined);
    const isManualCheckRef = useRef(false);
    const downloadedBytesRef = useRef(0);
    const contentLengthRef = useRef(0);

    useEffect(() => {
        // Get the Linux format on component mount (only on Linux)
        const initLinuxFormat = async () => {
            try {
                const target = await invoke<string>('get_linux_format');
                setLinuxTarget(target);
                // Check for updates with the detected target
                await performUpdateCheck(target);
            } catch {
                // get_linux_format not available (not on Linux)
                setLinuxTarget(null);
                // Still check for updates even without Linux target (Windows, macOS, etc.)
                await performUpdateCheck(null);
            }
        };

        initLinuxFormat();
        return () => {
            if (upToDateTimeoutRef.current)
                clearTimeout(upToDateTimeoutRef.current);
        };
    }, []);

    const performUpdateCheck = async (targetParam: string | null) => {
        if (isChecking) return;
        try {
            setIsChecking(true);
            const update = await check(
                targetParam ? { target: targetParam } : undefined
            );

            if (update) {
                setUpdateAvailable(true);
                setShowUpToDate(false);
            } else {
                setUpdateAvailable(false);
                if (isManualCheckRef.current) {
                    setShowUpToDate(true);
                    if (upToDateTimeoutRef.current)
                        clearTimeout(upToDateTimeoutRef.current);
                    upToDateTimeoutRef.current = setTimeout(
                        () => setShowUpToDate(false),
                        3000
                    );
                }
            }
        } catch (e) {
            console.error('Failed to check for updates:', e);
        } finally {
            setIsChecking(false);
            isManualCheckRef.current = false;
        }
    };

    const checkForUpdates = async () => {
        await performUpdateCheck(linuxTarget);
    };

    const handleManualUpdateCheck = () => {
        isManualCheckRef.current = true;
        checkForUpdates();
    };

    const installUpdate = async () => {
        try {
            setIsInstalling(true);
            setDownloadProgress(0);
            downloadedBytesRef.current = 0;
            contentLengthRef.current = 0;
            const update = await check(
                linuxTarget ? { target: linuxTarget } : undefined
            );
            if (!update) return;
            await update.downloadAndInstall((event) => {
                switch (event.event) {
                    case 'Started':
                        downloadedBytesRef.current = 0;
                        contentLengthRef.current =
                            event.data.contentLength ?? 0;
                        break;
                    case 'Progress':
                        downloadedBytesRef.current += event.data.chunkLength;
                        const total = contentLengthRef.current;
                        const progress =
                            total > 0
                                ? Math.round(
                                      (downloadedBytesRef.current / total) * 100
                                  )
                                : 0;
                        setDownloadProgress(Math.min(progress, 100));
                        break;
                    default:
                        break;
                }
            });
            // On Windows, the app exits automatically during install.
            // On other platforms, the user can restart manually later.
        } catch (e) {
            console.error('Failed to install update:', e);
        } finally {
            setIsInstalling(false);
            setDownloadProgress(0);
            downloadedBytesRef.current = 0;
            contentLengthRef.current = 0;
        }
    };

    const getUpdateStatusText = () => {
        if (isInstalling) {
            if (downloadProgress > 0 && downloadProgress < 100)
                return `Downloading... ${String(downloadProgress).padStart(3)}%`;
            if (downloadProgress === 100) return 'Installing...';
            return 'Preparing...';
        }
        if (isChecking) return 'Checking...';
        if (showUpToDate) return 'Up to date';
        if (updateAvailable) return 'Update available';
        return 'Check for updates';
    };

    const onClick = () => {
        if (updateAvailable && !isInstalling) return installUpdate();
        if (!isChecking && !isInstalling && !updateAvailable)
            return handleManualUpdateCheck();
    };

    const isDisabled = isChecking || isInstalling;
    const isClickable =
        !isDisabled && (updateAvailable || (!isChecking && !showUpToDate));

    return (
        <button
            onClick={onClick}
            disabled={isDisabled || !isClickable}
            className={`text-xs text-zinc-500 hover:text-zinc-300 transition-colors flex items-center gap-1.5 px-2 py-1 rounded hover:bg-zinc-800 cursor-pointer disabled:opacity-50 ${className}`}
        >
            <RefreshCcw className="w-4 h-4" />
            <span>{getUpdateStatusText()}</span>
            {isInstalling && downloadProgress > 0 && downloadProgress < 100 && (
                <span className="text-[10px] text-zinc-400 tabular-nums">
                    {downloadProgress}%
                </span>
            )}
        </button>
    );
};

export default UpdateChecker;
