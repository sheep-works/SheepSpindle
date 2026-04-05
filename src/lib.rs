use wasm_bindgen::prelude::*;
use rapidfuzz::distance::levenshtein;
use aho_corasick::AhoCorasick;
use serde::Serialize;

// JS側に返す解析結果の構造体
#[derive(Serialize)]
pub struct AnalysisResult {
    pub tm_indices: Vec<u32>,   // 類似する翻訳メモリ(TM)のインデックス
    pub prev_indices: Vec<u32>, // 現在のテキスト(Texts)内での前方一致インデックス
    pub tb_indices: Vec<u32>,   // ヒットした用語集(TB)のインデックス
}

#[wasm_bindgen]
pub fn analyze_text(tm: JsValue, texts: JsValue, tb: JsValue, threshold: f64) -> JsValue {
    // 1. JSからのデータをRustの型に変換（シリアライズ）
    let tm_list: Vec<String> = serde_wasm_bindgen::from_value(tm).unwrap();
    let text_list: Vec<String> = serde_wasm_bindgen::from_value(texts).unwrap();
    let tb_list: Vec<String> = serde_wasm_bindgen::from_value(tb).unwrap();

    // 2. TB（用語集）検索用の「Aho-Corasick」検索機をビルド
    // この一行で、全用語を一度にスキャンできる「木構造」が作られます
    let ac = AhoCorasick::new(&tb_list).expect("TB検索機の作成に失敗しました");

    let mut final_results = Vec::with_capacity(text_list.len());

    // 各文（texts）に対してループ処理
    for i in 0..text_list.len() {
        let current_str = &text_list[i];
        let current_bytes = current_str.as_bytes(); // 比較高速化のためバイト列へ

        // --- A. TM（翻訳メモリ）との全件比較 ---
        // Levenshtein距離でしきい値を超えるものだけインデックスを残す
        let tm_indices: Vec<u32> = tm_list.iter().enumerate()
            .filter(|(_, entry)| {
                levenshtein::normalized_similarity(current_bytes, entry.as_bytes()) >= threshold
            })
            .map(|(idx, _)| idx as u32)
            .collect();

        // --- B. 同じファイル内の「自分より前の文」との比較 ---
        let prev_indices: Vec<u32> = (0..i)
            .filter(|&j| {
                levenshtein::normalized_similarity(current_bytes, text_list[j].as_bytes()) >= threshold
            })
            .map(|j| j as u32)
            .collect();

        // --- C. TB（用語集）の抽出 ---
        // テキストを一回なぞるだけで、登録用語をすべて見つける（AC法）
        let mut tb_indices: Vec<u32> = ac.find_iter(current_str)
            .map(|mat| mat.pattern().as_u32()) // 見つかった用語のインデックスを取得
            .collect();
        
        // 重複ヒット（同じ単語が2回出た場合など）を整理
        tb_indices.sort_unstable();
        tb_indices.dedup();

        // 3. 結果をまとめてリストに追加
        final_results.push(AnalysisResult {
            tm_indices,
            prev_indices,
            tb_indices,
        });
    }

    // 4. 全結果をJSが理解できる形式に変換して返却
    serde_wasm_bindgen::to_value(&final_results).unwrap()
}