# Dashboard Component Test Documentation

這是 Ifecaro Engine Dashboard 組件的完整測試套件說明文件。

## 概覽

Dashboard 組件是 Ifecaro Engine 的核心管理介面，負責段落內容的創建、編輯和管理。本測試套件涵蓋了所有主要功能和邊界情況。

## 測試檔案結構

```
tests/
├── dashboard_tests.rs              # 主要測試檔案
├── dashboard_interaction_tests.rs  # 互動流程測試
├── dashboard_benchmark_tests.rs    # 效能和壓力測試
├── README_dashboard_tests.md       # 本說明文件
└── scripts/
    └── run_dashboard_tests.sh      # 測試執行腳本
```

## 測試類別

### 1. 單元測試 (Unit Tests)
- **檔案位置**: `tests/dashboard_tests.rs::unit_tests`
- **測試範圍**:
  - Dashboard Props 創建
  - 語言狀態初始化
  - 章節狀態初始化
  - 段落狀態初始化
  - 段落文本多語言支援
  - 段落選項結構驗證
  - 可用語言列表驗證

### 2. 組件測試 (Component Tests)
- **檔案位置**: `tests/dashboard_tests.rs::component_tests`
- **測試範圍**:
  - Dashboard 組件渲染
  - 不同語言 props 處理
  - VirtualDom 建構驗證

### 3. 整合測試 (Integration Tests)
- **檔案位置**: `tests/dashboard_tests.rs::integration_tests`
- **測試範圍**:
  - 狀態管理流程
  - 段落內容翻譯
  - 選項本地化
  - 章節標題本地化
  - Context 間的資料流

### 4. 表單驗證測試 (Form Validation Tests)
- **檔案位置**: `tests/dashboard_tests.rs::form_validation_tests`
- **測試範圍**:
  - 段落內容驗證
  - 章節選擇驗證
  - 選項驗證邏輯
  - 空值處理
  - 空白字元處理

### 5. 錯誤處理測試 (Error Handling Tests)
- **檔案位置**: `tests/dashboard_tests.rs::error_handling_tests`
- **測試範圍**:
  - 缺失翻譯處理
  - 無效段落 ID 處理
  - 無效章節 ID 處理
  - 回退機制驗證

### 6. 互動測試 (Interaction Tests)
- **檔案位置**: `tests/dashboard_interaction_tests.rs::interaction_tests`
- **測試範圍**:
  - 語言切換工作流程
  - 章節選擇工作流程
  - 段落編輯工作流程
  - 選項管理工作流程
  - 表單驗證工作流程
  - 編輯模式切換
  - 多語言內容工作流程
  - 段落章節篩選
  - 複雜選項結構處理

### 7. 邊界情況測試 (Edge Case Tests)
- **檔案位置**: `tests/dashboard_interaction_tests.rs::edge_case_tests`
- **測試範圍**:
  - 空段落內容
  - 選項數量不匹配
  - 無效目標引用
  - 循環引用
  - 格式錯誤的 JSON 值

### 8. 效能測試 (Performance Tests)
- **檔案位置**: `tests/dashboard_benchmark_tests.rs::benchmark_tests`
- **測試範圍**:
  - 大型資料集創建
  - 章節篩選效能
  - 段落查找效能
  - 語言內容檢索
  - 選項處理效能
  - 並發操作處理
  - 記憶體使用情況
  - 表單驗證效能

### 9. 壓力測試 (Stress Tests)
- **檔案位置**: `tests/dashboard_benchmark_tests.rs::stress_tests`
- **測試範圍**:
  - 大規模資料集處理
  - 快速語言切換
  - 並發表單操作

### 10. 無障礙性測試 (Accessibility Tests)
- **檔案位置**: `tests/dashboard_tests.rs::accessibility_tests`
- **測試範圍**:
  - 語言無障礙性
  - 內容結構無障礙性
  - 必要屬性驗證

### 11. 序列化測試 (Serialization Tests)
- **檔案位置**: `tests/dashboard_tests.rs::serialization_tests`
- **測試範圍**:
  - 段落序列化/反序列化
  - 章節序列化/反序列化
  - JSON 相容性

### 12. API 相容性測試 (API Tests)
- **檔案位置**: `tests/dashboard_tests.rs::api_tests`
- **測試範圍**:
  - 資料結構相容性
  - 系統資料結構
  - API 介面驗證

### 13. UI 狀態管理測試 (UI State Tests)
- **檔案位置**: `tests/dashboard_tests.rs::ui_state_tests`
- **測試範圍**:
  - 表單狀態驗證
  - 編輯模式狀態變化
  - 狀態持久性

