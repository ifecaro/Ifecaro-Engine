export function setSettingToIndexedDB(key, value) {
    const request = indexedDB.open('ifecaro', 2);
    request.onupgradeneeded = function (event) {
        const db = event.target.result;
        if (!db.objectStoreNames.contains('settings')) {
            db.createObjectStore('settings');
        }
        if (!db.objectStoreNames.contains('choices')) {
            db.createObjectStore('choices');
        }
    };
    request.onsuccess = function (event) {
        const db = event.target.result;
        const tx = db.transaction('settings', 'readwrite');
        const store = tx.objectStore('settings');
        const putReq = store.put(String(value), key);
        putReq.onsuccess = function () {
        };
        putReq.onerror = function (e) {
        };
        tx.oncomplete = function () {
            db.close();
        };
        tx.onerror = function (e) {
        };
    };
    request.onerror = function (event) {
    };
}

export function getSettingsFromIndexedDB(callback) {
    const request = indexedDB.open('ifecaro', 2);
    request.onupgradeneeded = function (event) {
        const db = event.target.result;
        if (!db.objectStoreNames.contains('settings')) {
            db.createObjectStore('settings');
        }
        if (!db.objectStoreNames.contains('choices')) {
            db.createObjectStore('choices');
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
                getReq.onerror = function (e) {
                    callback({});
                    db.close();
                };
            });
        };
        allReq.onerror = function (e) {
            callback({});
            db.close();
        };
        tx.oncomplete = function () {
        };
        tx.onerror = function (e) {
        };
    };
    request.onerror = function (event) {
        callback({});
    };
}

// 新版：儲存段落選擇紀錄，所有章節都存在 'choices' object store，key 為 chapterId
export function setChoiceToIndexedDB(chapterId, paragraphId) {
    const request = indexedDB.open('ifecaro', 2);
    request.onupgradeneeded = function (event) {
        const db = event.target.result;
        if (!db.objectStoreNames.contains('settings')) {
            db.createObjectStore('settings');
        }
        if (!db.objectStoreNames.contains('choices')) {
            db.createObjectStore('choices');
        }
    };
    request.onsuccess = function (event) {
        const db = event.target.result;
        const tx = db.transaction('choices', 'readwrite');
        const store = tx.objectStore('choices');
        const getReq = store.get(chapterId);
        getReq.onsuccess = function () {
            let choices = getReq.result || [];
            if (!Array.isArray(choices)) choices = [];
            if (!choices.includes(paragraphId)) {
                choices.push(paragraphId);
            }
            const putReq = store.put(choices, chapterId);
            putReq.onsuccess = function () {
            };
            putReq.onerror = function (e) {
            };
        };
        getReq.onerror = function (e) {
        };
        tx.oncomplete = function () {
            db.close();
        };
        tx.onerror = function (e) {
        };
    };
    request.onerror = function (event) {
    };
}

// 取得段落選擇紀錄，所有章節都存在 'choices' object store，key 為 chapterId
export function getChoiceFromIndexedDB(chapterId, callback) {
    const request = indexedDB.open('ifecaro', 2);
    request.onupgradeneeded = function (event) {
        const db = event.target.result;
        if (!db.objectStoreNames.contains('settings')) {
            db.createObjectStore('settings');
        }
        if (!db.objectStoreNames.contains('choices')) {
            db.createObjectStore('choices');
        }
    };
    request.onsuccess = function (event) {
        const db = event.target.result;
        const tx = db.transaction('choices', 'readonly');
        const store = tx.objectStore('choices');
        const getReq = store.get(chapterId);
        getReq.onsuccess = function () {
            let choices = getReq.result || [];
            if (!Array.isArray(choices)) choices = [];
            callback(choices);
            db.close();
        };
        getReq.onerror = function (e) {
            callback([]);
            db.close();
        };
        tx.oncomplete = function () { };
        tx.onerror = function (e) { };
    };
    request.onerror = function (event) {
        callback([]);
    };
} 