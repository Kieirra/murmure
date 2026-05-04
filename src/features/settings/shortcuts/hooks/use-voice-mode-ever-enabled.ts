import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';

// Gates the Voice Mode shortcut UI: stays hidden on fresh installs until
// the user has activated Voice Mode at least once.
export const useVoiceModeEverEnabled = () => {
    const [everEnabled, setEverEnabled] = useState(false);

    useEffect(() => {
        invoke<boolean>('get_voice_mode_ever_enabled')
            .then((value) => setEverEnabled(value))
            .catch(() => setEverEnabled(false));
    }, []);

    return everEnabled;
};
