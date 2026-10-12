import * as React from 'react';

import { cn } from '@/components/lib/utils';

function Textarea({ className, ...props }: React.ComponentProps<'textarea'>) {
    return (
        <textarea
            data-slot="textarea"
            className={cn(
                'w-full bg-field border border-input rounded-md px-3 py-2 text-sm text-white placeholder:text-zinc-500 outline-none focus-visible:border-sky-500 focus-visible:ring-[3px] focus-visible:ring-sky-500/25 min-h-[60px] resize-y',
                className
            )}
            {...props}
        />
    );
}

export { Textarea };
