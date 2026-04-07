interface StatusBarProps {
    connected: boolean;
    statusText: string;
    pcName: string;
}

export const StatusBar = ({ connected, statusText, pcName }: StatusBarProps) => {
    return (
        <div className="h-8 flex items-center justify-between px-3 text-xs text-[#888] shrink-0 border-b border-[#222]">
            <div className="flex items-center">
                <div
                    className="w-2 h-2 rounded-full mr-2 transition-colors duration-150"
                    style={{ background: connected ? '#22c55e' : '#555' }}
                />
                <span>{statusText}</span>
            </div>
            <span>{pcName}</span>
        </div>
    );
};
