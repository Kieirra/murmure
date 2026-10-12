import { Undo2 } from 'lucide-react';
import { Button } from './button';
import { Tooltip, TooltipContent, TooltipTrigger } from './tooltip';

interface ResetButtonProps extends React.HTMLAttributes<HTMLButtonElement> {
    label: string;
}

export const ResetButton = ({ label, ...props }: ResetButtonProps) => {
    return (
        <Tooltip>
            <TooltipTrigger asChild>
                <Button variant="subtle" size="icon-sm" aria-label={label} className="size-7" {...props}>
                    <Undo2 />
                </Button>
            </TooltipTrigger>
            <TooltipContent>{label}</TooltipContent>
        </Tooltip>
    );
};
