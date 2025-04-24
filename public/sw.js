const CACHE_VERSION = 'v2';
const CACHE_NAME = `ifecaro-cache-${CACHE_VERSION}`;

// 靜態資源清單
const STATIC_RESOURCES = [
    '/',
    '/assets/dioxus/Ifecaro-Engine.js',
    '/assets/dioxus/Ifecaro-Engine_bg.wasm',
    '/public/tailwind.css',
    '/public/fonts/NotoSansTC-Regular.woff2',
    '/public/img/icons/favicon.ico',
    '/public/img/icons/apple-touch-icon.png',
    '/public/img/icons/android-chrome-192x192.png',
    '/public/img/icons/android-chrome-512x512.png',
    '/public/manifest.json'
];

// 需要 Network First 策略的 API 路徑
const API_PATHS = [
    '/api/'
];

// 檢查資源是否需要更新
async function checkResourceUpdate(request) {
    const cachedResponse = await caches.match(request);

    if (!cachedResponse) return true; // 如果沒有快取，需要更新

    try {
        const networkResponse = await fetch(request);
        const networkContent = await networkResponse.text();
        const networkHash = btoa(networkContent).substring(0, 8);

        const cachedContent = await cachedResponse.text();
        const cachedHash = btoa(cachedContent).substring(0, 8);

        return networkHash !== cachedHash;
    } catch (error) {
        return false; // 如果網路請求失敗，使用快取的內容
    }
}

// 安裝新的 service worker
self.addEventListener('install', event => {
    event.waitUntil(
        caches.open(CACHE_NAME)
            .then(cache => cache.addAll(STATIC_RESOURCES))
            .then(() => self.skipWaiting()) // 立即啟用新的 service worker
    );
});

// 啟用新的 service worker
self.addEventListener('activate', event => {
    event.waitUntil(
        caches.keys().then(cacheNames => {
            return Promise.all(
                cacheNames
                    .filter(cacheName => cacheName.startsWith('ifecaro-cache-') && cacheName !== CACHE_NAME)
                    .map(cacheName => caches.delete(cacheName))
            );
        }).then(() => self.clients.claim()) // 立即控制所有頁面
    );
});

// 處理請求
self.addEventListener('fetch', event => {
    const url = new URL(event.request.url);

    // 檢查是否為 API 請求
    const isApiRequest = API_PATHS.some(path => url.pathname.startsWith(path));

    if (isApiRequest) {
        // API 請求使用 Network First 策略
        event.respondWith(
            fetch(event.request)
                .then(response => {
                    // 如果網路請求成功，更新快取
                    const responseToCache = response.clone();
                    caches.open(CACHE_NAME)
                        .then(cache => cache.put(event.request, responseToCache));
                    return response;
                })
                .catch(() => {
                    // 如果網路請求失敗，嘗試從快取中獲取
                    return caches.match(event.request);
                })
        );
    } else {
        // 靜態資源使用 Cache First 策略，但會檢查更新
        event.respondWith(
            caches.match(event.request)
                .then(async cachedResponse => {
                    if (cachedResponse) {
                        // 檢查是否需要更新
                        const needsUpdate = await checkResourceUpdate(event.request);
                        if (needsUpdate) {
                            // 在背景更新快取
                            fetch(event.request)
                                .then(networkResponse => {
                                    const responseToCache = networkResponse.clone();
                                    caches.open(CACHE_NAME)
                                        .then(cache => cache.put(event.request, responseToCache));
                                })
                                .catch(() => { }); // 忽略更新失敗
                        }
                        return cachedResponse;
                    }
                    // 如果快取中沒有，從網路獲取
                    return fetch(event.request)
                        .then(response => {
                            const responseToCache = response.clone();
                            caches.open(CACHE_NAME)
                                .then(cache => cache.put(event.request, responseToCache));
                            return response;
                        });
                })
        );
    }
}); 