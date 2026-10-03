import type { ChangeEvent } from 'react';
import clsx from 'clsx';
import { Input } from '@/components/input';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/select';
import { Textarea } from '@/components/textarea';
import { Typography } from '@/components/typography';
import { useTranslation } from '@/i18n';
import type { EndpointField } from '../../../local-api.types';
import { WavFilePicker } from '../../wav-file-picker/wav-file-picker';

const PROVIDER_OPTIONS = ['local', 'remote'];

interface EndpointFieldControlProps {
    field: EndpointField;
    value?: string;
    file: File | null;
    promptNames: string[];
    testIdPrefix: string;
    onValueChange: (value: string) => void;
    onFileChange: (file: File | null) => void;
}

export const EndpointFieldControl = ({
    field,
    value,
    file,
    promptNames,
    testIdPrefix,
    onValueChange,
    onFileChange,
}: EndpointFieldControlProps) => {
    const { t } = useTranslation();
    const testId = `${testIdPrefix}-${field.name}`;
    const textProps = {
        value: value ?? '',
        onChange: (event: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => onValueChange(event.target.value),
        'data-testid': testId,
    };

    if (field.kind === 'prompt-name' && promptNames.length === 0) {
        return <Typography.Paragraph>{t('No prompt yet. Create one in Prompt Mode.')}</Typography.Paragraph>;
    }

    switch (field.kind) {
        case 'file':
            return <WavFilePicker file={file} onChange={onFileChange} testId={testId} />;
        case 'model':
            return <Input {...textProps} />;
        case 'instruction':
            return <Textarea {...textProps} />;
        case 'provider':
        case 'prompt-name':
            return (
                <Select value={value} onValueChange={onValueChange}>
                    <SelectTrigger
                        className={clsx(
                            field.kind === 'provider' && 'w-[180px]',
                            field.kind === 'prompt-name' && 'w-[240px]'
                        )}
                        data-testid={testId}
                    >
                        <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                        {(field.kind === 'provider' ? PROVIDER_OPTIONS : promptNames).map((option, index) => (
                            <SelectItem key={`${option}-${index}`} value={option}>
                                {option}
                            </SelectItem>
                        ))}
                    </SelectContent>
                </Select>
            );
    }
};
