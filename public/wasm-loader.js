// 更新進度條
function updateProgress(percent, loaded, total, speed) {
    const progress = document.getElementById('progress');
    const loadingText = document.getElementById('loadingText');
    const downloadSize = document.getElementById('downloadSize');
    const downloadSpeed = document.getElementById('downloadSpeed');

    progress.style.width = `${percent}%`;
    loadingText.textContent = `${Math.round(percent)}%`;

    // 格式化檔案大小
    const formatSize = (bytes) => {
        if (bytes < 1024) return bytes + ' B';
        if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(2) + ' KB';
        return (bytes / (1024 * 1024)).toFixed(2) + ' MB';
    };

    // 格式化速度
    const formatSpeed = (bytesPerSecond) => {
        if (bytesPerSecond < 1024) return bytesPerSecond.toFixed(1) + ' B/s';
        if (bytesPerSecond < 1024 * 1024) return (bytesPerSecond / 1024).toFixed(1) + ' KB/s';
        return (bytesPerSecond / (1024 * 1024)).toFixed(1) + ' MB/s';
    };

    downloadSize.textContent = `${formatSize(loaded)} / ${formatSize(total)}`;
    downloadSpeed.textContent = formatSpeed(speed);
}

// 導出 loadWasm 作為 module function
export async function loadWasm({ wasmPath, jsPath }) {
    try {
        // 檢查 Service Worker 是否已快取 WASM
        const cache = await caches.open('ifecaro-cache-v3');
        const cachedResponse = await cache.match(wasmPath);

        if (!cachedResponse) {
            console.log('[WASM] 沒有快取，顯示進度條');
            document.getElementById('loadingContainer').style.display = 'flex';
        }

        // 下載 WASM
        console.log('[WASM] 開始下載:', wasmPath);
        const response = await fetch(wasmPath);
        if (!response.ok) throw new Error('WASM 檔案載入失敗');
        const contentLength = response.headers.get('content-length');
        const total = parseInt(contentLength, 10);
        let loaded = 0;
        let startTime = Date.now();
        const collectedChunks = [];
        const reader = response.body.getReader();

        while (true) {
            const { done, value } = await reader.read();
            if (done) break;
            collectedChunks.push(value);
            loaded += value.length;
            // 用總下載量除以總耗時計算平均速度
            const currentTime = Date.now();
            const timeElapsed = (currentTime - startTime) / 1000; // 秒
            const averageSpeed = timeElapsed > 0 ? loaded / timeElapsed : 0;
            const progress = (loaded / total) * 100;
            updateProgress(progress, loaded, total, averageSpeed);
        }
        console.log(`[WASM] 下載完成: ${loaded} / ${total}`);
        if (loaded !== total) {
            throw new Error(`[WASM] 下載長度不符: ${loaded} / ${total}`);
        }

        const blob = new Blob(collectedChunks);
        const bytes = await blob.arrayBuffer();

        // 載入 JavaScript 模組
        console.log('[WASM] 載入 JS 模組:', jsPath);
        const { default: init } = await import(jsPath);

        // 直接使用 init 函數
        const wasm = await init(bytes);

        // 載入完成後顯示主頁面
        document.getElementById('loadingContainer').style.display = 'none';
        document.getElementById('main').style.display = 'block';

        // 清除 loading 畫面的 CSS 和 JS 引用
        // 移除 CSS
        const cssLinks = document.querySelectorAll('link[rel="stylesheet"][href*="wasm-loader.css"]');
        cssLinks.forEach(link => link.parentNode.removeChild(link));
        // 移除所有 <script> 標籤
        document.querySelectorAll('script').forEach(script => script.parentNode.removeChild(script));

        // 只在沒有 __wbindgen_start 或 main 函數時才調用
        if (!wasm.__wbindgen_start && !wasm.main) {
            if (wasm.__wbindgen_start) {
                wasm.__wbindgen_start();
            } else if (wasm.main) {
                wasm.main();
            }
        }
    } catch (error) {
        console.error('[WASM] 載入失敗:', error);
        document.getElementById('loadingContainer').style.display = 'flex';
        document.querySelector('.loading-text').textContent = '載入失敗，請重新整理頁面';
    }
} 