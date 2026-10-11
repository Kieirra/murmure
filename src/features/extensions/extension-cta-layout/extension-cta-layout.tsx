import { type ReactNode } from 'react';
import { type ColorScheme } from '@/features/home/audio-visualizer/audio-pixel/audio-pixel.helpers';
import { buildExtensionHaloGradient } from './extension-cta-layout.helpers';

interface ExtensionCtaLayoutProps {
    colorScheme: ColorScheme;
    haloIntensity?: number;
    children: ReactNode;
}

export const ExtensionCtaLayout = ({ colorScheme, haloIntensity = 20, children }: ExtensionCtaLayoutProps) => (
    <main className="relative isolate">
        <div
            aria-hidden="true"
            className="pointer-events-none absolute inset-x-0 -top-8 h-96"
            style={{ backgroundImage: buildExtensionHaloGradient(colorScheme, haloIntensity) }}
        />
        <div className="relative space-y-6">{children}</div>
    </main>
);
