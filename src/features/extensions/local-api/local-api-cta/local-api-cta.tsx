import { Lock, Plug } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { ExtensionEnableAction } from '@/features/extensions/extension-enable-action/extension-enable-action';
import { PreviewCell } from '@/features/extensions/extension-demo/preview-cell/preview-cell';
import { PreviewArrow } from '@/features/extensions/extension-demo/preview-arrow/preview-arrow';
import { CodeBlock } from '../code-block/code-block';
import { buildCurl, DEFAULT_API_PORT, TRANSCRIBE_ENDPOINT } from '../local-api.helpers';

interface LocalApiCtaProps {
    onEnable: () => void;
}

export const LocalApiCta = ({ onEnable }: LocalApiCtaProps) => {
    const { t } = useTranslation();

    return (
        <section data-testid="local-api-cta" className="flex flex-col items-center text-center gap-12 pt-6 pb-10">
            <ExtensionEnableAction
                icon={Plug}
                label={t('Enable Local API')}
                onEnable={onEnable}
                testId="local-api-cta-enable"
                colorScheme="standard"
                hintIcon={Lock}
                hint={t('Only reachable from this computer.')}
            />

            <div className="w-full rounded-md border border-surface-border bg-surface-header p-4">
                <div className="grid grid-cols-1 gap-4 text-left">
                    <PreviewCell label={t('Request')}>
                        <CodeBlock bare>{buildCurl(TRANSCRIBE_ENDPOINT, DEFAULT_API_PORT, {})}</CodeBlock>
                    </PreviewCell>

                    <PreviewArrow horizontalFrom="never" />

                    <PreviewCell label={t('Response')}>
                        <CodeBlock bare>{TRANSCRIBE_ENDPOINT.responseExample}</CodeBlock>
                    </PreviewCell>
                </div>
            </div>
        </section>
    );
};
