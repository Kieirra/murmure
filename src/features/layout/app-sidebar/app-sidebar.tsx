import {
    Home,
    Settings,
    Info,
    ChevronRight,
    Keyboard,
    BookText,
    Power,
    Bug,
} from 'lucide-react';
import { Link } from '@tanstack/react-router';
import { useState } from 'react';
import {
    Sidebar,
    SidebarHeader,
    SidebarContent,
    SidebarFooter,
    SidebarGroup,
    SidebarMenu,
    SidebarMenuItem,
    SidebarMenuButton,
    SidebarMenuSub,
    SidebarMenuSubItem,
    SidebarMenuSubButton,
} from '../../../components/sidebar';
import { useLocation } from '@tanstack/react-router';
import { useGetVersion } from '../hooks/use-get-version';
import { UpdateChecker } from '../../update-checker/update-checker';
import { Separator } from '@/components/separator';
import { useTranslation } from '@/i18n';

const getSettingsSubItems = (t: (key: string) => string) => [
    { name: t('navigation:shortcuts'), url: '/settings/shortcuts', icon: Keyboard },
    {
        name: t('navigation:customDictionary'),
        url: '/settings/custom-dictionary',
        icon: BookText,
    },
    { name: t('navigation:system'), url: '/settings/system', icon: Power },
];

export const AppSidebar = () => {
    const { pathname } = useLocation();
    const [settingsOpen, setSettingsOpen] = useState(true);
    const version = useGetVersion();
    const { t } = useTranslation('navigation');
    const settingsSubItems = getSettingsSubItems(t);

    return (
        <Sidebar className="bg-zinc-900 border-zinc-700 border-r">
            <SidebarHeader className="flex items-center justify-center bg-zinc-900 border-b border-zinc-700">
                <img src="app-icon.png" alt="logo" className="w-16 h-16" />
            </SidebarHeader>
            <SidebarContent className="bg-zinc-900">
                <SidebarGroup>
                    <SidebarMenu>
                        <SidebarMenuItem>
                            <SidebarMenuButton
                                asChild
                                isActive={pathname === '/'}
                            >
                                <Link to="/">
                                    <Home />
                                    <span>{t('home')}</span>
                                </Link>
                            </SidebarMenuButton>
                        </SidebarMenuItem>

                        <SidebarMenuItem>
                            <SidebarMenuButton
                                onClick={() => setSettingsOpen(!settingsOpen)}
                            >
                                <Settings />
                                <span>{t('settings')}</span>
                                <ChevronRight
                                    className={`ml-auto transition-transform ${settingsOpen ? 'rotate-90' : ''}`}
                                />
                            </SidebarMenuButton>
                            {settingsOpen && (
                                <SidebarMenuSub>
                                    {settingsSubItems.map((item) => (
                                        <SidebarMenuSubItem key={item.url}>
                                            <SidebarMenuSubButton
                                                asChild
                                                isActive={pathname === item.url}
                                            >
                                                <Link to={item.url}>
                                                    <item.icon />
                                                    <span>{item.name}</span>
                                                </Link>
                                            </SidebarMenuSubButton>
                                        </SidebarMenuSubItem>
                                    ))}
                                </SidebarMenuSub>
                            )}
                        </SidebarMenuItem>

                        <SidebarMenuItem>
                            <SidebarMenuButton
                                asChild
                                isActive={pathname === '/about'}
                            >
                                <Link to="/about">
                                    <Info />
                                    <span>{t('about')}</span>
                                </Link>
                            </SidebarMenuButton>
                        </SidebarMenuItem>
                    </SidebarMenu>
                </SidebarGroup>
            </SidebarContent>
            <SidebarFooter className="bg-zinc-900 ">
                <a
                    href="https://github.com/Kieirra/murmure/issues/new"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-zinc-500 text-xs hover:text-zinc-300 transition-colors flex items-center gap-2 justify-center"
                >
                    <Bug className="w-4 h-4" />
                    <span>{t('reportBug')}</span>
                </a>
                <Separator />
                <div className="flex items-center gap-2 justify-center">
                    <UpdateChecker />
                    <p className="text-xs text-zinc-500">{version}</p>
                </div>
            </SidebarFooter>
        </Sidebar>
    );
};
