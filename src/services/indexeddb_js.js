export function setSettingToIndexedDB(key, value) {
    const request = indexedDB.open('ifecaro', 1);
    request.onupgradeneeded = function (event) {
        const db = event.target.result;
        if (!db.objectStoreNames.contains('settings')) {
            db.createObjectStore('settings');
        }
    };
    request.onsuccess = function (event) {
        const db = event.target.result;
        const tx = db.transaction('settings', 'readwrite');
        const store = tx.objectStore('settings');
        const putReq = store.put(String(value), key);
        putReq.onsuccess = function () {
            console.log(String(value));
        };
        tx.oncomplete = function () {
            db.close();
        };
    };
    request.onerror = function (event) {
        // 可以略過錯誤
    };
}

export function getSettingsFromIndexedDB(callback) {
    const request = indexedDB.open('ifecaro', 1);
    request.onupgradeneeded = function (event) {
        const db = event.target.result;
        if (!db.objectStoreNames.contains('settings')) {
            db.createObjectStore('settings');
        }
    };
    request.onsuccess = function (event) {
        const db = event.target.result;
        const tx = db.transaction('settings', 'readonly');
        const store = tx.objectStore('settings');
        const allReq = store.getAllKeys();
        allReq.onsuccess = function () {
            const keys = allReq.result;
            const result = {};
            let count = 0;
            if (keys.length === 0) {
                callback(result);
                db.close();
                return;
            }
            keys.forEach(key => {
                const getReq = store.get(key);
                getReq.onsuccess = function () {
                    result[key] = getReq.result;
                    count++;
                    if (count === keys.length) {
                        callback(result);
                        db.close();
                    }
                };
            });
        };
        allReq.onerror = function () {
            callback({});
            db.close();
        };
    };
    request.onerror = function (event) {
        callback({});
    };
} 