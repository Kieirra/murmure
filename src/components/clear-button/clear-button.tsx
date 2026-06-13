import { X } from 'lucide-react';
import { Button } from '@/components/button';

export const ClearButton = (props: React.ButtonHTMLAttributes<HTMLButtonElement>) => {
    return (
        <Button
            variant="link"
            size="icon-sm"
            aria-label="Clear"
            className="border border-transparent hover:border-border"
            {...props}
        >
            <X />
        </Button>
    );
};
