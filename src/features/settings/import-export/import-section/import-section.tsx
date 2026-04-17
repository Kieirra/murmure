import { Typography } from '@/components/typography';
import { useTranslation } from '@/i18n';
import { ImportDropZone } from './import-drop-zone/import-drop-zone';
import { ImportPreview } from './import-preview/import-preview';
import { ImportProgressModal } from './import-progress-modal/import-progress-modal';
import { useImport } from './hooks/use-import';

export const ImportSection = () => {
    const { t } = useTranslation();
    const {
        state,
        configData,
        fileName,
        errorMessage,
        isImporting,
        importSteps,
        isImportComplete,
        hasImportError,
        loadFile,
        browseFile,
        applyImport,
        reset,
    } = useImport();

    const showPreview = state === 'previewing' || state === 'importing' || state === 'done';
    const showModal = state === 'importing' || state === 'done' || state === 'partial_error';

    return (
        <div className="space-y-4">
            <Typography.Title className="font-semibold text-sky-400!">{t('Import')}</Typography.Title>

            {showPreview && configData != null ? (
                <ImportPreview
                    key={configData.exported_at + fileName}
                    configData={configData}
                    fileName={fileName}
                    isImporting={isImporting}
                    onImport={applyImport}
                    onCancel={reset}
                />
            ) : (
                <ImportDropZone
                    state={state}
                    errorMessage={errorMessage}
                    onBrowse={browseFile}
                    onFileDrop={loadFile}
                    onTryAnother={reset}
                />
            )}

            <ImportProgressModal
                open={showModal}
                steps={importSteps}
                isComplete={isImportComplete}
                hasError={hasImportError}
                onDone={reset}
            />
        </div>
    );
};
