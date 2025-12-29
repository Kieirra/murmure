import { Outlet } from '@tanstack/react-router';
import { useEffect } from 'react';
import { emit } from '@tauri-apps/api/event';
import { SidebarProvider, SidebarInset } from '../../components/sidebar';
import { AppSidebar } from './app-sidebar/app-sidebar';
import clsx from 'clsx';
import { Bounce, ToastContainer } from 'react-toastify';

export const Layout = () => {
    useEffect(() => {
        const handleKeyDown = (e: KeyboardEvent) => {
            if (!e.repeat) {
                emit('internal:shortcut-event', {
                    event_type: 'press',
                    key: e.code,
                });
            }
        };
        const handleKeyUp = (e: KeyboardEvent) => {
            emit('internal:shortcut-event', {
                event_type: 'release',
                key: e.code,
            });
        };

        window.addEventListener('keydown', handleKeyDown);
        window.addEventListener('keyup', handleKeyUp);

        return () => {
            window.removeEventListener('keydown', handleKeyDown);
            window.removeEventListener('keyup', handleKeyUp);
        };
    }, []);

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
