import { ReactNode } from 'react';
import { LucideIcon } from 'lucide-react';

interface NoticeRowProps {
    icon: LucideIcon;
    title: string;
    children: ReactNode;
}

export const NoticeRow = ({ icon: Icon, title, children }: NoticeRowProps) => {
    return (
        <div className="flex items-start gap-3 pr-6">
            <div className="w-8 h-8 bg-yellow-300/20 rounded-full flex items-center justify-center flex-shrink-0">
                <Icon className="w-4 h-4 text-yellow-300" />
            </div>
            <div className="space-y-1">
                <p className="text-yellow-300 font-semibold text-sm">{title}</p>
                <p className="text-foreground text-xs">{children}</p>
            </div>
        </div>
    );
};
