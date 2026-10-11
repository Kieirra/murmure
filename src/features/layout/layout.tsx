import { Outlet, useLocation } from '@tanstack/react-router';
import { SidebarProvider, SidebarInset } from '../../components/sidebar';
import { AppSidebar } from './app-sidebar/app-sidebar';
import clsx from 'clsx';
import { Bounce, ToastContainer } from 'react-toastify';
import { AccessibilityListener } from './listeners/accessibility-listener';
import { RecordingErrorListener } from './listeners/recording-error-listener';
import { LlmErrorListener } from './listeners/llm-error-listener';
import { LlmModeNotConfiguredListener } from './listeners/llm-mode-not-configured-listener';
import { ToastEventListener } from './listeners/toast-event-listener';
import { ConfigImportedListener } from './listeners/config-imported-listener';
import { WaylandListener } from './listeners/wayland-listener';
import { VoiceModeToggleListener } from './listeners/voice-mode-toggle-listener';
import { TrayMenuI18nListener } from './listeners/tray-menu-i18n-listener';

export const Layout = () => {
    const { pathname } = useLocation();
    return (
        <SidebarProvider open={true} onOpenChange={() => {}} className="bg-background dark">
            <AccessibilityListener />
            <RecordingErrorListener />
            <LlmErrorListener />
            <LlmModeNotConfiguredListener />
            <ToastEventListener />
            <ConfigImportedListener />
            <WaylandListener />
            <VoiceModeToggleListener />
            <TrayMenuI18nListener />
            <AppSidebar />
            <SidebarInset
                className={clsx('bg-background', 'text-white', 'pr-8', 'pt-8', 'flex', 'items-center', 'pl-[17.2rem]')}
            >
                {pathname === '/' && (
                    <div
                        aria-hidden="true"
                        className={clsx(
                            'pointer-events-none fixed inset-0 isolate',
                            'bg-[linear-gradient(to_bottom,#243f54_0%,#191b20_52%,#18191c_57%,var(--background)_62%)]'
                        )}
                    >
                        <div className="bg-grain absolute inset-0 opacity-20 mix-blend-overlay" />
                    </div>
                )}
                <div className="relative max-w-[800px] w-full pb-12" data-testid="murmure-content">
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
