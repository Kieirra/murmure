import clsx from 'clsx';
import { ArrowDown, ArrowRight } from 'lucide-react';

interface PreviewArrowProps {
    horizontalFrom?: 'md' | 'lg';
}

export const PreviewArrow = ({ horizontalFrom = 'md' }: PreviewArrowProps) => (
    <div className="flex justify-center self-center text-muted-foreground">
        <ArrowDown className={clsx('w-4 h-4', horizontalFrom === 'md' ? 'md:hidden' : 'lg:hidden')} />
        <ArrowRight className={clsx('hidden w-4 h-4', horizontalFrom === 'md' ? 'md:block' : 'lg:block')} />
    </div>
);
