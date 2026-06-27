import { Typography } from '@/components/typography';
import { LucideIcon } from 'lucide-react';

export type StatCardAccent = 'cyan' | 'sky' | 'indigo';

interface StatCardProps {
    icon: LucideIcon;
    value: string;
    label: string;
    accent: StatCardAccent;
    subtitle?: string;
    iconNudge?: 'up' | 'down' | 'down-sm';
}

const accentColorClasses: Record<StatCardAccent, string> = {
    cyan: 'text-cyan-300',
    sky: 'text-sky-500',
    indigo: 'text-indigo-400',
};

const accentValueClasses: Record<StatCardAccent, string> = {
    cyan: 'text-cyan-300!',
    sky: 'text-sky-500!',
    indigo: 'text-indigo-400!',
};

const iconNudgeClasses: Record<NonNullable<StatCardProps['iconNudge']>, string> = {
    up: '-translate-y-1',
    down: 'translate-y-1',
    'down-sm': 'translate-y-0.5',
};

export const StatCard = ({ icon: Icon, value, label, accent, subtitle, iconNudge }: StatCardProps) => {
    return (
        <div className="relative flex min-h-[6.5rem] flex-1 flex-col overflow-hidden rounded-md border border-border bg-gradient-to-t from-black/30 to-white/10 p-3 text-center">
            <Icon
                width={72}
                height={72}
                className={`pointer-events-none absolute left-1/2 top-2 -translate-x-1/2 [mask-image:linear-gradient(to_bottom,black_30%,transparent_85%)] ${iconNudge != null ? iconNudgeClasses[iconNudge] : ''} ${accentColorClasses[accent]}`}
            />
            <div className="absolute inset-x-0 bottom-2 z-10 flex h-2/5 flex-col items-center justify-start pb-3 pt-2">
                <div className="flex items-baseline gap-1">
                    <Typography.Title className={`font-bold! leading-none ${accentValueClasses[accent]}`}>
                        {value}
                    </Typography.Title>
                    <span className="text-xs text-muted-foreground">{label}</span>
                </div>
                {subtitle != null && <span className="text-xs text-muted-foreground">{subtitle}</span>}
            </div>
        </div>
    );
};
