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
        <div className="flex flex-row items-center gap-1">
            <Dialog open={isRecording} onOpenChange={startRecording}>
                <DialogTrigger asChild>
                    <Button
                        variant="ghost"
                        className="justify-start px-2 whitespace-normal w-[176px] h-auto border border-input bg-field shadow-xs hover:border-sky-500 dark:hover:bg-field focus-visible:border-sky-500 focus-visible:ring-sky-500/25 focus-visible:ring-[3px]"
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
                            <div className="px-2 w-full bg-field border border-input rounded-md py-2">{label}</div>
                            {conflict != null && (
                                <span className="text-sm text-destructive">
                                    {t('Already used by "{{name}}". Choose another combination.', { name: conflict })}
                                </span>
                            )}
                        </DialogDescription>
                    </div>
                </DialogContent>
            </Dialog>
            <div className="-mr-1.5 flex flex-row items-center">
                <ClearButton label={t('Remove shortcut')} onClick={() => saveShortcut('')} disabled={isUnassigned} />
                <ResetButton label={t('Restore default shortcut')} onClick={resetRecording} />
            </div>
        </div>
    );
};
