import { useState } from 'react';
import clsx from 'clsx';
import { AlertTriangle, Play } from 'lucide-react';
import { Button } from '@/components/button';
import { InternalLink } from '@/components/internal-link';
import { Typography } from '@/components/typography';
import type { LlmExtensionsEnabled } from '@/features/extensions/llm-connect/hooks/use-llm-extensions-enabled';
import { CliCommandRow } from '@/features/settings/shortcuts/shortcuts-cli/cli-commands-panel/cli-command-row/cli-command-row';
import { useTranslation } from '@/i18n';
import type { EndpointDefinition } from '../../local-api.types';
import { CodeBlock } from '../../code-block/code-block';
import { buildCurl, buildEndpointUrl } from '../../local-api.helpers';
import { useEndpointTry } from '../hooks/use-endpoint-try';
import { EndpointFieldControl } from './endpoint-field-control/endpoint-field-control';
import {
    buildFormData,
    formatResponseBody,
    getInitialValues,
    getPromptNames,
    getStatusTone,
    getTryBlockedReason,
} from '../local-api-endpoints.helpers';

interface EndpointDocProps {
    definition: EndpointDefinition;
    port: number;
    isServerRunning: boolean;
    availability: LlmExtensionsEnabled;
}

export const EndpointDoc = ({ definition, port, isServerRunning, availability }: EndpointDocProps) => {
    const { t } = useTranslation();
    const [values, setValues] = useState(() => getInitialValues(definition.fields, availability.modes));
    const [file, setFile] = useState<File | null>(null);
    const { state, execute } = useEndpointTry();

    const url = buildEndpointUrl(definition, port);
    const promptNames = getPromptNames(availability.modes);
    const blockedReason = getTryBlockedReason(definition.fields, values, file, isServerRunning);
    const isRunning = state.phase === 'running';
    const statusTone = state.phase === 'done' ? getStatusTone(state.status) : undefined;
    const testIdPrefix = `local-api-endpoint-${definition.path.slice(1).replaceAll('/', '-')}`;

    const setValue = (name: string, value: string) => setValues((previous) => ({ ...previous, [name]: value }));

    const handleTry = () => {
        void execute({ url, method: definition.method, body: buildFormData(definition.fields, values, file) });
    };

    return (
        <article className="border border-border rounded-md p-4 space-y-4" data-testid={testIdPrefix}>
            <div className="flex items-center gap-2 min-w-0">
                <span
                    className={clsx(
                        'font-mono text-xs font-bold px-2 py-0.5 rounded border',
                        definition.method === 'POST' && 'bg-emerald-500/15 text-emerald-400 border-emerald-500/30',
                        definition.method === 'GET' && 'bg-sky-500/15 text-sky-400 border-sky-500/30'
                    )}
                >
                    {definition.method}
                </span>
                <code className="font-mono text-sm text-foreground break-all">{url}</code>
            </div>

            <div className="space-y-1">
                <Typography.Title>{t(definition.title)}</Typography.Title>
                <Typography.Paragraph>{t(definition.description)}</Typography.Paragraph>
                {definition.requiresPromptMode && !availability.llmConnectEnabled && (
                    <div className="flex items-center gap-1.5 text-xs text-yellow-300">
                        <AlertTriangle className="w-3 h-3 shrink-0" />
                        {t('Requires Prompt Mode')}
                        <InternalLink to="/extensions/llm-connect">{t('Enable Prompt Mode')}</InternalLink>
                    </div>
                )}
            </div>

            <div className="space-y-2">
                <h3 className="text-xs font-bold uppercase tracking-wider text-muted-foreground">{t('Request')}</h3>
                <div className="border border-border rounded-md">
                    <CliCommandRow label="Copy this command" command={buildCurl(definition, port, values)} />
                </div>
                {definition.fields.length > 0 && (
                    <ul className="space-y-1">
                        {definition.fields.map((field) => (
                            <li key={field.name} className="text-xs text-muted-foreground">
                                <code className="font-mono text-foreground">{field.name}</code>
                                {` · ${field.kind === 'file' ? 'file' : 'text'} · ${t('required')} · ${t(field.description)}`}
                            </li>
                        ))}
                    </ul>
                )}
            </div>

            <div className="space-y-2">
                <h3 className="text-xs font-bold uppercase tracking-wider text-muted-foreground">{t('Response')}</h3>
                <CodeBlock>{definition.responseExample}</CodeBlock>
            </div>

            <div className="space-y-3">
                <h3 className="text-xs font-bold uppercase tracking-wider text-muted-foreground">{t('Try it')}</h3>
                {definition.fields.map((field) => (
                    <div key={field.name} className="space-y-1">
                        <span className="block font-mono text-xs text-foreground">{field.name}</span>
                        <EndpointFieldControl
                            field={field}
                            value={values[field.name]}
                            file={file}
                            promptNames={promptNames}
                            testIdPrefix={testIdPrefix}
                            onValueChange={(value) => setValue(field.name, value)}
                            onFileChange={setFile}
                        />
                    </div>
                ))}
                <div className="flex items-center gap-3">
                    <Button
                        variant="outline"
                        size="sm"
                        onClick={handleTry}
                        disabled={blockedReason !== null || isRunning}
                        data-testid={`${testIdPrefix}-try`}
                    >
                        <Play className="w-3.5 h-3.5" />
                        {isRunning ? t('Running...') : t('Try it')}
                    </Button>
                    {blockedReason !== null && (
                        <span className="text-xs text-muted-foreground">{t(blockedReason)}</span>
                    )}
                </div>
                <div role="status" aria-live="polite" className="space-y-2">
                    {state.phase === 'done' && (
                        <>
                            <p
                                className={clsx(
                                    'font-mono text-sm font-bold',
                                    statusTone === 'success' && 'text-emerald-400',
                                    statusTone === 'warning' && 'text-yellow-300',
                                    statusTone === 'error' && 'text-red-400',
                                    statusTone === 'neutral' && 'text-foreground'
                                )}
                            >
                                {`${state.status} ${state.statusText}`}
                            </p>
                            {state.body.length > 0 && <CodeBlock>{formatResponseBody(state.body)}</CodeBlock>}
                        </>
                    )}
                    {state.phase === 'failed' && (
                        <p className="text-sm text-red-400">{t('Could not reach the local API.')}</p>
                    )}
                </div>
            </div>
        </article>
    );
};
