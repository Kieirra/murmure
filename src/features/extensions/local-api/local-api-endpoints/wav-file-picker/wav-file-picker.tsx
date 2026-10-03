import { useRef } from 'react';
import { FileAudio } from 'lucide-react';
import { Button } from '@/components/button';
import { useTranslation } from '@/i18n';

interface WavFilePickerProps {
    file: File | null;
    onChange: (file: File | null) => void;
    testId: string;
}

export const WavFilePicker = ({ file, onChange, testId }: WavFilePickerProps) => {
    const { t } = useTranslation();
    const inputRef = useRef<HTMLInputElement>(null);

    return (
        <div className="flex items-center gap-3 min-w-0">
            <input
                ref={inputRef}
                type="file"
                accept=".wav,audio/wav,audio/x-wav"
                className="hidden"
                onChange={(event) => onChange(event.target.files?.[0] ?? null)}
                data-testid={`${testId}-input`}
            />
            <Button variant="outline" size="sm" onClick={() => inputRef.current?.click()} data-testid={testId}>
                <FileAudio className="w-3.5 h-3.5" />
                {t('Choose a WAV file')}
            </Button>
            {file !== null && <span className="text-sm text-foreground truncate">{file.name}</span>}
        </div>
    );
};
