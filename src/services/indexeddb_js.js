export function setSettingToIndexedDB(key, value) {
    const request = indexedDB.open('ifecaro', 3);
    request.onupgradeneeded = function (event) {
        const db = event.target.result;
        if (!db.objectStoreNames.contains('settings')) {
            db.createObjectStore('settings');
        }
        if (!db.objectStoreNames.contains('choices')) {
            db.createObjectStore('choices');
        }
        if (!db.objectStoreNames.contains('disabled_choices')) {
            db.createObjectStore('disabled_choices');
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
    const request = indexedDB.open('ifecaro', 3);
    request.onupgradeneeded = function (event) {
        const db = event.target.result;
        if (!db.objectStoreNames.contains('settings')) {
            db.createObjectStore('settings');
        }
        if (!db.objectStoreNames.contains('choices')) {
            db.createObjectStore('choices');
        }
        if (!db.objectStoreNames.contains('disabled_choices')) {
            db.createObjectStore('disabled_choices');
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
    const request = indexedDB.open('ifecaro', 3);
    request.onupgradeneeded = function (event) {
        const db = event.target.result;
        if (!db.objectStoreNames.contains('settings')) {
            db.createObjectStore('settings');
        }
        if (!db.objectStoreNames.contains('choices')) {
            db.createObjectStore('choices');
        }
        if (!db.objectStoreNames.contains('disabled_choices')) {
            db.createObjectStore('disabled_choices');
        }
    };
    request.onsuccess = function (event) {
        const db = event.target.result;
        const tx = db.transaction('choices', 'readwrite');
        const store = tx.objectStore('choices');
        // 先讀出原本的陣列
        const getReq = store.get(chapterId);
        getReq.onsuccess = function () {
            let arr = getReq.result;
            if (!Array.isArray(arr)) arr = [];
            // 單選只保留一個，若要多選可改 push
            arr = [paragraphId];
            const putReq = store.put(arr, chapterId);
            putReq.onsuccess = function () { };
            putReq.onerror = function (e) { };
        };
        getReq.onerror = function (e) {
            // 若讀取失敗直接存新陣列
            const putReq = store.put([paragraphId], chapterId);
            putReq.onsuccess = function () { };
            putReq.onerror = function (e) { };
        };
        tx.oncomplete = function () {
            db.close();
        };
        tx.onerror = function (e) { };
    };
    request.onerror = function (event) { };
}

// 取得段落選擇紀錄，所有章節都存在 'choices' object store，key 為 chapterId
export function getChoiceFromIndexedDB(chapterId, callback) {
    const request = indexedDB.open('ifecaro', 3);
    request.onupgradeneeded = function (event) {
        const db = event.target.result;
        if (!db.objectStoreNames.contains('settings')) {
            db.createObjectStore('settings');
        }
        if (!db.objectStoreNames.contains('choices')) {
            db.createObjectStore('choices');
        }
        if (!db.objectStoreNames.contains('disabled_choices')) {
            db.createObjectStore('disabled_choices');
        }
    };
    request.onsuccess = function (event) {
        const db = event.target.result;
        const tx = db.transaction('choices', 'readonly');
        const store = tx.objectStore('choices');
        const getReq = store.get(chapterId);
        getReq.onsuccess = function () {
            let arr = getReq.result;
            if (!Array.isArray(arr)) arr = [];
            callback(arr);
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

// 儲存停用選項狀態，key 為 "paragraphId:choiceIndex"
export function setDisabledChoiceToIndexedDB(paragraphId, choiceIndex) {
    const request = indexedDB.open('ifecaro', 3);
    request.onupgradeneeded = function (event) {
        const db = event.target.result;
        if (!db.objectStoreNames.contains('settings')) {
            db.createObjectStore('settings');
        }
        if (!db.objectStoreNames.contains('choices')) {
            db.createObjectStore('choices');
        }
        if (!db.objectStoreNames.contains('disabled_choices')) {
            db.createObjectStore('disabled_choices');
        }
    };
    request.onsuccess = function (event) {
        const db = event.target.result;
        const tx = db.transaction('disabled_choices', 'readwrite');
        const store = tx.objectStore('disabled_choices');
        const key = `${paragraphId}:${choiceIndex}`;
        const putReq = store.put(true, key);
        putReq.onsuccess = function () { };
        putReq.onerror = function (e) { };
        tx.oncomplete = function () {
            db.close();
        };
        tx.onerror = function (e) { };
    };
    request.onerror = function (event) { };
}

// 取得停用選項狀態，返回指定段落的所有停用選項陣列
export function getDisabledChoicesFromIndexedDB(paragraphId, callback) {
    const request = indexedDB.open('ifecaro', 3);
    request.onupgradeneeded = function (event) {
        const db = event.target.result;
        if (!db.objectStoreNames.contains('settings')) {
            db.createObjectStore('settings');
        }
        if (!db.objectStoreNames.contains('choices')) {
            db.createObjectStore('choices');
        }
        if (!db.objectStoreNames.contains('disabled_choices')) {
            db.createObjectStore('disabled_choices');
        }
    };
    request.onsuccess = function (event) {
        const db = event.target.result;
        const tx = db.transaction('disabled_choices', 'readonly');
        const store = tx.objectStore('disabled_choices');
        const getAllReq = store.getAllKeys();
        getAllReq.onsuccess = function () {
            const keys = getAllReq.result;
            const disabledChoices = [];

            // 篩選出屬於指定段落的停用選項
            keys.forEach(key => {
                if (key.startsWith(`${paragraphId}:`)) {
                    const choiceIndex = parseInt(key.split(':')[1]);
                    if (!isNaN(choiceIndex)) {
                        disabledChoices.push(choiceIndex);
                    }
                }
            });

            callback(disabledChoices);
            db.close();
        };
        getAllReq.onerror = function (e) {
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

// 清除指定段落的所有停用選項（當切換到新段落時使用）
export function clearDisabledChoicesForParagraph(paragraphId) {
    const request = indexedDB.open('ifecaro', 3);
    request.onupgradeneeded = function (event) {
        const db = event.target.result;
        if (!db.objectStoreNames.contains('settings')) {
            db.createObjectStore('settings');
        }
        if (!db.objectStoreNames.contains('choices')) {
            db.createObjectStore('choices');
        }
        if (!db.objectStoreNames.contains('disabled_choices')) {
            db.createObjectStore('disabled_choices');
        }
    };
    request.onsuccess = function (event) {
        const db = event.target.result;
        const tx = db.transaction('disabled_choices', 'readwrite');
        const store = tx.objectStore('disabled_choices');
        const getAllReq = store.getAllKeys();
        getAllReq.onsuccess = function () {
            const keys = getAllReq.result;
            let deleteCount = 0;
            const keysToDelete = keys.filter(key => key.startsWith(`${paragraphId}:`));

            if (keysToDelete.length === 0) {
                db.close();
                return;
            }

            keysToDelete.forEach(key => {
                const deleteReq = store.delete(key);
                deleteReq.onsuccess = function () {
                    deleteCount++;
                    if (deleteCount === keysToDelete.length) {
                        db.close();
                    }
                };
                deleteReq.onerror = function (e) {
                    deleteCount++;
                    if (deleteCount === keysToDelete.length) {
                        db.close();
                    }
                };
            });
        };
        getAllReq.onerror = function (e) {
            db.close();
        };
        tx.oncomplete = function () { };
        tx.onerror = function (e) { };
    };
    request.onerror = function (event) { };
} 