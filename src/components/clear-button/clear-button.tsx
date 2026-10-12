import { X } from 'lucide-react';
import { Button } from '@/components/button';
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/tooltip';

interface ClearButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
    label: string;
}

export const ClearButton = ({ label, ...props }: ClearButtonProps) => {
    return (
        <Tooltip>
            <TooltipTrigger asChild>
                <Button variant="subtle" size="icon-sm" aria-label={label} className="size-7" {...props}>
                    <X />
                </Button>
            </TooltipTrigger>
            <TooltipContent>{label}</TooltipContent>
        </Tooltip>
    );
};
