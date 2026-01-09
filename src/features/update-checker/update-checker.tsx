import { useEffect, useRef, useState } from 'react';
import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import { RefreshCcw } from 'lucide-react';
import { useTranslation } from '@/i18n';

type UpdateCheckerProps = {
    className?: string;
};

export const UpdateChecker = ({ className = '' }: UpdateCheckerProps) => {
    const [isChecking, setIsChecking] = useState(false);
    const [updateAvailable, setUpdateAvailable] = useState(false);
    const [isInstalling, setIsInstalling] = useState(false);
    const [downloadProgress, setDownloadProgress] = useState(0);
    const [showUpToDate, setShowUpToDate] = useState(false);
    const { t } = useTranslation();

    const upToDateTimeoutRef = useRef<
        ReturnType<typeof setTimeout> | undefined
    >(undefined);
    const isManualCheckRef = useRef(false);
    const downloadedBytesRef = useRef(0);
    const contentLengthRef = useRef(0);

    useEffect(() => {
        checkForUpdates();
        return () => {
            if (upToDateTimeoutRef.current)
                clearTimeout(upToDateTimeoutRef.current);
        };
    }, []);

    const checkForUpdates = async () => {
        if (isChecking) return;
        try {
            setIsChecking(true);
            const update = await check();
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
            const update = await check();
            if (!update) return;
            await update.downloadAndInstall((event) => {
                switch (event.event) {
                    case 'Started':
                        downloadedBytesRef.current = 0;
                        contentLengthRef.current =
                            event.data.contentLength ?? 0;
                        break;
                    case 'Progress': {
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
                    }
                    default:
                        break;
                }
            });
            try {
                await relaunch();
            } catch (e) {
                console.error('Failed to relaunch after install:', e);
            }
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
                return t('Downloading... {{progress}}%', {
                    progress: String(downloadProgress).padStart(3),
                });
            if (downloadProgress === 100) return t('Installing...');
            return t('Preparing...');
        }
        if (isChecking) return t('Checking...');
        if (showUpToDate) return t('Up to date');
        if (updateAvailable) return t('Update available');
        return t('Check for updates');
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
