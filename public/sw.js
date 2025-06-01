const CACHE_VERSION = 'v3';
const CACHE_NAME = `ifecaro-cache-${CACHE_VERSION}`;

// Static resource list
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

// API paths that need Network First strategy
const API_PATHS = [
    '/api/'
];

// Check if resource needs update
async function checkResourceUpdate(request) {
    try {
        const cachedResponse = await caches.match(request);
        if (!cachedResponse) return true;

        const networkResponse = await fetch(request);
        if (!networkResponse.ok) return false;

        // Check ETag or Last-Modified
        const cachedETag = cachedResponse.headers.get('ETag');
        const networkETag = networkResponse.headers.get('ETag');

        if (cachedETag && networkETag && cachedETag !== networkETag) {
            return true;
        }

        // If no ETag, compare content
        const networkContent = await networkResponse.text();
        const cachedContent = await cachedResponse.text();

        return networkContent !== cachedContent;
    } catch (error) {
        console.error('Resource update check failed:', error);
        return false;
    }
}

// Install new service worker
self.addEventListener('install', event => {
    event.waitUntil(
        caches.open(CACHE_NAME)
            .then(cache => {
                // console.log('Caching static resources...');
                return cache.addAll(STATIC_RESOURCES);
            })
            .then(() => {
                // console.log('Service Worker installation complete');
                return self.skipWaiting();
            })
            .catch(error => {
                console.error('Service Worker installation failed:', error);
            })
    );
});

// Activate new service worker
self.addEventListener('activate', event => {
    event.waitUntil(
        caches.keys()
            .then(cacheNames => {
                return Promise.all(
                    cacheNames
                        .filter(cacheName => cacheName.startsWith('ifecaro-cache-') && cacheName !== CACHE_NAME)
                        .map(cacheName => {
                            // console.log('Deleting old cache:', cacheName);
                            return caches.delete(cacheName);
                        })
                );
            })
            .then(() => {
                // console.log('Service Worker activated');
                return self.clients.claim();
            })
            .catch(error => {
                console.error('Service Worker activation failed:', error);
            })
    );
});

// Handle requests
self.addEventListener('fetch', event => {
    const url = new URL(event.request.url);

    // Check if it's an API request
    const isApiRequest = API_PATHS.some(path => url.pathname.startsWith(path));

    if (isApiRequest) {
        // API requests use Network First strategy
        event.respondWith(
            fetch(event.request)
                .then(response => {
                    if (!response.ok) throw new Error('Network request failed');

                    // If network request succeeds, update cache
                    const responseToCache = response.clone();
                    caches.open(CACHE_NAME)
                        .then(cache => cache.put(event.request, responseToCache))
                        .catch(error => console.error('Cache update failed:', error));

                    return response;
                })
                .catch(async () => {
                    // If network request fails, try to get from cache
                    const cachedResponse = await caches.match(event.request);
                    if (cachedResponse) {
                        // console.log('Using cached API response');
                        return cachedResponse;
                    }
                    throw new Error('Unable to fetch resource');
                })
        );
    } else {
        // Static resources use Cache First strategy, but check for updates
        event.respondWith(
            caches.match(event.request)
                .then(async cachedResponse => {
                    if (cachedResponse) {
                        // Check if update is needed
                        const needsUpdate = await checkResourceUpdate(event.request);
                        if (needsUpdate) {
                            // Update cache in background
                            fetch(event.request)
                                .then(networkResponse => {
                                    if (networkResponse.ok) {
                                        const responseToCache = networkResponse.clone();
                                        caches.open(CACHE_NAME)
                                            .then(cache => cache.put(event.request, responseToCache))
                                            .catch(error => console.error('Background cache update failed:', error));
                                    }
                                })
                                .catch(error => console.error('Background update failed:', error));
                        }
                        return cachedResponse;
                    }

                    // If not in cache, fetch from network
                    return fetch(event.request)
                        .then(response => {
                            if (!response.ok) throw new Error('Network request failed');

                            const responseToCache = response.clone();
                            caches.open(CACHE_NAME)
                                .then(cache => cache.put(event.request, responseToCache))
                                .catch(error => console.error('Cache update failed:', error));

                            return response;
                        })
                        .catch(error => {
                            console.error('Resource fetch failed:', error);
                            return new Response('Unable to fetch resource', { status: 503 });
                        });
                })
        );
    }
}); 