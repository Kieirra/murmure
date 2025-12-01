import { useTranslation } from 'react-i18next';
import { Button } from '@/components/button';
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
        } catch (err) {
            setError(t('Connection failed.'));
        } finally {
            setIsTesting(false);
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

            <div className="w-full bg-zinc-900/50 border border-zinc-800 rounded-xl p-8 space-y-8">
                {/* Step 1: Download */}
                <div className="flex gap-4">
                    <div className="flex-shrink-0 w-8 h-8 rounded-full bg-blue-500/20 text-blue-400 flex items-center justify-center font-bold">
                        1
                    </div>
                    <div className="space-y-3 flex-1">
                        <h3 className="font-semibold text-lg">
                            {t('Download & Install')}
                        </h3>
                        <p className="text-sm text-zinc-400">
                            {t(
                                'Download Ollama from the official website and install it.'
                            )}
                        </p>
                        <a
                            href="https://ollama.com/download"
                            target="_blank"
                            rel="noopener noreferrer"
                            className="inline-flex items-center gap-2 text-blue-400 hover:text-blue-300 transition-colors text-sm font-medium"
                        >
                            <Download className="w-4 h-4" />
                            {t('Download Ollama')}
                            <ExternalLink className="w-3 h-3" />
                        </a>
                    </div>
                </div>

                <div className="w-full h-px bg-zinc-800" />

                {/* Step 2: Verify */}
                <div className="flex gap-4">
                    <div
                        className={`flex-shrink-0 w-8 h-8 rounded-full flex items-center justify-center font-bold transition-colors ${isConnected ? 'bg-green-500/20 text-green-400' : 'bg-zinc-800 text-zinc-400'}`}
                    >
                        2
                    </div>
                    <div className="space-y-4 flex-1">
                        <h3 className="font-semibold text-lg">
                            {t('Verify Connection')}
                        </h3>
                        <p className="text-sm text-zinc-400">
                            {t(
                                'Once installed and running, test the connection.'
                            )}
                        </p>

                        <div className="flex items-center gap-4">
                            <Button
                                onClick={handleTest}
                                disabled={isTesting || isConnected}
                                variant={isConnected ? 'outline' : 'default'}
                                className={
                                    isConnected
                                        ? 'border-green-500/50 text-green-400 hover:bg-green-500/10 hover:text-green-300'
                                        : ''
                                }
                            >
                                {isTesting ? (
                                    <RefreshCw className="w-4 h-4 animate-spin mr-2" />
                                ) : isConnected ? (
                                    <CheckCircle2 className="w-4 h-4 mr-2" />
                                ) : null}
                                {isConnected
                                    ? t('Connected')
                                    : t('Test Connection')}
                            </Button>

                            {error && (
                                <div className="flex items-center gap-2 text-red-400 text-sm animate-in fade-in slide-in-from-left-2">
                                    <AlertCircle className="w-4 h-4" />
                                    {error}
                                </div>
                            )}
                        </div>
                    </div>
                </div>
            </div>

            <div className="flex justify-between w-full pt-4">
                <div /> {/* Spacer */}
                <Button
                    onClick={onNext}
                    disabled={!isConnected}
                    size="lg"
                    className="px-8"
                >
                    {t('Next Step')}
                </Button>
            </div>
        </motion.div>
    );
};
