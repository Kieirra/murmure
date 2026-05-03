import {
    Home,
    Settings,
    Heart,
    ChevronRight,
    Keyboard,
    BookText,
    Power,
    Bug,
    Sparkles,
    Wrench,
    AlignLeft,
    Newspaper,
    Mic,
    ArrowDownUp,
    Puzzle,
    Smartphone,
} from 'lucide-react';
import { Link } from '@tanstack/react-router';
import { useEffect, useState } from 'react';
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
import appIcon from '@/assets/app-icon.png';

const getPersonalizeSubItems = (t: (key: string) => string) => [
    {
        name: t('Custom Dictionary'),
        url: '/personalize/custom-dictionary',
        icon: BookText,
        dataTestId: 'dictionary-tab',
    },
    {
        name: t('Formatting Rules'),
        url: '/personalize/formatting-rules',
        icon: AlignLeft,
        dataTestId: 'formatting-rules-tab',
    },
];

const getExtensionsSubItems = (t: (key: string) => string) => [
    {
        name: t('LLM Connect'),
        url: '/extensions/llm-connect',
        icon: Sparkles,
        dataTestId: 'llm-connect-tab',
    },
    {
        name: t('Voice Mode'),
        url: '/extensions/voice-mode',
        icon: Mic,
        dataTestId: 'voice-mode-tab',
    },
    {
        name: t('Smart Mic'),
        url: '/extensions/smart-mic',
        icon: Smartphone,
        dataTestId: 'smart-mic-tab',
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
    {
        name: t('Import / Export'),
        url: '/settings/import-export',
        icon: ArrowDownUp,
        dataTestId: 'import-export-tab',
    },
];

export const AppSidebar = () => {
    const { pathname } = useLocation();
    const [personalizeOpen, setPersonalizeOpen] = useState(pathname.startsWith('/personalize'));
    const [extensionsOpen, setExtensionsOpen] = useState(pathname.startsWith('/extensions'));
    const [settingsOpen, setSettingsOpen] = useState(pathname.startsWith('/settings'));

    useEffect(() => {
        if (pathname.startsWith('/extensions')) {
            setExtensionsOpen(true);
        }
    }, [pathname]);
    const version = useGetVersion();
    const { t } = useTranslation();
    const personalizeSubItems = getPersonalizeSubItems(t);
    const extensionsSubItems = getExtensionsSubItems(t);
    const settingsSubItems = getSettingsSubItems(t);

    return (
        <Sidebar className="bg-background border-border border-r overflow-hidden w-[14.3rem]">
            <SidebarHeader className="flex items-center justify-center bg-background border-b border-border">
                <img src={appIcon} alt="logo" className="w-16 h-16" />
            </SidebarHeader>
            <SidebarContent className="bg-background">
                <SidebarGroup>
                    <SidebarMenu>
                        <SidebarMenuItem>
                            <SidebarMenuButton asChild isActive={pathname === '/'} data-testid="home-tab">
                                <Link to="/">
                                    <Home />
                                    <span>{t('Home')}</span>
                                </Link>
                            </SidebarMenuButton>
                        </SidebarMenuItem>

                        <SidebarMenuItem>
                            <SidebarMenuButton
                                onClick={() => setPersonalizeOpen(!personalizeOpen)}
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
                                        <SidebarMenuSubItem key={item.url} data-testid={item.dataTestId}>
                                            <SidebarMenuSubButton asChild isActive={pathname === item.url}>
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
                                onClick={() => setExtensionsOpen(!extensionsOpen)}
                                data-testid="extensions-tab"
                            >
                                <Puzzle />
                                <span>{t('Extensions')}</span>
                                <ChevronRight
                                    className={`ml-auto transition-transform ${extensionsOpen ? 'rotate-90' : ''}`}
                                />
                            </SidebarMenuButton>
                            {extensionsOpen && (
                                <SidebarMenuSub>
                                    {extensionsSubItems.map((item) => (
                                        <SidebarMenuSubItem key={item.url} data-testid={item.dataTestId}>
                                            <SidebarMenuSubButton asChild isActive={pathname === item.url}>
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
                                        <SidebarMenuSubItem key={item.url} data-testid={item.dataTestId}>
                                            <SidebarMenuSubButton asChild isActive={pathname === item.url}>
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
                    </SidebarMenu>
                </SidebarGroup>
            </SidebarContent>
            <SidebarFooter className="bg-background ">
                <a
                    href={
                        version.length > 0
                            ? `https://github.com/Kieirra/murmure/releases/tag/${version}`
                            : 'https://github.com/Kieirra/murmure/releases/latest'
                    }
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-muted-foreground text-xs hover:text-foreground transition-colors flex items-center gap-2 px-2"
                >
                    <Newspaper className="w-4 h-4" />
                    <span>{t('Release notes')}</span>
                </a>
                <a
                    href="https://github.com/Kieirra/murmure/issues/new"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-muted-foreground text-xs hover:text-foreground transition-colors flex items-center gap-2 px-2"
                >
                    <Bug className="w-4 h-4" />
                    <span>{t('Report a bug')}</span>
                </a>
                <Link
                    to="/about"
                    className="text-muted-foreground text-xs hover:text-foreground transition-colors flex items-center gap-2 px-2"
                    data-testid="about-tab"
                >
                    <Heart className="w-4 h-4 text-rose-300/70" />
                    <span>{t('Donate')}</span>
                </Link>
                <Separator />
                <div className="flex items-center gap-2 justify-center">
                    <UpdateChecker />
                    <p className="text-xs text-muted-foreground">{version}</p>
                </div>
            </SidebarFooter>
        </Sidebar>
    );
};
