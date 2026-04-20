import { useState } from 'react';
import { SettingsUI } from '@/components/settings-ui';
import { Settings2, ChevronDown, ChevronUp } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { PairedDevicesList } from './settings/paired-devices-list/paired-devices-list';
import { ServerSettings } from './settings/server-settings/server-settings';
import { BindAddressSettings } from './settings/bind-address-settings/bind-address-settings';
import { TokenTtlSettings } from './settings/token-ttl-settings/token-ttl-settings';
import { RelaySettings } from './settings/relay-settings/relay-settings';
import type { PairedDevice } from './hooks/use-paired-devices';
import type { NetworkInterface } from './hooks/use-bind-address';

interface SmartMicSettingsProps {
    pairedDevices: {
        devices: PairedDevice[];
        remove: (token: string) => void;
    };
    server: {
        port: number;
        setPort: (value: number) => void;
    };
    bindAddress: {
        bindAddress: string | null;
        availableInterfaces: NetworkInterface[];
        setBindAddress: (value: string | null) => void;
    };
    tokenTtl: {
        tokenTtlHours: number;
        setTokenTtlHours: (value: number) => void;
    };
    relay: {
        relayEnabled: boolean;
        relayUrl: string;
        machineIdEnabled: boolean;
        machineId: string;
        setRelayEnabled: (value: boolean) => void;
        setRelayUrl: (value: string) => void;
        setMachineIdEnabled: (value: boolean) => void;
        setMachineId: (value: string) => void;
        handleRelayUrlBlur: () => void;
        handleMachineIdBlur: () => void;
    };
}

export const SmartMicSettings = ({ pairedDevices, server, bindAddress, tokenTtl, relay }: SmartMicSettingsProps) => {
    const [isAdvancedOpen, setIsAdvancedOpen] = useState<boolean>(false);
    const { t } = useTranslation();

    const toggleAdvanced = () => setIsAdvancedOpen((prev) => !prev);

    return (
        <>
            <PairedDevicesList devices={pairedDevices.devices} remove={pairedDevices.remove} />
            <div className="p-4">
                <button
                    type="button"
                    onClick={toggleAdvanced}
                    className="flex items-center gap-2 text-sm font-medium text-foreground hover:text-foreground transition-colors w-fit cursor-pointer"
                >
                    <Settings2 className="w-4 h-4" />
                    {t('Advanced Settings')}
                    {isAdvancedOpen ? (
                        <ChevronUp className="w-4 h-4 text-muted-foreground" />
                    ) : (
                        <ChevronDown className="w-4 h-4 text-muted-foreground" />
                    )}
                </button>
                {isAdvancedOpen && (
                    <div className="mt-4 flex flex-col">
                        <ServerSettings port={server.port} setPort={server.setPort} />
                        <SettingsUI.Separator />
                        <BindAddressSettings
                            bindAddress={bindAddress.bindAddress}
                            availableInterfaces={bindAddress.availableInterfaces}
                            setBindAddress={bindAddress.setBindAddress}
                        />
                        <SettingsUI.Separator />
                        <TokenTtlSettings
                            tokenTtlHours={tokenTtl.tokenTtlHours}
                            setTokenTtlHours={tokenTtl.setTokenTtlHours}
                        />
                        <SettingsUI.Separator />
                        <RelaySettings {...relay} />
                    </div>
                )}
            </div>
        </>
    );
};
