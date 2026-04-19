import { type Dispatch, useEffect } from 'react';
import type { ServerMessage } from '../smartmic.types';
import type { SmartMicAction } from '../store/smartmic-reducer';
import type { I18n } from '../i18n/use-i18n';

interface UseServerMessageDispatcherParams {
    lastMessage: ServerMessage | null;
    dispatch: Dispatch<SmartMicAction>;
    t: I18n['t'];
}

// Intercepts `error` and `force_disconnect` to produce a localized overlay error,
// then delegates every other message to the reducer.
export const useServerMessageDispatcher = ({
    lastMessage,
    dispatch,
    t,
}: UseServerMessageDispatcherParams): void => {
    useEffect(() => {
        if (lastMessage === null) return;
        if (lastMessage.type === 'force_disconnect') {
            dispatch({
                type: 'set_error',
                error: { title: t('errors.disconnected'), message: t('errors.forceDisconnect') },
            });
            return;
        }
        if (lastMessage.type === 'error') {
            dispatch({
                type: 'set_error',
                error: { title: t('errors.title'), message: lastMessage.message || t('errors.micGeneric') },
            });
            return;
        }
        dispatch({ type: 'server_message', message: lastMessage });
    }, [lastMessage, t, dispatch]);
};
