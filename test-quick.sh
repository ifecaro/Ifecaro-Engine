#!/bin/bash

echo "🚀 快速測試 - 僅核心功能"
echo "=========================="

# 編譯檢查
echo "🔧 編譯檢查..."
docker compose exec app cargo check || exit 1

# 故事內容相關測試
echo "📖 Story Content 測試..."
docker compose exec app cargo test story_content || exit 1

# API 測試
echo "🌐 API 測試..."
docker compose exec app cargo test api || exit 1

echo ""
echo "✅ 快速測試完成！" 