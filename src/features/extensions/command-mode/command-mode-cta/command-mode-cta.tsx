import { Server, Zap } from 'lucide-react';
import { Page } from '@/components/page';
import { useTranslation } from '@/i18n';
import { CommandPreview } from '../command-preview/command-preview';

interface CommandModeCtaProps {
    onEnable: () => void;
    needsSetup: boolean;
}

export const CommandModeCta = ({ onEnable, needsSetup }: CommandModeCtaProps) => {
    const { t } = useTranslation();

    return (
        <section data-testid="command-mode-cta" className="flex flex-col items-center text-center gap-8 pb-10">
            <div className="flex w-full flex-col gap-8">
                <CommandPreview variant="selection" />
                <CommandPreview variant="question" />
            </div>

            <div className="flex flex-col items-center gap-3 py-3 md:py-4">
                <Page.PrimaryButton onClick={onEnable} data-testid="command-mode-cta-enable">
                    <Zap className="w-4 h-4" />
                    {needsSetup ? t('Set up Command Mode') : t('Enable Command Mode')}
                </Page.PrimaryButton>
                <div className="flex items-center gap-2 text-xs text-muted-foreground">
                    <Server className="w-3.5 h-3.5 text-sky-400 shrink-0" />
                    <span>
                        {needsSetup
                            ? t('Requires Ollama (free) or your own server. Guided setup in the next step.')
                            : t('Uses the server you already set up.')}
                    </span>
                </div>
            </div>
        </section>
    );
};
