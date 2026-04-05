use wasm_bindgen::prelude::*;
use rapidfuzz::distance::levenshtein;
use serde::{Serialize};

#[derive(Serialize)]
pub struct SearchResult {
    origin: u8,    // 0: TMから, 1: 前のTextsから
    index: u32,    // マッチした相手のインデックス
    ratio: f64,    // 類似度 (0.0 - 1.0)
}

#[derive(serde::Serialize)]
pub struct FinalResponse {
    similarities: Vec<Vec<SearchResult>>, // 前に作った [origin, index, ratio] のリスト
    term_hits: Vec<Vec<u32>>,             // 各文ごとの [TB_index, TB_index, ...]
}


#[wasm_bindgen]
pub fn search_hybrid(tm: JsValue, texts: JsValue, threshold: f64) -> JsValue {
    let tm_list: Vec<String> = serde_wasm_bindgen::from_value(tm).unwrap();
    let text_list: Vec<String> = serde_wasm_bindgen::from_value(texts).unwrap();
    
    let mut final_results: Vec<Vec<SearchResult>> = Vec::with_capacity(text_list.len());

    for i in 0..text_list.len() {
        let mut matches = Vec::new();
        let current = text_list[i].as_bytes();

        // --- A. TM（既存メモリ）との全件比較 ---
        for (idx, entry) in tm_list.iter().enumerate() {
            let score = levenshtein::normalized_similarity(current, entry.as_bytes());
            if score >= threshold {
                matches.push(SearchResult { origin: 0, index: idx as u32, ratio: score });
            }
        }

        // --- B. 自分の前の要素（texts）との比較 ---
        for j in 0..i {
            let score = levenshtein::normalized_similarity(current, text_list[j].as_bytes());
            if score >= threshold {
                matches.push(SearchResult { origin: 1, index: j as u32, ratio: score });
            }
        }

        // スコア順にソート（一番似ているものを先頭に）
        matches.sort_by(|a, b| b.ratio.partial_cmp(&a.ratio).unwrap());
        final_results.push(matches);
    }

    serde_wasm_bindgen::to_value(&final_results).unwrap()
}