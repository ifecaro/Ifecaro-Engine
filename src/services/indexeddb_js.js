const DB_NAME = 'ifecaro';
const DB_VERSION = 6;
const STORES = ['settings', 'choices', 'disabled_choices', 'random_choices', 'choice_impacts', 'character_states'];

function openDB() {
    return new Promise((resolve, reject) => {
        const request = indexedDB.open(DB_NAME, DB_VERSION);
        request.onupgradeneeded = function (event) {
            const db = event.target.result;
            STORES.forEach(storeName => {
                if (!db.objectStoreNames.contains(storeName)) {
                    db.createObjectStore(storeName);
                }
            });
        };
        request.onsuccess = function (event) {
            resolve(event.target.result);
        };
        request.onerror = function (event) {
            console.error("Database error: ", event.target.error);
            reject(event.target.error);
        };
    });
}

export function setSettingToIndexedDB(key, value) {
    const request = indexedDB.open('ifecaro', DB_VERSION);
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
        if (!db.objectStoreNames.contains('random_choices')) {
            db.createObjectStore('random_choices');
        }
        if (!db.objectStoreNames.contains('choice_impacts')) {
            db.createObjectStore('choice_impacts');
        }
        if (!db.objectStoreNames.contains('character_states')) {
            db.createObjectStore('character_states');
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
    const request = indexedDB.open('ifecaro', DB_VERSION);
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
        if (!db.objectStoreNames.contains('random_choices')) {
            db.createObjectStore('random_choices');
        }
        if (!db.objectStoreNames.contains('choice_impacts')) {
            db.createObjectStore('choice_impacts');
        }
        if (!db.objectStoreNames.contains('character_states')) {
            db.createObjectStore('character_states');
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
export async function setChoiceToIndexedDB(chapterId, paragraphId) {
    const db = await openDB();
    return new Promise((resolve, reject) => {
        const tx = db.transaction('choices', 'readwrite');
        const store = tx.objectStore('choices');
        const getReq = store.get(chapterId);

        getReq.onsuccess = function () {
            let arr = getReq.result;
            if (!Array.isArray(arr)) arr = [];
            if (!arr.includes(paragraphId)) {
                arr.push(paragraphId);
            }
            const putReq = store.put(arr, chapterId);
            putReq.onsuccess = () => { };
            putReq.onerror = (e) => { };
        };
        getReq.onerror = function (e) {
            const putReq = store.put([paragraphId], chapterId);
            putReq.onsuccess = () => { };
            putReq.onerror = (e) => { };
        };
        tx.oncomplete = function () {
            db.close();
            resolve();
        };
        tx.onerror = function (e) {
            console.error("Transaction error in setChoiceToIndexedDB: ", e.target.error);
            db.close();
            reject(e.target.error);
        };
    });
}

// 取得段落選擇紀錄，所有章節都存在 'choices' object store，key 為 chapterId
export async function getChoiceFromIndexedDB(chapterId) {
    const db = await openDB();
    return new Promise((resolve, reject) => {
        const tx = db.transaction('choices', 'readonly');
        const store = tx.objectStore('choices');
        const getReq = store.get(chapterId);
        getReq.onsuccess = function () {
            let arr = getReq.result;
            if (!Array.isArray(arr)) arr = [];
            resolve(arr);
        };
        getReq.onerror = function (e) {
            console.error("Get request error in getChoiceFromIndexedDB: ", e.target.error);
            reject(e.target.error);
        };
        tx.oncomplete = function () {
            db.close();
        };
        tx.onerror = function (e) {
            console.error("Transaction error in getChoiceFromIndexedDB: ", e.target.error);
            db.close();
            reject(e.target.error);
        };
    });
}

// 儲存停用選項狀態，key 為 paragraphId，value 為 choiceIndex 陣列
export async function setDisabledChoiceToIndexedDB(paragraphId, choiceIndex) {
    const db = await openDB();
    return new Promise((resolve, reject) => {
        const tx = db.transaction('disabled_choices', 'readwrite');
        const store = tx.objectStore('disabled_choices');
        const getReq = store.get(paragraphId);

        getReq.onsuccess = function () {
            let arr = getReq.result;
            if (!Array.isArray(arr)) arr = [];
            if (!arr.includes(choiceIndex)) {
                arr.push(choiceIndex);
            }
            const putReq = store.put(arr, paragraphId);
            putReq.onsuccess = () => { };
            putReq.onerror = (e) => { };
        };
        getReq.onerror = function (e) {
            const putReq = store.put([choiceIndex], paragraphId);
            putReq.onsuccess = () => { };
            putReq.onerror = (e) => { };
        };

        tx.oncomplete = function () {
            db.close();
            resolve();
        };
        tx.onerror = function (e) {
            console.error("Transaction error in setDisabledChoiceToIndexedDB: ", e.target.error);
            db.close();
            reject(e.target.error);
        };
    });
}

// 取得停用選項狀態，返回指定段落的所有停用選項陣列
export async function getDisabledChoicesFromIndexedDB(paragraphId) {
    const db = await openDB();
    return new Promise((resolve, reject) => {
        const tx = db.transaction('disabled_choices', 'readonly');
        const store = tx.objectStore('disabled_choices');
        const getReq = store.get(paragraphId);

        getReq.onsuccess = function () {
            let arr = getReq.result;
            if (!Array.isArray(arr)) arr = [];
            resolve(arr);
        };
        getReq.onerror = function (e) {
            console.error("Get request error in getDisabledChoicesFromIndexedDB: ", e.target.error);
            reject(e.target.error);
        };

        tx.oncomplete = function () {
            db.close();
        };
        tx.onerror = function (e) {
            console.error("Transaction error in getDisabledChoicesFromIndexedDB: ", e.target.error);
            db.close();
            reject(e.target.error);
        };
    });
}

// 儲存具有影響的選項影響（人格/關係）
export async function setChoiceImpactsToIndexedDB(paragraphId, choiceIndex, impactsJson) {
    const db = await openDB();
    return new Promise((resolve, reject) => {
        const tx = db.transaction('choice_impacts', 'readwrite');
        const store = tx.objectStore('choice_impacts');
        const key = `${paragraphId}:${choiceIndex}`;
        const putReq = store.put(impactsJson, key);

        putReq.onsuccess = () => { };
        putReq.onerror = (e) => {
            console.error("Put request error in setChoiceImpactsToIndexedDB: ", e.target.error);
        };

        tx.oncomplete = function () {
            db.close();
            resolve();
        };
        tx.onerror = function (e) {
            console.error("Transaction error in setChoiceImpactsToIndexedDB: ", e.target.error);
            db.close();
            reject(e.target.error);
        };
    });
}

// 取得指定段落選項影響
export async function getChoiceImpactsFromIndexedDB(paragraphId, choiceIndex) {
    const db = await openDB();
    return new Promise((resolve, reject) => {
        const tx = db.transaction('choice_impacts', 'readonly');
        const store = tx.objectStore('choice_impacts');
        const key = `${paragraphId}:${choiceIndex}`;
        const getReq = store.get(key);

        getReq.onsuccess = function () {
            resolve(getReq.result || null);
        };
        getReq.onerror = function (e) {
            console.error("Get request error in getChoiceImpactsFromIndexedDB: ", e.target.error);
            reject(e.target.error);
        };

        tx.oncomplete = function () {
            db.close();
        };
        tx.onerror = function (e) {
            console.error("Transaction error in getChoiceImpactsFromIndexedDB: ", e.target.error);
            db.close();
            reject(e.target.error);
        };
    });
}

// 儲存累積後的最新人物屬性/關係狀態
export async function setLatestCharacterStateToIndexedDB(stateJson) {
    const db = await openDB();
    return new Promise((resolve, reject) => {
        const tx = db.transaction('character_states', 'readwrite');
        const store = tx.objectStore('character_states');
        const putReq = store.put(stateJson, 'latest');

        putReq.onsuccess = () => { };
        putReq.onerror = (e) => {
            console.error("Put request error in setLatestCharacterStateToIndexedDB: ", e.target.error);
        };

        tx.oncomplete = function () {
            db.close();
            resolve();
        };
        tx.onerror = function (e) {
            console.error("Transaction error in setLatestCharacterStateToIndexedDB: ", e.target.error);
            db.close();
            reject(e.target.error);
        };
    });
}

// 取得最新人物屬性/關係狀態
export async function getLatestCharacterStateFromIndexedDB() {
    const db = await openDB();
    return new Promise((resolve, reject) => {
        const tx = db.transaction('character_states', 'readonly');
        const store = tx.objectStore('character_states');
        const getReq = store.get('latest');

        getReq.onsuccess = function () {
            resolve(getReq.result || null);
        };
        getReq.onerror = function (e) {
            console.error("Get request error in getLatestCharacterStateFromIndexedDB: ", e.target.error);
            reject(e.target.error);
        };

        tx.oncomplete = function () {
            db.close();
        };
        tx.onerror = function (e) {
            console.error("Transaction error in getLatestCharacterStateFromIndexedDB: ", e.target.error);
            db.close();
            reject(e.target.error);
        };
    });
}

// 清除指定段落的停用選項
export async function clearDisabledChoicesForParagraph(paragraphId) {
    const db = await openDB();
    return new Promise((resolve, reject) => {
        const tx = db.transaction('disabled_choices', 'readwrite');
        const store = tx.objectStore('disabled_choices');
        const deleteReq = store.delete(paragraphId);

        deleteReq.onsuccess = function () { };
        deleteReq.onerror = function (e) { };

        tx.oncomplete = function () {
            db.close();
            resolve();
        };
        tx.onerror = function (e) {
            console.error("Transaction error in clearDisabledChoicesForParagraph: ", e.target.error);
            db.close();
            reject(e.target.error);
        };
    });
}

// 儲存隨機選擇結果
export function setRandomChoiceToIndexedDB(paragraphId, choiceIndex, originalChoices, selectedChoice) {
    const request = indexedDB.open('ifecaro', DB_VERSION);
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
        if (!db.objectStoreNames.contains('random_choices')) {
            db.createObjectStore('random_choices');
        }
    };
    request.onsuccess = function (event) {
        const db = event.target.result;
        const tx = db.transaction('random_choices', 'readwrite');
        const store = tx.objectStore('random_choices');
        const key = `${paragraphId}:${choiceIndex}`;
        const value = {
            originalChoices: originalChoices,
            selectedChoice: selectedChoice
        };
        const putReq = store.put(value, key);
        putReq.onsuccess = function () { };
        putReq.onerror = function (e) { };
        tx.oncomplete = function () {
            db.close();
        };
        tx.onerror = function (e) { };
    };
    request.onerror = function (event) { };
}

// 取得隨機選擇結果
export function getRandomChoiceFromIndexedDB(paragraphId, choiceIndex, callback) {
    const request = indexedDB.open('ifecaro', DB_VERSION);
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
        if (!db.objectStoreNames.contains('random_choices')) {
            db.createObjectStore('random_choices');
        }
    };
    request.onsuccess = function (event) {
        const db = event.target.result;
        const tx = db.transaction('random_choices', 'readonly');
        const store = tx.objectStore('random_choices');
        const key = `${paragraphId}:${choiceIndex}`;
        const getReq = store.get(key);
        getReq.onsuccess = function () {
            const result = getReq.result;
            callback(result);
            db.close();
        };
        getReq.onerror = function (e) {
            callback(null);
            db.close();
        };
        tx.oncomplete = function () { };
        tx.onerror = function (e) { };
    };
    request.onerror = function (event) {
        callback(null);
    };
}

// 儲存完整路徑
export async function setChoicesToIndexedDB(chapterId, idsArray) {
    const db = await openDB();
    return new Promise((resolve, reject) => {
        const tx = db.transaction('choices', 'readwrite');
        const store = tx.objectStore('choices');
        const putReq = store.put(idsArray, chapterId);
        putReq.onsuccess = function () { };
        putReq.onerror = function (e) { };
        tx.oncomplete = function () {
            db.close();
            resolve();
        };
        tx.onerror = function (e) {
            console.error("Transaction error in setChoicesToIndexedDB: ", e.target.error);
            db.close();
            reject(e.target.error);
        };
    });
}

// 清除所有選擇和隨機選擇
export async function clearChoicesAndRandomChoices() {
    const db = await openDB();
    return new Promise((resolve, reject) => {
        const tx = db.transaction(['choices', 'random_choices'], 'readwrite');
        const choicesStore = tx.objectStore('choices');
        const randomChoicesStore = tx.objectStore('random_choices');
        choicesStore.clear();
        randomChoicesStore.clear();
        tx.oncomplete = function () {
            db.close();
            resolve();
        };
        tx.onerror = function (e) {
            console.error("Transaction error in clearChoicesAndRandomChoices: ", e.target.error);
            db.close();
            reject(e.target.error);
        };
    });
}

// 清除所有停用選項
export async function clearAllDisabledChoices() {
    const db = await openDB();
    return new Promise((resolve, reject) => {
        const tx = db.transaction('disabled_choices', 'readwrite');
        const store = tx.objectStore('disabled_choices');
        store.clear();
        tx.oncomplete = function () {
            db.close();
            resolve();
        };
        tx.onerror = function (e) {
            console.error("Transaction error in clearAllDisabledChoices: ", e.target.error);
            db.close();
            reject(e.target.error);
        };
    });
}

// 確保函數被暴露給 window 物件
window.clearAllDisabledChoices = clearAllDisabledChoices;