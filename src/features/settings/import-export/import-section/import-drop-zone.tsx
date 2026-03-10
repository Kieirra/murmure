import { Upload, AlertTriangle } from 'lucide-react';
import clsx from 'clsx';
import { Page } from '@/components/page';
import { useTranslation } from '@/i18n';
import { ImportState } from '../types';

interface ImportDropZoneProps {
    state: ImportState;
    errorMessage: string;
    onBrowse: () => void;
    onTryAnother: () => void;
}

export const ImportDropZone = ({
    state,
    errorMessage,
    onBrowse,
    onTryAnother,
}: ImportDropZoneProps) => {
    const { t } = useTranslation();

    const isError = state === 'file_error';
    const isVersionError = state === 'version_error';

    if (isVersionError) {
        return (
            <div className="border border-border rounded-md p-8 flex flex-col items-center gap-4 text-center">
                <AlertTriangle className="h-10 w-10 text-yellow-400" />
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
                'border-2 border-dashed rounded-md p-8 flex flex-col items-center gap-4 transition-colors cursor-pointer',
                isError ? 'border-red-500/50' : 'border-border',
                'hover:border-sky-500 hover:bg-sky-500/10'
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
                    <AlertTriangle className="h-10 w-10 text-red-400" />
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
                    <Upload className="h-10 w-10 text-muted-foreground" />
                    <p className="text-sm text-muted-foreground">
                        {t('Select a .murmure file')}
                    </p>
                    <Page.SecondaryButton
                        onClick={(e) => {
                            e.stopPropagation();
                            onBrowse();
                        }}
                    >
                        {t('Browse')}
                    </Page.SecondaryButton>
                </>
            )}
        </div>
    );
};
