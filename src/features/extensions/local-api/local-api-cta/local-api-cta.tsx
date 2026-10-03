import { Lock, Plug } from 'lucide-react';
import { Page } from '@/components/page';
import { useTranslation } from '@/i18n';
import { CodeBlock } from '../code-block/code-block';
import { buildCurl, DEFAULT_API_PORT, TRANSCRIBE_ENDPOINT } from '../local-api.helpers';

interface LocalApiCtaProps {
    onEnable: () => void;
}

export const LocalApiCta = ({ onEnable }: LocalApiCtaProps) => {
    const { t } = useTranslation();

    return (
        <section data-testid="local-api-cta" className="flex flex-col items-center text-center gap-8 pb-10">
            <div className="flex w-full flex-col gap-3 text-left">
                <CodeBlock>{buildCurl(TRANSCRIBE_ENDPOINT, DEFAULT_API_PORT, {})}</CodeBlock>
                <CodeBlock>{TRANSCRIBE_ENDPOINT.responseExample}</CodeBlock>
                <p className="text-sm text-muted-foreground text-center">
                    {t('Send an audio file, get the text back.')}
                </p>
            </div>

            <div className="flex flex-col items-center gap-3 py-3 md:py-4">
                <Page.PrimaryButton onClick={onEnable} data-testid="local-api-cta-enable">
                    <Plug className="w-4 h-4" />
                    {t('Enable Local API')}
                </Page.PrimaryButton>
                <div className="flex items-center gap-2 text-xs text-muted-foreground">
                    <Lock className="w-3.5 h-3.5 text-sky-400 shrink-0" />
                    <span>{t('Only reachable from this computer.')}</span>
                </div>
            </div>
        </section>
    );
};
