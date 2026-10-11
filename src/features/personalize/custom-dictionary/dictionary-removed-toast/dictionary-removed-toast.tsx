import { useTranslation } from '@/i18n';

interface DictionaryRemovedToastProps {
    word: string;
    onUndo: () => void;
}

export const DictionaryRemovedToast = ({ word, onUndo }: DictionaryRemovedToastProps) => {
    const { t } = useTranslation();

    return (
        <div className="flex w-full items-center justify-between gap-3" data-testid="custom-dictionary-removed-toast">
            <span className="text-sm">{t('"{{word}}" removed from the dictionary', { word })}</span>
            <button
                type="button"
                onClick={onUndo}
                className="shrink-0 cursor-pointer rounded-md px-2 py-1 text-sm font-medium text-sky-400 hover:text-sky-300 transition-colors"
                data-testid="custom-dictionary-undo-button"
            >
                {t('Undo')}
            </button>
        </div>
    );
};
