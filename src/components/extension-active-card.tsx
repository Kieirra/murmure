import { LucideIcon } from 'lucide-react';
import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { Switch } from '@/components/switch';

interface ExtensionActiveCardProps {
    icon: LucideIcon;
    label: string;
    checked: boolean;
    onCheckedChange: (checked: boolean) => void;
    testId: string;
}

export const ExtensionActiveCard = ({ icon: Icon, label, checked, onCheckedChange, testId }: ExtensionActiveCardProps) => {
    return (
        <section>
            <SettingsUI.Container className="border-emerald-400/40 bg-linear-to-r from-emerald-900/20 to-transparent">
                <SettingsUI.Item>
                    <SettingsUI.Description>
                        <Typography.Title className="flex items-center gap-2">
                            <Icon className="w-4 h-4 text-emerald-400" />
                            {label}
                        </Typography.Title>
                    </SettingsUI.Description>
                    <Switch checked={checked} onCheckedChange={onCheckedChange} data-testid={testId} />
                </SettingsUI.Item>
            </SettingsUI.Container>
        </section>
    );
};
