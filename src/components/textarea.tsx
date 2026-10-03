import * as React from 'react';

import { cn } from '@/components/lib/utils';

function Textarea({ className, ...props }: React.ComponentProps<'textarea'>) {
    return (
        <textarea
            data-slot="textarea"
            className={cn(
                'w-full bg-background border border-border rounded-md px-3 py-2 text-sm text-white placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-sky-500 min-h-[60px] resize-y',
                className
            )}
            {...props}
        />
    );
}

export { Textarea };
