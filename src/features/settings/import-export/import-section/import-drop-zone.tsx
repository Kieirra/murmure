import { useState, useEffect } from 'react';
import { Upload, AlertTriangle } from 'lucide-react';
import { getCurrentWebview } from '@tauri-apps/api/webview';
import clsx from 'clsx';
import { Page } from '@/components/page';
import { useTranslation } from '@/i18n';
import { ImportState } from '../types';

interface ImportDropZoneProps {
    state: ImportState;
    errorMessage: string;
    onBrowse: () => void;
    onFileDrop: (filePath: string) => void;
    onTryAnother: () => void;
}

export const ImportDropZone = ({
    state,
    errorMessage,
    onBrowse,
    onFileDrop,
    onTryAnother,
}: ImportDropZoneProps) => {
    const { t } = useTranslation();
    const [isDragging, setIsDragging] = useState(false);

    useEffect(() => {
        const unlisten = getCurrentWebview().onDragDropEvent((event) => {
            if (event.payload.type === 'enter') {
                setIsDragging(true);
            } else if (event.payload.type === 'drop') {
                setIsDragging(false);
                const path = event.payload.paths[0];
                if (path != null) {
                    onFileDrop(path);
                }
            } else if (event.payload.type === 'leave') {
                setIsDragging(false);
            }
        });

        return () => {
            unlisten.then((fn) => fn());
        };
    }, [onFileDrop]);

    const isError = state === 'file_error';
    const isVersionError = state === 'version_error';

    if (isVersionError) {
        return (
            <div className="border border-border rounded-md p-6 flex flex-col items-center gap-3 text-center">
                <AlertTriangle className="h-8 w-8 text-yellow-400" />
                <p className="text-sm text-muted-foreground">{errorMessage}</p>
                <div className="flex gap-2">
                    <Page.SecondaryButton onClick={onTryAnother}>
                        {t('Try another file')}
                    </Page.SecondaryButton>
                    <a
                        href="https://github.com/Kieirra/murmure/releases/latest"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        <Page.PrimaryButton>
                            {t('Update Murmure')}
                        </Page.PrimaryButton>
                    </a>
                </div>
            </div>
        );
    }

    return (
        <div
            role="button"
            tabIndex={0}
            className={clsx(
                'border-2 border-dashed rounded-md p-6 flex flex-col items-center gap-3 transition-colors cursor-pointer',
                isDragging
                    ? 'border-sky-500 bg-sky-500/10'
                    : isError
                      ? 'border-red-500/50'
                      : 'border-border',
                !isDragging && 'hover:border-sky-500 hover:bg-sky-500/10'
            )}
            onClick={onBrowse}
            onKeyDown={(e) => {
                if (e.key === 'Enter' || e.key === ' ') {
                    e.preventDefault();
                    onBrowse();
                }
            }}
        >
            {isError ? (
                <>
                    <AlertTriangle className="h-8 w-8 text-red-400" />
                    <p className="text-sm text-red-400">{errorMessage}</p>
                    <Page.SecondaryButton
                        onClick={(e) => {
                            e.stopPropagation();
                            onTryAnother();
                        }}
                    >
                        {t('Try another file')}
                    </Page.SecondaryButton>
                </>
            ) : (
                <>
                    <Upload className="h-8 w-8 text-muted-foreground" />
                    <p className="text-sm text-muted-foreground">
                        {isDragging
                            ? t('Drop your file here')
                            : t('Drag & drop or select a .murmure file')}
                    </p>
                    <div className={clsx(isDragging && 'invisible')}>
                        <Page.SecondaryButton
                            onClick={(e) => {
                                e.stopPropagation();
                                onBrowse();
                            }}
                        >
                            {t('Browse')}
                        </Page.SecondaryButton>
                    </div>
                </>
            )}
        </div>
    );
};
