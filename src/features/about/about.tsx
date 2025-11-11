import { Shield, Lock, Code, Cpu, Github, BadgeEuro } from 'lucide-react';
import { Separator } from '../../components/separator';
import { Page } from '@/components/page';
import { Typography } from '@/components/typography';
import { Button } from '@/components/button';
import { useGetVersion } from '../layout/hooks/use-get-version';
import { useTranslation } from '@/i18n';

export const About = () => {
    const version = useGetVersion();
    const { t } = useTranslation('about');
    const features = [
        {
            icon: Lock,
            title: t('features.privacyFirst.title'),
            description: t('features.privacyFirst.description'),
        },
        {
            icon: Shield,
            title: t('features.noTelemetry.title'),
            description: t('features.noTelemetry.description'),
        },
        {
            icon: Code,
            title: t('features.openSource.title'),
            description: t('features.openSource.description'),
        },
        {
            icon: Cpu,
            title: t('features.poweredByParakeet.title'),
            description: t('features.poweredByParakeet.description'),
        },
    ];

    return (
        <main className="space-y-8">
            <Page.Header>
                <Typography.MainTitle>{t('title')}</Typography.MainTitle>
                <Typography.Paragraph className="text-zinc-400">
                    {t('subtitle')}
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
                        <Typography.Title>{t('technology.title')}</Typography.Title>
                        <Typography.Paragraph>
                            {t('technology.description')}
                        </Typography.Paragraph>
                    </div>

                    <div className="space-y-2">
                        <Typography.Title>{t('license.title')}</Typography.Title>
                        <Typography.Paragraph>
                            {t('license.description')}
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
                                <span>{t('actions.viewOnGitHub')}</span>
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
                                <span>{t('actions.supportDevelopment')}</span>
                            </a>
                        </Button>
                    </div>
                </div>

                <Separator className="bg-zinc-700 my-2" />

                <div className="flex items-center gap-4">
                    <Typography.Paragraph className="text-xs text-zinc-500">
                        {t('footer.version', { version })}
                    </Typography.Paragraph>
                    <span className="text-zinc-700">•</span>
                    <Typography.Paragraph className="text-xs text-zinc-500">
                        {t('footer.copyright')}
                    </Typography.Paragraph>
                </div>
            </div>
        </main>
    );
};
