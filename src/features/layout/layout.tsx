import { Outlet } from '@tanstack/react-router';
import { SidebarProvider, SidebarInset } from '../../components/sidebar';
import { AppSidebar } from './app-sidebar/app-sidebar';
import clsx from 'clsx';
import { Bounce, ToastContainer } from 'react-toastify';

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
                    'pl-[16rem]'
                )}
            >
                <div
                    className="max-w-[800px] w-full"
                    data-testid="murmure-content"
                >
                    <Outlet />
                </div>
            </SidebarInset>
            <ToastContainer
                position="bottom-right"
                autoClose={3000}
                hideProgressBar={false}
                newestOnTop={false}
                closeOnClick={false}
                rtl={false}
                pauseOnFocusLoss
                draggable
                pauseOnHover
                theme="dark"
                transition={Bounce}
            />
        </SidebarProvider>
    );
};
