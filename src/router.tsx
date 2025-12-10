import {
    createRouter,
    createRoute,
    createRootRoute,
    Navigate,
} from '@tanstack/react-router';
import { Home } from './features/home/home';
import { Layout } from './features/layout/layout';
import { About } from './features/about/about';
import { Shortcuts } from './features/settings/shortcuts/shortcuts';
import { CustomDictionary } from './features/settings/custom-dictionary/custom-dictionary';
import { FormattingRules } from './features/settings/formatting-rules/formatting-rules';
import { System } from './features/settings/system/system';
import { LLMConnect } from './features/llm-connect/llm-connect';

const rootRoute = createRootRoute({
    component: () => <Layout />,
});

const indexRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: '/',
    component: Home,
});

const settingsShortcutsRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: '/settings/shortcuts',
    component: Shortcuts,
});

const personalizeCustomDictionaryRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: '/personalize/custom-dictionary',
    component: CustomDictionary,
});

const personalizeFormattingRulesRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: '/personalize/formatting-rules',
    component: FormattingRules,
});

const personalizeLLMConnectRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: '/personalize/llm-connect',
    component: LLMConnect,
});

const settingsSystemRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: '/settings/system',
    component: System,
});

const settingsIndexRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: '/settings',
    component: () => <Navigate to="/settings/shortcuts" />,
});

const personalizeIndexRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: '/personalize',
    component: () => <Navigate to="/personalize/custom-dictionary" />,
});

const aboutRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: '/about',
    component: About,
});

const routeTree = rootRoute.addChildren([
    indexRoute,
    settingsIndexRoute,
    settingsShortcutsRoute,
    settingsSystemRoute,
    personalizeIndexRoute,
    personalizeCustomDictionaryRoute,
    personalizeFormattingRulesRoute,
    personalizeLLMConnectRoute,
    aboutRoute,
]);

export const router = createRouter({ routeTree });
