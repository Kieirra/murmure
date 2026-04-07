import { useCallback, useState } from 'react';

interface TranscriptionZoneProps {
    transcriptions: string[];
}

export const TranscriptionZone = ({ transcriptions }: TranscriptionZoneProps) => {
    const [showToast, setShowToast] = useState(false);

    const handleTap = useCallback(() => {
        if (transcriptions.length === 0) return;
        if (navigator.clipboard?.writeText) {
            navigator.clipboard.writeText(transcriptions[0]).then(() => {
                setShowToast(true);
                setTimeout(() => setShowToast(false), 2000);
            });
        }
    }, [transcriptions]);

    return (
        <button
            type="button"
            className="flex-1 min-h-[60px] py-2 px-3 text-sm text-[#ccc] border-b border-[#222] overflow-y-auto relative cursor-pointer text-left w-full"
            onClick={handleTap}
        >
            {transcriptions.length === 0 ? (
                <span className="text-[#555] italic">La transcription apparaitra ici</span>
            ) : (
                <div>
                    {transcriptions.map((text, i) => (
                        <div
                            key={`${i}-${text.slice(0, 20)}`}
                            className={`py-1 border-b border-[#1a1a1a] last:border-b-0 leading-snug ${
                                i > 0 ? 'text-[#666] text-xs' : 'text-[#ccc] text-sm'
                            }`}
                        >
                            {text}
                        </div>
                    ))}
                </div>
            )}
            {showToast && (
                <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-[#22c55e] text-white px-3 py-1 rounded-md text-xs font-semibold">
                    Copie
                </div>
            )}
        </button>
    );
};
