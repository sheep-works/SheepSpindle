use wasm_bindgen::prelude::*;
use rapidfuzz::distance::levenshtein;
use aho_corasick::AhoCorasick;
use serde::Serialize;

// JS側に返す解析結果の構造体
#[derive(Serialize)]
pub struct AnalysisResult {
    pub t: Vec<u32>, // Translation Memory：類似する翻訳メモリ(TM)のインデックス
    pub i: Vec<u32>, // Internal Similarity：現在のテキスト(Texts)内での前方一致インデックス
    pub g: Vec<u32>, // Glossary：ヒットした用語集(TB)のインデックス
}

// --- [内部用] 類似度探索のロジック (JSからは直接見えない) ---
fn internal_tm_search(current: &str, tm_list: &[String], threshold: f64) -> Vec<u32> {
    let current_bytes = current.as_bytes();
    tm_list.iter().enumerate()
        .filter(|(_, entry)| {
            levenshtein::normalized_similarity(current_bytes, entry.as_bytes()) >= threshold
        })
        .map(|(idx, _)| idx as u32)
        .collect()
}

// --- [内部用] 用語抽出のロジック ---
fn internal_tb_search(current: &str, ac: &AhoCorasick) -> Vec<u32> {
    let mut hits: Vec<u32> = ac.find_iter(current)
        .map(|mat| mat.pattern().as_u32())
        .collect();
    hits.sort_unstable();
    hits.dedup();
    hits
}

// ==========================================
//   ここから JS (Wasm) 用の公開インターフェース
// ==========================================

// 1. [単体] 類似度だけ調べたい時用
#[wasm_bindgen]
pub fn only_tm_search(tm: JsValue, texts: JsValue, threshold: f64) -> JsValue {
    // 1. JSからのデータをRustの型に変換（シリアライズ）
    let tm_list: Vec<String> = serde_wasm_bindgen::from_value(tm).unwrap();
    let text_list: Vec<String> = serde_wasm_bindgen::from_value(texts).unwrap();
    
    let results: Vec<Vec<u32>> = text_list.iter()
        .map(|txt| internal_tm_search(txt, &tm_list, threshold))
        .collect();
        
    serde_wasm_bindgen::to_value(&results).unwrap()
}

// 2. [単体] 用語抽出だけしたい時用
#[wasm_bindgen]
pub fn only_tb_search(texts: JsValue, tb: JsValue) -> JsValue {
    let text_list: Vec<String> = serde_wasm_bindgen::from_value(texts).unwrap();
    let tb_list: Vec<String> = serde_wasm_bindgen::from_value(tb).unwrap();
    let ac = AhoCorasick::new(&tb_list).unwrap();

    let results: Vec<Vec<u32>> = text_list.iter()
        .map(|txt| internal_tb_search(txt, &ac))
        .collect();

    serde_wasm_bindgen::to_value(&results).unwrap()
}

// 3. [全部盛り] これまでの analyze_text
#[wasm_bindgen]
pub fn analyze_all(tm: JsValue, texts: JsValue, tb: JsValue, threshold: f64) -> JsValue {
    // 1. JSからのデータ (JsValue) をRustの型 (Vec<String>) に変換（シリアライズ）
    // serde_wasm_bindgen を使うことで、JS配列をRustのVecへ簡単にマッピングできます。
    let tm_list: Vec<String> = serde_wasm_bindgen::from_value(tm).unwrap();
    let text_list: Vec<String> = serde_wasm_bindgen::from_value(texts).unwrap();
    let tb_list: Vec<String> = serde_wasm_bindgen::from_value(tb).unwrap();

    // 2. 用語集(TB)検索用の AhoCorasick インスタンスを作成
    // 固定されたキーワード群に対して高速にマルチパターンマッチングを行うための前準備です。
    let ac = AhoCorasick::new(&tb_list).unwrap();

    // 3. 各テキストに対して解析を実行
    // iter().enumerate() を使うことで、要素 (txt) と同時にそのインデックス (i) を取得できます。
    let results: Vec<AnalysisResult> = text_list.iter().enumerate()
        .map(|(i, txt)| {
            // 内部関数を呼び出して、現在のテキストに対する解析ロジックを実行
            let tm_hits = internal_tm_search(txt, &tm_list, threshold);
            let tb_hits = internal_tb_search(txt, &ac);

            // 前方一致（ファイル内重複）のチェック
            // 自分より前の位置にあるテキストを走査して、類似度がしきい値以上のものを抽出します。
            let prev_hits: Vec<u32> = text_list[0..i].iter().enumerate()
                .filter(|(_, prev_txt)| {
                    let current_bytes = txt.as_bytes();
                    levenshtein::normalized_similarity(current_bytes, prev_txt.as_bytes()) >= threshold
                })
                .map(|(prev_idx, _)| prev_idx as u32)
                .collect();

            // 最終的な解析結果を構造体に詰めて返します。
            AnalysisResult {
                t: tm_hits,
                i: prev_hits,
                g: tb_hits,
            }
        })
        .collect(); // mapの結果(イテレータ)をVecにまとめます

    // 4. 解析結果 (Vec<AnalysisResult>) を JS が扱える JsValue に変換して返却
    serde_wasm_bindgen::to_value(&results).unwrap()
}
