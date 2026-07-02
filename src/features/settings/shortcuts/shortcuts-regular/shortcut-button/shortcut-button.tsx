import { Button } from '@/components/button';
import { ResetButton } from '@/components/reset-button';
import { ClearButton } from '@/components/clear-button/clear-button';
import { RenderKeys } from '@/components/render-keys';
import { Pencil } from 'lucide-react';
import { Dialog, DialogTrigger, DialogContent, DialogTitle, DialogDescription } from '@/components/dialog';
import { Typography } from '@/components/typography';
import { useShortcutInteractions } from './hooks/use-shortcut-interactions';
import { ExistingShortcut } from './hooks/use-shortcut-interactions.helpers';
import { useTranslation } from '@/i18n';

export const ShortcutButton = ({
    keyName,
    shortcut,
    saveShortcut,
    resetShortcut,
    dataTestId,
    existingShortcuts = [],
}: {
    keyName: string;
    shortcut: string;
    saveShortcut: (shortcut: string) => void;
    resetShortcut: (existingShortcuts?: ExistingShortcut[]) => void;
    dataTestId?: string;
    existingShortcuts?: ExistingShortcut[];
}) => {
    const { binding, isRecording, conflict, resetRecording, startRecording } = useShortcutInteractions(
        shortcut,
        saveShortcut,
        resetShortcut,
        existingShortcuts
    );

    const { t } = useTranslation();
    const isUnassigned = shortcut.length === 0;

    let label: React.ReactNode;
    if (isRecording && binding.length > 0) {
        label = <RenderKeys keyString={binding} className="flex-wrap" />;
    } else if (isRecording) {
        label = <span className="text-muted-foreground">{t('Press keys...')}</span>;
    } else if (isUnassigned) {
        label = <span className="text-muted-foreground">{t('Unassigned')}</span>;
    } else {
        label = <RenderKeys keyString={shortcut} className="flex-wrap" />;
    }

    return (
        <div className="flex flex-row items-center gap-4">
            <Dialog open={isRecording} onOpenChange={startRecording}>
                <DialogTrigger asChild>
                    <Button
                        variant="outline"
                        className="px-2 whitespace-normal w-[158px] h-auto"
                        data-testid={dataTestId}
                    >
                        <Pencil />
                        {label}
                    </Button>
                </DialogTrigger>
                <DialogContent>
                    <div className="flex flex-col gap-4 text-center">
                        <DialogTitle>
                            <Typography.Title>{keyName}</Typography.Title>
                        </DialogTitle>
                        <DialogDescription className="flex flex-col gap-4">
                            <Typography.Paragraph>
                                <span className="font-bold text-foreground">{t('Enter')}</span> {t('to validate or')}{' '}
                                <span className="font-bold text-foreground">{t('Escape')}</span> {t('to cancel.')}
                            </Typography.Paragraph>
                            <div className="px-2 w-full bg-card border border-border rounded-md py-2">{label}</div>
                            {conflict != null && (
                                <span className="text-sm text-destructive">
                                    {t('Already used by "{{name}}". Choose another combination.', { name: conflict })}
                                </span>
                            )}
                        </DialogDescription>
                    </div>
                </DialogContent>
            </Dialog>
            <div className="flex flex-row items-center gap-0.5">
                <ClearButton onClick={() => saveShortcut('')} disabled={isUnassigned} />
                <ResetButton onClick={resetRecording} />
            </div>
        </div>
    );
};
