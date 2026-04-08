interface ErrorOverlayProps {
    visible: boolean;
    title: string;
    message: string;
    onDismiss: () => void;
}

export const ErrorOverlay = ({ visible, title, message, onDismiss }: ErrorOverlayProps) => {
    if (!visible) return null;

    return (
        <div className="fixed inset-0 bg-black/85 flex items-center justify-center p-5 z-50">
            <div className="bg-[#1a1a1a] border border-[#dc2626] rounded-xl p-5 max-w-[320px] text-center">
                <h3 className="text-[#fca5a5] text-base mb-2">{title}</h3>
                <p className="text-[#999] text-sm leading-relaxed">{message}</p>
                <button
                    className="mt-4 px-6 py-2 bg-[#333] border-none rounded-lg text-[#e5e5e5] text-sm cursor-pointer"
                    onClick={onDismiss}
                >
                    OK
                </button>
            </div>
        </div>
    );
};