## 如何執行測試

### 完整測試套件
```bash
# 執行完整的 Dashboard 測試套件
./scripts/run_dashboard_tests.sh
```

### 個別測試類別
```bash
# 單元測試
cargo test dashboard_tests::unit_tests --verbose

# 組件測試
cargo test dashboard_tests::component_tests --verbose

# 整合測試
cargo test dashboard_tests::integration_tests --verbose

# 互動測試
cargo test dashboard_interaction_tests::interaction_tests --verbose

# 效能測試
cargo test dashboard_benchmark_tests::benchmark_tests --verbose

# 壓力測試
cargo test dashboard_benchmark_tests::stress_tests --verbose
```

### 特定測試
```bash
# 執行特定測試函數
cargo test test_dashboard_props_creation --verbose

# 執行特定模組的所有測試
cargo test dashboard_tests::form_validation_tests --verbose
```

## 測試資料

### 測試語言
- 中文（台灣） (zh-TW)
- 英文 (en-US)
- 日文 (ja-JP)
- 韓文 (ko-KR)
- 法文 (fr-FR)
- 德文 (de-DE)
- 西班牙文 (es-ES)

### 測試資料規模
- 小型資料集: 3 個段落，2 個章節
- 中型資料集: 2000 個段落，50 個章節
- 大型資料集: 10000 個段落，100 個章節

### 測試場景
- 基本 CRUD 操作
- 多語言內容管理
- 複雜選項結構
- 錯誤處理和回退
- 效能極限測試

## 效能基準

### 預期效能指標
- 大型資料集創建: < 1 秒
- 章節篩選 (50 章節): < 100ms
- 段落查找 (1000 次): < 50ms
- 語言內容檢索: < 100ms
- 選項處理: < 200ms
- 並發操作: < 100ms
- 表單驗證 (1000 表單): < 50ms

### 壓力測試限制
- 大規模資料集創建: < 5 秒
- 大規模操作: < 1 秒
- 快速語言切換: < 500ms
- 並發表單操作: < 2 秒

## 測試環境要求

### 系統要求
- Rust 1.70+
- Docker 和 Docker Compose
- 至少 4GB 可用記憶體

### 依賴項
- `dioxus` - UI 框架
- `tokio` - 異步運行時
- `serde` 和 `serde_json` - 序列化
- `pretty_assertions` - 測試斷言
- `reqwest` - HTTP 客戶端

## 測試資料結構

### Paragraph 結構
```rust
Paragraph {
    id: String,
    chapter_id: String,
    texts: Vec<Text>,
    choices: Vec<ParagraphChoice>,
}
```

### Text 結構
```rust
Text {
    lang: String,
    paragraphs: String,
    choices: Vec<String>,
}
```

### ParagraphChoice 結構
```rust
ParagraphChoice::Complex {
    to: Vec<String>,
    type_: String,
    key: Option<String>,
    value: Option<serde_json::Value>,
    same_page: Option<bool>,
    time_limit: Option<u32>,
}
```

## 常見問題和解決方案

### Q: 測試執行失敗怎麼辦？
A: 
1. 確保 Docker 容器正在運行
2. 檢查依賴項是否正確安裝
3. 確認在專案根目錄執行測試
4. 查看具體錯誤訊息進行調試

### Q: 效能測試失敗怎麼辦？
A:
1. 檢查系統資源是否充足
2. 關閉其他佔用資源的程序
3. 調整測試閾值（如果合理）
4. 檢查是否有記憶體洩漏

### Q: 如何新增新的測試？
A:
1. 在相應的測試檔案中新增測試函數
2. 遵循現有的測試模式
3. 添加適當的文檔和註釋
4. 更新此 README 文件

### Q: 如何調試測試？
A:
1. 使用 `println!` 或 `tracing` 進行調試輸出
2. 運行單個測試: `cargo test test_name --verbose`
3. 使用 Rust 調試器
4. 檢查測試資料和預期結果

## 持續整合

這些測試應該在以下情況下自動執行：
- Pull Request 創建/更新時
- 主分支合併前
- 發布版本前
- 每日構建

## 測試覆蓋率目標

- 單元測試覆蓋率: > 90%
- 整合測試覆蓋率: > 80%
- 整體測試覆蓋率: > 85%

## 維護和更新

- 定期檢查測試的有效性
- 隨著功能更新而更新測試
- 監控效能基準的變化
- 保持測試文檔的最新狀態

---

如有任何問題或建議，請聯繫開發團隊或提交 issue。 