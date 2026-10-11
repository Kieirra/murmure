import { useState } from 'react';
import { AlertTriangle } from 'lucide-react';
import { Input } from '@/components/input';
import { Page } from '@/components/page';
import { ExternalLink } from '@/components/external-link';
import { InternalLink } from '@/components/internal-link';
import { useTranslation } from '@/i18n';

interface DictionaryToolbarProps {
    onAdd: (word: string) => void;
}

export const DictionaryToolbar = ({ onAdd }: DictionaryToolbarProps) => {
    const { t } = useTranslation();
    const [newWord, setNewWord] = useState('');
    const containsDigit = /\d/.test(newWord);
    const hasMultipleSpaces =
        newWord
            .trim()
            .split('')
            .filter((c) => c === ' ').length > 1;

    const handleAddWord = () => {
        onAdd(newWord);
        setNewWord('');
    };

    const handleKeyDown = (e: React.KeyboardEvent) => {
        if (e.key === 'Enter') {
            e.preventDefault();
            handleAddWord();
        }
    };

    return (
        <div className="space-y-2">
            <div className="flex items-center gap-2">
                <Input
                    type="text"
                    value={newWord}
                    onChange={(e) => setNewWord(e.target.value)}
                    onKeyDown={handleKeyDown}
                    placeholder={t('Add names, technical terms and jargon that Murmure recognizes poorly')}
                    data-testid="custom-dictionary-input"
                />
                <Page.PrimaryButton
                    size="default"
                    className="!px-4"
                    onClick={handleAddWord}
                    disabled={!newWord.trim()}
                    data-testid="custom-dictionary-add-button"
                >
                    {t('Add')}
                </Page.PrimaryButton>
            </div>
            {containsDigit && (
                <div
                    className="flex items-start gap-1.5 text-xs text-yellow-300/90"
                    data-testid="custom-dictionary-number-warning"
                >
                    <AlertTriangle className="w-3 h-3 shrink-0 mt-0.5" />
                    <span>
                        {t('Numbers are not supported in the dictionary. Use')}{' '}
                        <InternalLink to="/personalize/formatting-rules" hash="custom-rules">
                            {t('Formatting Rules')}
                        </InternalLink>{' '}
                        {t('to handle words with digits.')}{' '}
                        <ExternalLink href="https://docs.murmure.app/features/formatting-rules/">
                            {t('Learn more')}
                        </ExternalLink>
                    </span>
                </div>
            )}
            {hasMultipleSpaces && (
                <div
                    className="flex items-start gap-1.5 text-xs text-yellow-300/90"
                    data-testid="custom-dictionary-space-warning"
                >
                    <AlertTriangle className="w-3 h-3 shrink-0 mt-0.5" />
                    <span>{t('Only single words or two-word pairs are allowed.')}</span>
                </div>
            )}
        </div>
    );
};
