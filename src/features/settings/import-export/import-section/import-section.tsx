import { Typography } from '@/components/typography';
import { useTranslation } from '@/i18n';
import { ImportDropZone } from './import-drop-zone';
import { ImportPreview } from './import-preview';
import { useImport } from './hooks/use-import';

export const ImportSection = () => {
    const { t } = useTranslation();
    const {
        state,
        configData,
        fileName,
        errorMessage,
        isImporting,
        browseFile,
        applyImport,
        reset,
    } = useImport();

    const showPreview =
        state === 'previewing' || state === 'importing' || state === 'done';

    return (
        <div className="space-y-4">
            <Typography.Title>{t('Import')}</Typography.Title>

            {showPreview && configData != null ? (
                <ImportPreview
                    key={configData.exported_at + fileName}
                    configData={configData}
                    fileName={fileName}
                    isImporting={isImporting}
                    onImport={applyImport}
                    onCancel={reset}
                    onChangeFile={browseFile}
                />
            ) : (
                <ImportDropZone
                    state={state}
                    errorMessage={errorMessage}
                    onBrowse={browseFile}
                    onTryAnother={reset}
                />
            )}
        </div>
    );
};
