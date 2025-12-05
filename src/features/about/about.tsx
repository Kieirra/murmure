import { Shield, Lock, Code, Cpu, Github, BadgeEuro } from 'lucide-react';
import { Separator } from '../../components/separator';
import { Page } from '@/components/page';
import { Typography } from '@/components/typography';
import { Button } from '@/components/button';
import { useGetVersion } from '../layout/hooks/use-get-version';
import { useTranslation } from '@/i18n';

export const About = () => {
    const version = useGetVersion();
    const { t } = useTranslation();
    const features = [
        {
            icon: Lock,
            title: t('Privacy First'),
            description: t(
                'All processing happens locally on your device. No data ever leaves your computer.'
            ),
        },
        {
            icon: Shield,
            title: t('No Telemetry'),
            description: t(
                'Zero tracking, zero analytics. Your data stays yours, always.'
            ),
        },
        {
            icon: Code,
            title: t('Open Source'),
            description: t(
                'Free and open source software. Inspect, modify, and contribute.'
            ),
        },
        {
            icon: Cpu,
            title: t('Powered by Parakeet'),
            description: t(
                "NVIDIA's state-of-the-art speech recognition model runs entirely on-device."
            ),
        },
    ];

    return (
        <main className="space-y-8">
            <Page.Header>
                <Typography.MainTitle data-testid="about-title">
                    {t('Murmure')}
                </Typography.MainTitle>
                <Typography.Paragraph className="text-zinc-400">
                    {t(
                        'Privacy-first speech-to-text, running entirely on your machine'
                    )}
                </Typography.Paragraph>
            </Page.Header>
            <div className="space-y-8">
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                    {features.map((feature) => (
                        <div
                            key={feature.title}
                            className="rounded-lg border border-zinc-700 p-5 space-y-4"
                        >
                            <Typography.Title className="flex items-center gap-2">
                                <feature.icon className="w-4 h-4 text-zinc-400 inline-block" />
                                {feature.title}
                            </Typography.Title>
                            <Typography.Paragraph>
                                {feature.description}
                            </Typography.Paragraph>
                        </div>
                    ))}
                </div>

                <div className="space-y-8">
                    <div className="space-y-2">
                        <Typography.Title>{t('Technology')}</Typography.Title>
                        <Typography.Paragraph>
                            {t(
                                "Murmure uses NVIDIA's Parakeet TDT model, a highly optimized transformer-based speech recognition system designed for low-latency on-device inference."
                            )}
                        </Typography.Paragraph>
                    </div>

                    <div className="space-y-2">
                        <Typography.Title>{t('License')}</Typography.Title>
                        <Typography.Paragraph>
                            {t(
                                'Free and open source under GNU GPL v3 License.'
                            )}
                        </Typography.Paragraph>
                    </div>

                    <div className="flex items-center gap-4">
                        <Button variant="outline" asChild>
                            <a
                                href="https://github.com/Kieirra/murmure"
                                target="_blank"
                                rel="noopener noreferrer"
                                aria-label="View the Murmure project on GitHub"
                            >
                                <Github />
                                <span>{t('View on GitHub')}</span>
                            </a>
                        </Button>
                        <Page.PrimaryButton asChild>
                            <a
                                href="https://fr.tipeee.com/murmure-al1x-ai/"
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                <BadgeEuro />
                                <span>{t('Support Development')}</span>
                            </a>
                        </Page.PrimaryButton>
                    </div>
                </div>

                <Separator className="bg-zinc-700 my-2" />

                <div className="flex items-center gap-4">
                    <Typography.Paragraph className="text-xs text-zinc-500">
                        {t('Version {{version}}', { version })}
                    </Typography.Paragraph>
                    <span className="text-zinc-700">•</span>
                    <Typography.Paragraph className="text-xs text-zinc-500">
                        {t('Copyright (c) 2025 al1x-ai.com')}
                    </Typography.Paragraph>
                </div>
            </div>
        </main>
    );
};
