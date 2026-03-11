import { useTranslation } from '@/i18n';
import { Typography } from '@/components/typography';
import { motion } from 'framer-motion';
import { CheckCircle2, RefreshCw, AlertCircle, AlertTriangle, Eye, EyeOff } from 'lucide-react';
import { useState } from 'react';
import { Page } from '@/components/page';
import { Input } from '@/components/input';
import { StepItem } from '@/components/step-item';
import clsx from 'clsx';
import { LLMConnectSettings } from '../../hooks/use-llm-connect';
import { DEFAULT_REMOTE_URL_PLACEHOLDER } from '../../llm-connect.constants';
import { isInsecureRemoteUrl } from '../../llm-connect.helpers';

interface StepRemoteConfigProps {
    onNext: () => void;
    testRemoteConnection: (url?: string) => Promise<number>;
    storeRemoteApiKey: (apiKey: string) => Promise<void>;
    updateSettings: (updates: Partial<LLMConnectSettings>) => Promise<void>;
}

export const StepRemoteConfig = ({
    onNext,
    testRemoteConnection,
    storeRemoteApiKey,
    updateSettings,
}: StepRemoteConfigProps) => {
    const { t } = useTranslation();
    const [url, setUrl] = useState('');
    const [apiKey, setApiKey] = useState('');
    const [showApiKey, setShowApiKey] = useState(false);
    const [isTesting, setIsTesting] = useState(false);
    const [isConnected, setIsConnected] = useState(false);
    const [modelCount, setModelCount] = useState(0);
    const [error, setError] = useState<string | null>(null);

    const handleTestConnection = async () => {
        setIsTesting(true);
        setError(null);
        try {
            await storeRemoteApiKey(apiKey);
            await updateSettings({
                remote_url: url,
                remote_privacy_acknowledged: true,
            });
            const count = await testRemoteConnection(url);
            setModelCount(count);
            setIsConnected(true);
        } catch (err: unknown) {
            const errorMessage = err instanceof Error ? err.message : String(err);
            setError(errorMessage || t('Could not connect. Check the URL and make sure the server is running.'));
            setIsConnected(false);
        } finally {
            setIsTesting(false);
        }
    };

    const renderTestButtonContent = () => {
        if (isTesting) {
            return (
                <>
                    <RefreshCw className="w-4 h-4 animate-spin mr-2" />
                    {t('Test Connection')}
                </>
            );
        }
        if (isConnected) {
            return (
                <>
                    <CheckCircle2 className="w-4 h-4 mr-2" />
                    {t('Connected')}
                </>
            );
        }
        return t('Test Connection');
    };

    return (
        <motion.div
            initial={{ opacity: 0, x: 20 }}
            animate={{ opacity: 1, x: 0 }}
            exit={{ opacity: 0, x: -20 }}
            className="flex flex-col items-center max-w-3xl mx-auto space-y-4 py-4"
        >
            <div className="text-center space-y-2">
                <Typography.MainTitle>{t('Configure Remote Server')}</Typography.MainTitle>
                <Typography.Paragraph className="text-muted-foreground">
                    {t('Connect to an OpenAI-compatible server to process your transcriptions remotely.')}
                </Typography.Paragraph>
            </div>

            <div className="w-full bg-card/30 border border-border rounded-xl p-6 space-y-5">
                <StepItem
                    step={1}
                    title={t('Server URL')}
                    description={t('Enter the base URL of your OpenAI-compatible server.')}
                    isActive={url.length > 0}
                >
                    <Input
                        value={url}
                        onChange={(e) => {
                            setUrl(e.target.value);
                            setIsConnected(false);
                        }}
                        placeholder={DEFAULT_REMOTE_URL_PLACEHOLDER}
                        className="w-full"
                        disabled={isTesting}
                    />
                </StepItem>

                <div className="w-full h-px bg-card" />

                <StepItem
                    step={2}
                    title={t('API Key (optional)')}
                    description={t("Leave empty if your server doesn't require authentication.")}
                    isActive={apiKey.length > 0}
                >
                    <div className="relative">
                        <Input
                            type={showApiKey ? 'text' : 'password'}
                            value={apiKey}
                            onChange={(e) => setApiKey(e.target.value)}
                            placeholder="sk-..."
                            className="w-full pr-10"
                            disabled={isTesting}
                        />
                        <button
                            type="button"
                            onClick={() => setShowApiKey(!showApiKey)}
                            className="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground transition-colors"
                        >
                            {showApiKey ? <EyeOff className="w-4 h-4" /> : <Eye className="w-4 h-4" />}
                        </button>
                    </div>
                </StepItem>

                <div className="w-full h-px bg-card" />

                <StepItem
                    step={3}
                    title={t('Verify Connection')}
                    description={t('Test the connection to your remote server.')}
                    isActive={isConnected}
                >
                    <div className="flex items-center gap-4">
                        <Page.SecondaryButton
                            onClick={handleTestConnection}
                            variant="outline"
                            disabled={url.length === 0 || isTesting}
                            className={clsx(
                                isConnected && 'text-emerald-500 hover:bg-emerald-400/10 hover:text-emerald-300'
                            )}
                        >
                            {renderTestButtonContent()}
                        </Page.SecondaryButton>

                        {isConnected && (
                            <span className="flex items-center gap-2 text-emerald-400 text-sm animate-in fade-in slide-in-from-left-2">
                                <CheckCircle2 className="w-4 h-4" />
                                {t('Connected')} — {modelCount} {t('models found')}
                            </span>
                        )}

                        {error && (
                            <div className="flex items-center gap-2 text-red-400 text-sm animate-in fade-in slide-in-from-left-2">
                                <AlertCircle className="w-4 h-4" />
                                {error}
                            </div>
                        )}
                    </div>
                </StepItem>
            </div>

            <div className="w-full space-y-2 pt-1">
                <div className="flex items-start gap-2 p-3 rounded-lg bg-amber-500/10 border border-amber-500/20 text-amber-500 text-xs">
                    <AlertTriangle className="w-3.5 h-3.5 mt-0.5 flex-shrink-0" />
                    <span>
                        {t(
                            'Your voice transcriptions will be sent to the remote server for processing. Make sure you trust this server.'
                        )}
                    </span>
                </div>

                {isInsecureRemoteUrl(url) && (
                    <div className="flex items-start gap-2 p-3 rounded-lg bg-amber-500/10 border border-amber-500/20 text-amber-500 text-xs animate-in fade-in">
                        <AlertTriangle className="w-3.5 h-3.5 mt-0.5 flex-shrink-0" />
                        <span>{t('This connection is not encrypted. Your data could be intercepted.')}</span>
                    </div>
                )}
            </div>

            <div className="flex justify-between w-full pt-2">
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
