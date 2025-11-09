import { Outlet } from '@tanstack/react-router';
import { SidebarProvider, SidebarInset } from '../../components/sidebar';
import { AppSidebar } from './app-sidebar/app-sidebar';
import { Toaster } from '@/components/sonner';
import clsx from 'clsx';

export const Layout = () => {
    return (
        <SidebarProvider defaultOpen={true} className="bg-zinc-900 dark">
            <AppSidebar />
            <SidebarInset
                className={clsx(
                    'bg-zinc-900',
                    'text-white',
                    'pr-8',
                    'pt-8',
                    'flex',
                    'items-center',
                    'md:pl-[calc(var(--sidebar-width))]'
                )}
            >
                <div className="max-w-[800px] w-full ">
                    <Outlet />
                </div>
            </SidebarInset>
            <Toaster />
        </SidebarProvider>
    );
};
