import { useTranslation } from 'react-i18next';
import { Typography } from '@/components/typography';
import { motion } from 'framer-motion';
import {
    CheckCircle2,
    Download,
    ExternalLink,
    RefreshCw,
    AlertCircle,
} from 'lucide-react';
import { useState } from 'react';
import { DEFAULT_OLLAMA_URL } from '../../llm-connect.constants';
import { Page } from '@/components/page';
import clsx from 'clsx';
import { StepItem } from '@/components/step-item';

interface StepInstallProps {
    onNext: () => void;
    testConnection: (url?: string) => Promise<boolean>;
}

export const StepInstall = ({ onNext, testConnection }: StepInstallProps) => {
    const { t } = useTranslation();
    const [isTesting, setIsTesting] = useState(false);
    const [isConnected, setIsConnected] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const handleTest = async () => {
        setIsTesting(true);
        setError(null);
        try {
            const success = await testConnection(DEFAULT_OLLAMA_URL);
            if (success) {
                setIsConnected(true);
            } else {
                setError(
                    t('Could not connect to Ollama. Make sure it is running.')
                );
            }
        } catch {
            setError(t('Connection failed.'));
        } finally {
            setIsTesting(false);
        }
    };

    const renderTestButtonContent = () => {
        if (isTesting) {
            return (
                <>
                    <RefreshCw className="w-4 h-4 animate-spin mr-2" />
                    {isConnected ? t('Connected') : t('Test Connection')}
                </>
            );
        } else if (isConnected) {
            return (
                <>
                    <CheckCircle2 className="w-4 h-4 mr-2" />
                    {t('Connected')}
                </>
            );
        } else {
            return t('Test Connection');
        }
    };

    return (
        <motion.div
            initial={{ opacity: 0, x: 20 }}
            animate={{ opacity: 1, x: 0 }}
            exit={{ opacity: 0, x: -20 }}
            className="flex flex-col items-center max-w-2xl mx-auto space-y-8 py-8"
        >
            <div className="text-center space-y-4">
                <Typography.MainTitle>
                    {t('Install Ollama')}
                </Typography.MainTitle>
                <Typography.Paragraph className="text-zinc-400 max-w-lg mx-auto">
                    {t(
                        'Ollama is the engine that runs local LLMs. You need to download and install it to use this feature.'
                    )}
                </Typography.Paragraph>
            </div>

            <div className="w-full bg-zinc-800/30 border border-zinc-800 rounded-xl p-8 space-y-8">
                <StepItem
                    step={1}
                    title={t('Download & Install')}
                    description={t(
                        'Download Ollama from the official website and install it.'
                    )}
                    isActive={true}
                >
                    <a
                        href="https://ollama.com/download"
                        target="_blank"
                        rel="noopener noreferrer"
                        className="inline-flex items-center gap-2 text-sky-400 hover:text-sky-300 transition-colors text-sm font-medium"
                    >
                        <Download className="w-4 h-4" />
                        {t('Download Ollama')}
                        <ExternalLink className="w-3 h-3" />
                    </a>
                </StepItem>

                <div className="w-full h-px bg-zinc-800" />

                <StepItem
                    step={2}
                    title={t('Verify Connection')}
                    description={t(
                        'Once installed and running, test the connection.'
                    )}
                    isActive={isConnected}
                >
                    <div className="flex items-center gap-4">
                        <Page.SecondaryButton
                            onClick={handleTest}
                            data-testid="llm-connect-test-button"
                            variant="outline"
                            className={clsx(
                                isConnected &&
                                    'text-emerald-500 hover:bg-emerald-400/10 hover:text-emerald-300'
                            )}
                        >
                            {renderTestButtonContent()}
                        </Page.SecondaryButton>

                        {error && (
                            <div className="flex items-center gap-2 text-red-400 text-sm animate-in fade-in slide-in-from-left-2">
                                <AlertCircle className="w-4 h-4" />
                                {error}
                            </div>
                        )}
                    </div>
                </StepItem>
            </div>

            <div className="flex justify-between w-full pt-4">
                <div />
                <Page.PrimaryButton
                    onClick={onNext}
                    disabled={!isConnected}
                    size="lg"
                    className="px-8"
                    data-testid="llm-connect-next-button"
                >
                    {t('Next Step')}
                </Page.PrimaryButton>
            </div>
        </motion.div>
    );
};
