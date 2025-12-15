# Temperature Conversion (Rust)

這是一個使用 **Rust** 撰寫的簡單命令列（CLI）溫度轉換程式，  
可以在攝氏溫度與華氏溫度之間進行轉換。  
本專案主要作為 Rust 入門學習與 Git / GitHub 操作練習使用。

---

## 功能說明

1. 攝氏溫度轉華氏溫度  
2. 華氏溫度轉攝氏溫度  
3. 命令列互動操作  
4. 輸入檢查，避免非數字或錯誤選項  
5. 支援小數溫度輸入  
6. 正確處理標準輸出緩衝（`stdout().flush()`）

---

## 程式執行流程

1. 程式啟動後顯示轉換選單  
2. 使用者輸入 `1` 或 `2` 選擇轉換模式  
3. 使用者輸入對應的溫度數值  
4. 程式計算並顯示轉換結果  
5. 程式結束  

---

## 執行範例

1. 攝氏溫度轉華氏溫度  
2. 華氏溫度轉攝氏溫度  
請輸入1或2  
1  
請輸入攝氏溫度 = 25  
25度攝氏 = 77度華氏  

---

## 執行方式

請先確認系統已安裝 Rust，  
在專案目錄中執行以下指令：

cargo run

---

## 專案結構

temperature-conversion-rust/  
├─ Cargo.toml  
├─ Cargo.lock  
├─ .gitignore  
└─ src/  
   └─ main.rs  

---

## Git 與 GitHub 使用流程紀錄

本專案使用 Git 進行版本控制，並同步至 GitHub。

### 基本流程

1. 初始化 Git 專案  
   git init  

2. 設定忽略檔案（避免上傳編譯產物）  
   建立 `.gitignore`，內容包含：  
   /target  

3. 將檔案加入版本控制  
   git add .  

4. 建立版本紀錄  
   git commit -m "Initial commit"  

5. 在 GitHub 建立遠端 repository  

6. 設定遠端倉庫位置  
   git remote add origin <GitHub repository URL>  

7. 推送至 GitHub  
   git branch -M main  
   git push -u origin main  

### 後續修改流程

git add .  
git commit -m "更新說明"  
git push  

---

## 學習重點紀錄

1. 使用 `parse()` 將使用者輸入的字串安全轉換為數值  
2. 理解 `parse` 與 `as` 在 Rust 中的差異與使用時機  
3. 知道為什麼陣列索引必須使用 `usize`  
4. 理解 `print!` 搭配 `stdout().flush()` 的必要性  
5. 使用 `loop` 作為表達式並回傳值  
6. 實際操作 Git 與 GitHub 的完整流程（init、commit、push）  

---

## 備註

本專案為學習用途，程式設計以可讀性與基礎 Rust 概念為主。
