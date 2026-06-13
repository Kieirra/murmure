import { Undo2 } from 'lucide-react';
import { Button } from './button';

export const ResetButton = (props: React.HTMLAttributes<HTMLButtonElement>) => {
    return (
        <Button
            variant="link"
            size="icon-sm"
            aria-label="Reset"
            className="border border-transparent hover:border-border"
            {...props}
        >
            <Undo2 />
        </Button>
    );
};
