import { Undo2 } from 'lucide-react';
import { Button } from './button';

export const ResetButton = (props: React.HTMLAttributes<HTMLButtonElement>) => {
    return (
        <Button
            variant="link"
            aria-label="Reset"
            className="hover:border-border hover:border p-2! border-transparent border"
            {...props}
        >
            <Undo2 />
        </Button>
    );
};
