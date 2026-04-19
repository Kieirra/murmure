import { t } from '../i18n';

interface DeviceConflictOverlayProps {
    deviceName: string | null;
    onForceConnect: () => void;
    onDismiss: () => void;
}

export const DeviceConflictOverlay = ({ deviceName, onForceConnect, onDismiss }: DeviceConflictOverlayProps) => {
    if (deviceName === null) return null;

    return (
        <div className="fixed inset-0 bg-black/85 flex items-center justify-center p-5 z-50">
            <div className="bg-[#1a1a1a] border border-[#dc2626] rounded-xl p-5 max-w-[320px] text-center">
                <h3 className="text-[#fca5a5] text-base mb-2">{t('conflict.title')}</h3>
                <p className="text-[#999] text-sm leading-relaxed">
                    {t('conflict.message', { name: deviceName })}
                </p>
                <div className="mt-4 flex gap-3 justify-center">
                    <button
                        className="px-4 py-2 bg-[#333] border-none rounded-lg text-[#e5e5e5] text-sm cursor-pointer"
                        onClick={onDismiss}
                    >
                        {t('conflict.cancel')}
                    </button>
                    <button
                        className="px-4 py-2 bg-[#dc2626] border-none rounded-lg text-white text-sm cursor-pointer"
                        onClick={onForceConnect}
                    >
                        {t('conflict.takeOver')}
                    </button>
                </div>
            </div>
        </div>
    );
};
