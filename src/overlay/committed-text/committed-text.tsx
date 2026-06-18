import clsx from 'clsx';
import { Check } from 'lucide-react';

const LINE_HEIGHT_RATIO = 1.625;
const VERTICAL_PADDING_PX = 12;

interface CommittedTextProps {
    text: string;
    isDone: boolean;
    textWidth: number;
    fontSize: number;
    maxLines: number;
}

export const CommittedText = ({ text, isDone, textWidth, fontSize, maxLines }: CommittedTextProps) => (
    <div
        className="overflow-y-auto px-2.5 py-1.5 leading-relaxed font-sans text-white"
        style={{
            width: `${textWidth}px`,
            fontSize: `${fontSize}px`,
            maxHeight: `${Math.ceil(maxLines * fontSize * LINE_HEIGHT_RATIO) + VERTICAL_PADDING_PX}px`,
        }}
    >
        <span>{text}</span>
        {isDone && (
            <span className={clsx('inline-flex items-center gap-1 ml-1.5 align-middle text-emerald-400')}>
                <Check className="h-3 w-3 shrink-0" />
            </span>
        )}
    </div>
);
