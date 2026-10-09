import { LucideIcon, Server } from 'lucide-react';
import { Page } from '@/components/page';
import { useTranslation } from '@/i18n';

interface LLMEnableFooterProps {
    icon: LucideIcon;
    label: string;
    needsSetup: boolean;
    onEnable: () => void;
    testId: string;
}

export const LLMEnableFooter = ({ icon: Icon, label, needsSetup, onEnable, testId }: LLMEnableFooterProps) => {
    const { t } = useTranslation();

    return (
        <div className="flex flex-col items-center gap-3 py-3 md:py-4">
            <Page.PrimaryButton onClick={onEnable} data-testid={testId}>
                <Icon className="w-4 h-4" />
                {label}
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
    );
};
