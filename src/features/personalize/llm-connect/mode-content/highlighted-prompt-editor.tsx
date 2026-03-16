import Editor from 'react-simple-code-editor';

interface HighlightedPromptEditorProps {
    value: string;
    onChange: (value: string) => void;
    maxLength?: number;
    placeholder?: string;
    className?: string;
}

const escapeHTML = (text: string): string =>
    text.replaceAll('&', '&amp;').replaceAll('<', '&lt;').replaceAll('>', '&gt;');

const highlightSyntax = (text: string): string => {
    const regex = /(<\/?[a-zA-Z][a-zA-Z0-9_-]*(?:\s[^>]*)?>)|({{(?:TRANSCRIPT|DICTIONARY)}})/g;
    let html = '';
    let lastIndex = 0;
    let match: RegExpExecArray | null;

    while ((match = regex.exec(text)) !== null) {
        html += escapeHTML(text.slice(lastIndex, match.index));

        const [fullMatch, xmlTag, variable] = match;
        if (xmlTag) {
            html += `<span class="text-cyan-400">${escapeHTML(xmlTag)}</span>`;
        } else if (variable) {
            html += `<span class="text-amber-400">${escapeHTML(variable)}</span>`;
        }

        lastIndex = match.index + fullMatch.length;
    }

    html += escapeHTML(text.slice(lastIndex));
    return html;
};

export const HighlightedPromptEditor = ({
    value,
    onChange,
    maxLength,
    placeholder,
    className = '',
}: HighlightedPromptEditorProps) => {
    const handleChange = (newValue: string) => {
        if (maxLength !== undefined && newValue.length > maxLength) return;
        onChange(newValue);
    };

    return (
        <div
            className={`${className} overflow-y-auto bg-background/50 border border-border rounded-lg focus-within:ring-1 focus-within:ring-sky-500/50`}
        >
            <Editor
                value={value}
                onValueChange={handleChange}
                highlight={highlightSyntax}
                placeholder={placeholder}
                className="text-sm font-mono! text-foreground"
                padding={12}
                style={{ caretColor: '#d4d4d8' }}
            />
        </div>
    );
};
