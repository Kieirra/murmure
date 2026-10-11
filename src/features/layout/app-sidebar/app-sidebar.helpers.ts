interface SidebarItem {
    url: string;
}

export const isPathInItems = (pathname: string, items: SidebarItem[]): boolean =>
    items.some((item) => item.url === pathname);
