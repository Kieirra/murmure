import { Shield, Lock, Code, Cpu, Github, BadgeEuro } from 'lucide-react';
import { Separator } from '../../components/separator';
import { Page } from '@/components/page';
import { Typography } from '@/components/typography';
import { Button } from '@/components/button';
import { useGetVersion } from '../layout/hooks/use-get-version';

export const About = () => {
    const version = useGetVersion();
    const features = [
        {
            icon: Lock,
            title: 'Privacy First',
            description:
                'All processing happens locally on your device. No data ever leaves your computer.',
        },
        {
            icon: Shield,
            title: 'No Telemetry',
            description:
                'Zero tracking, zero analytics. Your data stays yours, always.',
        },
        {
            icon: Code,
            title: 'Open Source',
            description:
                'Free and open source software. Inspect, modify, and contribute.',
        },
        {
            icon: Cpu,
            title: 'Powered by Parakeet',
            description:
                "NVIDIA's state-of-the-art speech recognition model runs entirely on-device.",
        },
    ];

    return (
        <main className="space-y-8">
            <Page.Header>
                <Typography.MainTitle>Murmure</Typography.MainTitle>
                <Typography.Paragraph className="text-zinc-400">
                    Privacy-first speech-to-text, running entirely on your
                    machine
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
                        <Typography.Title>Technology</Typography.Title>
                        <Typography.Paragraph>
                            Murmure uses NVIDIA's Parakeet TDT model, a highly
                            optimized transformer-based speech recognition
                            system designed for low-latency on-device inference.
                        </Typography.Paragraph>
                    </div>

                    <div className="space-y-2">
                        <Typography.Title>License</Typography.Title>
                        <Typography.Paragraph>
                            Free and open source under GNU GPL v3 License.
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
                                <span>View on GitHub</span>
                            </a>
                        </Button>
                        <Button
                            variant="outline"
                            asChild
                            className="bg-gradient-to-r from-indigo-800 to-sky-700 hover:from-indigo-500 hover:to-sky-400"
                        >
                            <a
                                href="https://fr.tipeee.com/murmure-al1x-ai/"
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                <BadgeEuro />
                                <span>Support Development</span>
                            </a>
                        </Button>
                    </div>
                </div>

                <Separator className="bg-zinc-700 my-2" />

                <div className="flex items-center gap-4">
                    <Typography.Paragraph className="text-xs text-zinc-500">
                        Version {version}
                    </Typography.Paragraph>
                    <span className="text-zinc-700">•</span>
                    <Typography.Paragraph className="text-xs text-zinc-500">
                        Copyright (c) 2025 al1x-ai.com
                    </Typography.Paragraph>
                </div>
            </div>
        </main>
    );
};
