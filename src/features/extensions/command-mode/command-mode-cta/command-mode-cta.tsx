import { Lock, Zap } from 'lucide-react';
import { Page } from '@/components/page';
import { useTranslation } from '@/i18n';
import { CommandPreview } from '../command-preview/command-preview';

interface CommandModeCtaProps {
    onEnable: () => void;
}

export const CommandModeCta = ({ onEnable }: CommandModeCtaProps) => {
    const { t } = useTranslation();

    return (
        <section data-testid="command-mode-cta" className="flex flex-col items-center text-center gap-10 pb-10">
            <div className="flex w-full flex-col gap-6">
                <CommandPreview variant="selection" />
                <CommandPreview variant="question" />
            </div>

            <div className="flex flex-col items-center gap-3">
                <Page.PrimaryButton onClick={onEnable} data-testid="command-mode-cta-enable">
                    <Zap className="w-4 h-4" />
                    {t('Enable Command Mode')}
                </Page.PrimaryButton>
                <div className="flex items-center gap-2 text-xs text-muted-foreground">
                    <Lock className="w-3.5 h-3.5 text-sky-400 shrink-0" />
                    <span>{t('Runs on your computer with Ollama, or on your own server.')}</span>
                </div>
            </div>
        </section>
    );
};
