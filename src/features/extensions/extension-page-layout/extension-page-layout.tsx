import { type ReactNode } from 'react';
import { type ColorScheme } from '@/components/lib/color-scheme';
import { buildExtensionHaloGradient } from './extension-page-layout.helpers';

interface ExtensionPageLayoutProps {
    colorScheme: ColorScheme;
    isActive: boolean;
    children: ReactNode;
}

export const ExtensionPageLayout = ({ colorScheme, isActive, children }: ExtensionPageLayoutProps) => (
    <main className="relative isolate">
        <div
            aria-hidden="true"
            className="pointer-events-none absolute inset-x-0 -top-8 h-96"
            style={{ backgroundImage: buildExtensionHaloGradient(colorScheme, isActive) }}
        />
        <div className="relative space-y-6">{children}</div>
    </main>
);
