import { Typography } from '@/components/typography';
import { Mic, Shield, AudioWaveform, Send } from 'lucide-react';
import { useTranslation } from '@/i18n';

interface CtaCardProps {
    icon: React.ReactNode;
    title: string;
    description: string;
}

const CtaCard = ({ icon, title, description }: CtaCardProps) => (
    <div className="flex items-start gap-4">
        {icon}
        <div className="space-y-1">
            <Typography.Title>{title}</Typography.Title>
            <Typography.Paragraph>{description}</Typography.Paragraph>
        </div>
    </div>
);

export const VoiceModeCta = () => {
    const { t } = useTranslation();

    return (
        <section data-testid="voice-mode-cta">
            <div className="space-y-4">
                <div className="bg-sky-950/20 border border-sky-900/30 p-5 rounded-xl">
                    <CtaCard
                        icon={
                            <div className="bg-sky-950 p-2.5 rounded-lg shrink-0">
                                <Mic className="w-5 h-5 text-sky-400" />
                            </div>
                        }
                        title={t('Hands-free recording')}
                        description={t(
                            'Say a trigger word and start recording instantly, no keyboard needed.'
                        )}
                    />
                </div>

                <div className="bg-emerald-950/20 border border-emerald-900/30 p-5 rounded-xl">
                    <CtaCard
                        icon={
                            <div className="bg-emerald-950 p-2.5 rounded-lg shrink-0">
                                <Shield className="w-5 h-5 text-emerald-400" />
                            </div>
                        }
                        title={t('Privacy safe')}
                        description={t(
                            'Only short audio buffers are analyzed in memory, then immediately discarded. No audio is ever recorded or stored.'
                        )}
                    />
                </div>

                <div className="bg-violet-950/20 border border-violet-900/30 p-5 rounded-xl">
                    <CtaCard
                        icon={
                            <div className="bg-violet-950 p-2.5 rounded-lg shrink-0">
                                <AudioWaveform className="w-5 h-5 text-violet-400" />
                            </div>
                        }
                        title={t('Fully customizable')}
                        description={t(
                            'Choose a different trigger word for transcription, LLM processing, and voice commands.'
                        )}
                    />
                </div>

                <div className="bg-amber-950/20 border border-amber-900/30 p-5 rounded-xl">
                    <CtaCard
                        icon={
                            <div className="bg-amber-950 p-2.5 rounded-lg shrink-0">
                                <Send className="w-5 h-5 text-amber-400" />
                            </div>
                        }
                        title={t('Auto-send ready')}
                        description={t(
                            'Enable auto-press Enter and your transcriptions go straight into the conversation. Perfect for Claude Code, ChatGPT, or any chat app.'
                        )}
                    />
                </div>
            </div>
        </section>
    );
};
