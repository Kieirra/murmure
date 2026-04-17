import clsx from 'clsx';
import { CheckCircle2, Loader2 } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { Button } from '@/components/button';
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription } from '@/components/dialog';

export interface ImportProgressStep {
    label: string;
    status: 'pending' | 'in_progress' | 'done' | 'error';
}

const STATUS_TEXT_CLASSES: Record<ImportProgressStep['status'], string> = {
    pending: 'text-muted-foreground/50',
    in_progress: 'text-muted-foreground',
    done: 'text-muted-foreground',
    error: 'text-red-400',
};

interface ImportProgressModalProps {
    open: boolean;
    steps: ImportProgressStep[];
    isComplete: boolean;
    hasError: boolean;
    onDone: () => void;
}

export const ImportProgressModal = ({ open, steps, isComplete, hasError, onDone }: ImportProgressModalProps) => {
    const { t } = useTranslation();

    return (
        <Dialog open={open}>
            <DialogContent className="sm:max-w-sm" onInteractOutside={(e) => e.preventDefault()}>
                <DialogHeader>
                    <DialogTitle>{t('Importing configuration')}</DialogTitle>
                    <DialogDescription className="sr-only">{t('Import progress')}</DialogDescription>
                </DialogHeader>

                <div className="space-y-3 py-2">
                    {steps.map((step) => (
                        <div key={step.label} className="flex items-center gap-3">
                            {step.status === 'in_progress' && (
                                <Loader2 className="w-4 h-4 text-sky-400 animate-spin shrink-0" />
                            )}
                            {step.status === 'done' && (
                                <CheckCircle2 className="w-4 h-4 text-emerald-400 shrink-0" />
                            )}
                            {step.status === 'pending' && (
                                <div className="w-4 h-4 rounded-full border border-border shrink-0" />
                            )}
                            {step.status === 'error' && (
                                <div className="w-4 h-4 rounded-full bg-red-400 shrink-0" />
                            )}
                            <span className={clsx('text-sm', STATUS_TEXT_CLASSES[step.status])}>
                                {step.label}
                            </span>
                        </div>
                    ))}
                </div>

                <div className="flex justify-center items-center h-16">
                    <CheckCircle2
                        className={clsx(
                            'w-16 h-16 text-emerald-400 transition-opacity',
                            isComplete && !hasError ? 'opacity-100 animate-in zoom-in' : 'opacity-0'
                        )}
                    />
                </div>

                <div className="flex justify-end">
                    <Button
                        onClick={onDone}
                        className={clsx(
                            'bg-sky-600 hover:bg-sky-700 text-white transition-opacity',
                            !(isComplete || hasError) && 'invisible pointer-events-none'
                        )}
                    >
                        {t('Done')}
                    </Button>
                </div>
            </DialogContent>
        </Dialog>
    );
};
