#!/bin/bash

# 顏色定義
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 測試結果計數
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

echo -e "${BLUE}🚀 開始執行 Ifecaro 引擎完整測試套件${NC}"
echo "================================================"

# 函數：執行測試並記錄結果
run_test() {
    local test_name="$1"
    local test_command="$2"
    
    echo -e "\n${YELLOW}📋 執行 $test_name...${NC}"
    echo "指令: $test_command"
    echo "------------------------------------------------"
    
    if bash -c "$test_command"; then
        echo -e "${GREEN}✅ $test_name 通過${NC}"
        ((PASSED_TESTS++))
    else
        echo -e "${RED}❌ $test_name 失敗${NC}"
        ((FAILED_TESTS++))
    fi
    ((TOTAL_TESTS++))
}

# 1. 編譯檢查
run_test "編譯檢查" "docker compose exec app cargo check"

# 2. 基礎 UI 測試 (story_content_tests.rs)
run_test "Story Content 基礎 UI 測試" "docker compose exec app cargo test story_content_tests"

# 3. 進階功能測試 (story_content_advanced_tests.rs)
run_test "Story Content 進階功能測試" "docker compose exec app cargo test story_content_advanced_tests"

# 4. API Mock 測試 (api_tests.rs)
run_test "API Mock 測試" "docker compose exec app cargo test api_tests"

# 5. API 整合測試 (story_content_api_integration_tests.rs)
run_test "API 整合測試" "docker compose exec app cargo test integration_tests"

# 6. 所有其他測試
run_test "其他單元測試" "docker compose exec app cargo test --lib --exclude-tests"

# 7. 整合測試
run_test "外部整合測試" "docker compose exec app cargo test --test '*'"

echo -e "\n================================================"
echo -e "${BLUE}📊 測試結果總結${NC}"
echo "================================================"
echo -e "總測試項目: ${TOTAL_TESTS}"
echo -e "${GREEN}通過: ${PASSED_TESTS}${NC}"
echo -e "${RED}失敗: ${FAILED_TESTS}${NC}"

if [ $FAILED_TESTS -eq 0 ]; then
    echo -e "\n${GREEN}🎉 所有測試都通過了！${NC}"
    exit 0
else
    echo -e "\n${RED}⚠️  有 $FAILED_TESTS 個測試失敗${NC}"
    exit 1
fi 