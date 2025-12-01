import {
    Home,
    Settings,
    Info,
    ChevronRight,
    Keyboard,
    BookText,
    Power,
    Bug,
    Sparkles,
    Wrench,
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

const getPersonalizeSubItems = (t: (key: string) => string) => [
    {
        name: t('Custom Dictionary'),
        url: '/personalize/custom-dictionary',
        icon: BookText,
        dataTestId: 'dictionary-tab',
    },
    {
        name: t('LLM Connect'),
        url: '/personalize/llm-connect',
        icon: Sparkles,
        dataTestId: 'llm-connect-tab',
    },
];

const getSettingsSubItems = (t: (key: string) => string) => [
    {
        name: t('Shortcuts'),
        url: '/settings/shortcuts',
        icon: Keyboard,
        dataTestId: 'shortcuts-tab',
    },
    {
        name: t('System'),
        url: '/settings/system',
        icon: Power,
        dataTestId: 'system-tab',
    },
];

export const AppSidebar = () => {
    const { pathname } = useLocation();
    const [personalizeOpen, setPersonalizeOpen] = useState(false);
    const [settingsOpen, setSettingsOpen] = useState(false);
    const version = useGetVersion();
    const { t } = useTranslation();
    const personalizeSubItems = getPersonalizeSubItems(t);
    const settingsSubItems = getSettingsSubItems(t);

    return (
        <Sidebar className="bg-zinc-900 border-zinc-700 border-r overflow-hidden w-[14.3rem]">
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
                                data-testid="home-tab"
                            >
                                <Link to="/">
                                    <Home />
                                    <span>{t('Home')}</span>
                                </Link>
                            </SidebarMenuButton>
                        </SidebarMenuItem>

                        <SidebarMenuItem>
                            <SidebarMenuButton
                                onClick={() =>
                                    setPersonalizeOpen(!personalizeOpen)
                                }
                                data-testid="personalize-tab"
                            >
                                <Wrench />
                                <span>{t('Personalize')}</span>
                                <ChevronRight
                                    className={`ml-auto transition-transform ${personalizeOpen ? 'rotate-90' : ''}`}
                                />
                            </SidebarMenuButton>
                            {personalizeOpen && (
                                <SidebarMenuSub>
                                    {personalizeSubItems.map((item) => (
                                        <SidebarMenuSubItem
                                            key={item.url}
                                            data-testid={item.dataTestId}
                                        >
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
                                onClick={() => setSettingsOpen(!settingsOpen)}
                                data-testid="settings-tab"
                            >
                                <Settings />
                                <span>{t('Settings')}</span>
                                <ChevronRight
                                    className={`ml-auto transition-transform ${settingsOpen ? 'rotate-90' : ''}`}
                                />
                            </SidebarMenuButton>
                            {settingsOpen && (
                                <SidebarMenuSub>
                                    {settingsSubItems.map((item) => (
                                        <SidebarMenuSubItem
                                            key={item.url}
                                            data-testid={item.dataTestId}
                                        >
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
                                data-testid="about-tab"
                            >
                                <Link to="/about">
                                    <Info />
                                    <span>{t('About')}</span>
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
                    <span>{t('Report a bug')}</span>
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
