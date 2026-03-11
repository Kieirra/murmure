import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface UseAdvancedSettingsParams {
    url: string;
    remoteUrl: string;
    onUrlChange: (url: string) => void;
    onRemoteUrlChange: (url: string) => void;
    onTestConnection: (url: string) => Promise<void>;
    onTestRemoteConnection: (url: string) => Promise<number>;
    onApiKeyChange: (apiKey: string) => Promise<void>;
}

export const useAdvancedSettings = ({
    url,
    remoteUrl,
    onUrlChange,
    onRemoteUrlChange,
    onTestConnection,
    onTestRemoteConnection,
    onApiKeyChange,
}: UseAdvancedSettingsParams) => {
    const [isOpen, setIsOpen] = useState(false);
    const [showApiKey, setShowApiKey] = useState(false);
    const [apiKeyValue, setApiKeyValue] = useState('');
    const [isApiKeyDirty, setIsApiKeyDirty] = useState(false);
    const [isTesting, setIsTesting] = useState(false);
    const [isTestingLocal, setIsTestingLocal] = useState(false);
    const [remoteError, setRemoteError] = useState<string | null>(null);
    const [remoteModelCount, setRemoteModelCount] = useState<number | null>(null);
    const [localUrl, setLocalUrl] = useState(url);
    const [localRemoteUrl, setLocalRemoteUrl] = useState(remoteUrl);

    useEffect(() => {
        setLocalUrl(url);
    }, [url]);

    useEffect(() => {
        setLocalRemoteUrl(remoteUrl);
    }, [remoteUrl]);

    useEffect(() => {
        const loadMaskedKey = async () => {
            try {
                const maskedKey = await invoke<string>('get_remote_api_key_masked');
                setApiKeyValue(maskedKey);
            } catch {
                // No key stored
            }
        };
        loadMaskedKey();
    }, []);

    const handleApiKeyChange = (value: string) => {
        setApiKeyValue(value);
        setIsApiKeyDirty(true);
    };

    const handleApiKeyBlur = async () => {
        if (isApiKeyDirty) {
            await onApiKeyChange(apiKeyValue);
            setIsApiKeyDirty(false);
            try {
                const maskedKey = await invoke<string>('get_remote_api_key_masked');
                setApiKeyValue(maskedKey);
            } catch {
                setApiKeyValue('');
            }
        }
    };

    const handleLocalUrlBlur = () => {
        if (localUrl !== url) {
            onUrlChange(localUrl);
        }
    };

    const handleRemoteUrlBlur = () => {
        if (localRemoteUrl !== remoteUrl) {
            onRemoteUrlChange(localRemoteUrl);
        }
    };

    const handleTestLocal = async () => {
        setIsTestingLocal(true);
        try {
            const testUrl = localUrl;
            if (localUrl !== url) {
                onUrlChange(localUrl);
            }
            await onTestConnection(testUrl);
        } finally {
            setIsTestingLocal(false);
        }
    };

    const handleTestRemote = async () => {
        setIsTesting(true);
        setRemoteError(null);
        setRemoteModelCount(null);
        try {
            const testUrl = localRemoteUrl;
            if (localRemoteUrl !== remoteUrl) {
                onRemoteUrlChange(localRemoteUrl);
            }
            if (isApiKeyDirty) {
                await onApiKeyChange(apiKeyValue);
                setIsApiKeyDirty(false);
            }
            const modelCount = await onTestRemoteConnection(testUrl);
            setRemoteModelCount(modelCount);
        } catch (err: unknown) {
            const errorMessage = err instanceof Error ? err.message : String(err);
            setRemoteError(errorMessage);
        } finally {
            setIsTesting(false);
        }
    };

    const toggleOpen = () => setIsOpen(!isOpen);
    const toggleShowApiKey = () => setShowApiKey(!showApiKey);

    return {
        isOpen,
        toggleOpen,
        showApiKey,
        toggleShowApiKey,
        apiKeyValue,
        isTesting,
        isTestingLocal,
        remoteError,
        remoteModelCount,
        localUrl,
        setLocalUrl,
        localRemoteUrl,
        setLocalRemoteUrl,
        handleApiKeyChange,
        handleApiKeyBlur,
        handleLocalUrlBlur,
        handleRemoteUrlBlur,
        handleTestLocal,
        handleTestRemote,
    };
};
