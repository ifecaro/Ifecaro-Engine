const CACHE_VERSION = 'v3';
const CACHE_NAME = `ifecaro-cache-${CACHE_VERSION}`;

// 靜態資源清單
const STATIC_RESOURCES = [
    '/',
    '/assets/ifecaro.js',
    '/assets/ifecaro_bg.wasm',
    '/assets/tailwind.css',
    '/assets/fonts/NotoSansTC-Regular.woff2',
    '/assets/icons/favicon.ico',
    '/assets/icons/apple-touch-icon.png',
    '/assets/icons/android-chrome-192x192.png',
    '/assets/icons/android-chrome-512x512.png',
    '/assets/manifest.json'
];

// 需要 Network First 策略的 API 路徑
const API_PATHS = [
    '/api/'
];

// 檢查資源是否需要更新
async function checkResourceUpdate(request) {
    try {
        const cachedResponse = await caches.match(request);
        if (!cachedResponse) return true;

        const networkResponse = await fetch(request);
        if (!networkResponse.ok) return false;

        // 檢查 ETag 或 Last-Modified
        const cachedETag = cachedResponse.headers.get('ETag');
        const networkETag = networkResponse.headers.get('ETag');

        if (cachedETag && networkETag && cachedETag !== networkETag) {
            return true;
        }

        // 如果沒有 ETag，則比較內容
        const networkContent = await networkResponse.text();
        const cachedContent = await cachedResponse.text();

        return networkContent !== cachedContent;
    } catch (error) {
        console.error('檢查資源更新失敗:', error);
        return false;
    }
}

// 安裝新的 service worker
self.addEventListener('install', event => {
    event.waitUntil(
        caches.open(CACHE_NAME)
            .then(cache => {
                // console.log('正在快取靜態資源...');
                return cache.addAll(STATIC_RESOURCES);
            })
            .then(() => {
                // console.log('Service Worker 安裝完成');
                return self.skipWaiting();
            })
            .catch(error => {
                console.error('Service Worker 安裝失敗:', error);
            })
    );
});

// 啟用新的 service worker
self.addEventListener('activate', event => {
    event.waitUntil(
        caches.keys()
            .then(cacheNames => {
                return Promise.all(
                    cacheNames
                        .filter(cacheName => cacheName.startsWith('ifecaro-cache-') && cacheName !== CACHE_NAME)
                        .map(cacheName => {
                            // console.log('刪除舊的快取:', cacheName);
                            return caches.delete(cacheName);
                        })
                );
            })
            .then(() => {
                // console.log('Service Worker 已啟用');
                return self.clients.claim();
            })
            .catch(error => {
                console.error('Service Worker 啟用失敗:', error);
            })
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
                    if (!response.ok) throw new Error('網路請求失敗');

                    // 如果網路請求成功，更新快取
                    const responseToCache = response.clone();
                    caches.open(CACHE_NAME)
                        .then(cache => cache.put(event.request, responseToCache))
                        .catch(error => console.error('快取更新失敗:', error));

                    return response;
                })
                .catch(async () => {
                    // 如果網路請求失敗，嘗試從快取中獲取
                    const cachedResponse = await caches.match(event.request);
                    if (cachedResponse) {
                        // console.log('使用快取的 API 回應');
                        return cachedResponse;
                    }
                    throw new Error('無法獲取資源');
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
                                    if (networkResponse.ok) {
                                        const responseToCache = networkResponse.clone();
                                        caches.open(CACHE_NAME)
                                            .then(cache => cache.put(event.request, responseToCache))
                                            .catch(error => console.error('背景更新快取失敗:', error));
                                    }
                                })
                                .catch(error => console.error('背景更新失敗:', error));
                        }
                        return cachedResponse;
                    }

                    // 如果快取中沒有，從網路獲取
                    return fetch(event.request)
                        .then(response => {
                            if (!response.ok) throw new Error('網路請求失敗');

                            const responseToCache = response.clone();
                            caches.open(CACHE_NAME)
                                .then(cache => cache.put(event.request, responseToCache))
                                .catch(error => console.error('快取更新失敗:', error));

                            return response;
                        })
                        .catch(error => {
                            console.error('獲取資源失敗:', error);
                            return new Response('無法獲取資源', { status: 503 });
                        });
                })
        );
    }
}); 