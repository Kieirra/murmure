import { useRef, useCallback, useEffect, type ChangeEvent } from 'react';

interface HighlightedPromptEditorProps {
    value: string;
    onChange: (value: string) => void;
    maxLength?: number;
    placeholder?: string;
    className?: string;
}

/**
 * Applies syntax highlighting to the prompt text.
 * - XML tags: highlighted in cyan
 * - Variables {{TRANSCRIPT}} and {{DICTIONARY}}: highlighted in amber
 * This is purely visual and does not modify the underlying text.
 */
const highlightSyntax = (text: string): React.ReactNode[] => {
    // Regex to match XML tags or variables
    const regex =
        /(<\/?[a-zA-Z][a-zA-Z0-9_-]*(?:\s[^>]*)?>)|({{(?:TRANSCRIPT|DICTIONARY)}})/g;
    const parts: React.ReactNode[] = [];
    let lastIndex = 0;
    let match: RegExpExecArray | null;

    while ((match = regex.exec(text)) !== null) {
        // Add text before the match
        if (match.index > lastIndex) {
            parts.push(text.slice(lastIndex, match.index));
        }

        const [fullMatch, xmlTag, variable] = match;

        if (xmlTag) {
            // XML tag - cyan color
            parts.push(
                <span key={match.index} className="text-cyan-400">
                    {xmlTag}
                </span>
            );
        } else if (variable) {
            // Variable - amber color
            parts.push(
                <span key={match.index} className="text-amber-400">
                    {variable}
                </span>
            );
        }

        lastIndex = match.index + fullMatch.length;
    }

    // Add remaining text after last match
    if (lastIndex < text.length) {
        parts.push(text.slice(lastIndex));
    }

    // Keep empty content visible for proper sizing
    if (parts.length === 0) {
        parts.push('');
    }

    return parts;
};

export const HighlightedPromptEditor = ({
    value,
    onChange,
    maxLength = 4000,
    placeholder,
    className = '',
}: HighlightedPromptEditorProps) => {
    const textareaRef = useRef<HTMLTextAreaElement>(null);
    const highlightRef = useRef<HTMLDivElement>(null);

    // Sync scroll between textarea and highlight layer
    const handleScroll = useCallback(() => {
        if (textareaRef.current && highlightRef.current) {
            highlightRef.current.scrollTop = textareaRef.current.scrollTop;
            highlightRef.current.scrollLeft = textareaRef.current.scrollLeft;
        }
    }, []);

    // Attach scroll listener
    useEffect(() => {
        const textarea = textareaRef.current;
        if (textarea) {
            textarea.addEventListener('scroll', handleScroll);
            return () => textarea.removeEventListener('scroll', handleScroll);
        }
    }, [handleScroll]);

    const handleChange = (e: ChangeEvent<HTMLTextAreaElement>) => {
        const newValue = e.target.value.slice(0, maxLength);
        onChange(newValue);
    };

    return (
        <div className={`relative ${className}`}>
            {/* Highlight layer (background) */}
            <div
                ref={highlightRef}
                aria-hidden="true"
                className="absolute inset-0 px-4 py-3 bg-zinc-900/50 border border-zinc-800 rounded-lg text-sm font-mono whitespace-pre-wrap break-words overflow-hidden pointer-events-none text-zinc-300"
            >
                {highlightSyntax(value)}
            </div>

            {/* Editable textarea (foreground, transparent text) */}
            <textarea
                ref={textareaRef}
                value={value}
                onChange={handleChange}
                maxLength={maxLength}
                placeholder={placeholder}
                className="w-full h-full px-4 py-3 bg-transparent border border-transparent rounded-lg text-sm font-mono resize-y relative z-10 focus:outline-none focus:ring-1 focus:ring-sky-500/50 text-transparent caret-zinc-300 selection:bg-sky-500/30"
                style={{ caretColor: '#d4d4d8', resize: 'none' }}
            />
        </div>
    );
};
