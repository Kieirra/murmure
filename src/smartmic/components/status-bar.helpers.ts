export const clearCachesAndReload = () => {
    const reload = () => {
        location.reload();
    };
    if ('serviceWorker' in navigator && navigator.serviceWorker.controller !== null) {
        caches
            .keys()
            .then((keys) => Promise.all(keys.map((k) => caches.delete(k))))
            .then(reload)
            .catch(reload);
    } else {
        reload();
    }
};
