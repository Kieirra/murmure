import clsx from 'clsx';
import { type LucideIcon } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { Page } from '@/components/page';
import { Typography } from '@/components/typography';
import { Switch } from '@/components/switch';
import { PALETTES, type ColorScheme } from '@/features/home/audio-visualizer/audio-pixel/audio-pixel.helpers';

interface ExtensionHeaderToggle {
    checked: boolean;
    onCheckedChange: (checked: boolean) => void;
    ariaLabel: string;
    testId: string;
}

interface ExtensionHeaderProps {
    icon: LucideIcon;
    title: string;
    colorScheme: ColorScheme;
    titleTestId: string;
    subtitle?: string;
    toggle?: ExtensionHeaderToggle;
}

export const ExtensionHeader = ({
    icon: Icon,
    title,
    colorScheme,
    titleTestId,
    subtitle,
    toggle,
}: ExtensionHeaderProps) => {
    const { t } = useTranslation();
    const accentColor = PALETTES[colorScheme].accent;

    return (
        <Page.Header>
            <div className={clsx('flex items-center justify-between gap-4', subtitle != null && 'mb-4')}>
                <Typography.MainTitle className="flex items-center gap-2 !mb-0" data-testid={titleTestId}>
                    <Icon className="w-6 h-6" style={{ color: accentColor }} />
                    {title}
                </Typography.MainTitle>
                {toggle != null && (
                    <div className="flex items-center gap-3">
                        {toggle.checked && (
                            <span className="text-sm font-medium" style={{ color: accentColor }}>
                                {t('Active')}
                            </span>
                        )}
                        <Switch
                            checked={toggle.checked}
                            onCheckedChange={toggle.onCheckedChange}
                            aria-label={toggle.ariaLabel}
                            data-testid={toggle.testId}
                        />
                    </div>
                )}
            </div>
            {subtitle != null && <Typography.Paragraph>{subtitle}</Typography.Paragraph>}
        </Page.Header>
    );
};
