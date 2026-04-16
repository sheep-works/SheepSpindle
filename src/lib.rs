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

// --- [内部用] 記号・数字のみで構成されているか判定するロジック ---
fn is_only_digits_and_symbols(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    s.chars().all(|c| {
        matches!(c,
            '0'..='9' | '０'..='９' |
            '(' | ')' | '（' | '）' | '【' | '】' | '[' | ']' |
            '%' | '％' |
            '.' | ',' | ':' | ';' | '/' | '+' | '±' |
            ' ' | '　' |
            '"' | '\'' | '’' | '!' | '?' |
            '“' | '”' | '‘' | '—' | '–' | '‑' | '_' | '\\' | '&' | '@' | '#' | '*' | '=' | '~' |
            '、' | '。' | '・' | '：' | '「' | '」' | '『' | '』' | 'ー' | '―' | '…' | '‥' | '；' |
            '$' | '€' | '£' | '¥' | '￥' | '-'
        )
    })
}

// --- [内部用] 類似度探索のロジック (JSからは直接見えない) ---
fn internal_tm_search(current: &str, tm_list: &[String], threshold: f64, limit: usize) -> Vec<u32> {
    if is_only_digits_and_symbols(current) {
        return Vec::new();
    }
    let current_bytes = current.as_bytes();
    let mut results: Vec<(u32, f64)> = tm_list.iter().enumerate()
        .filter_map(|(idx, entry)| {
            if is_only_digits_and_symbols(entry) {
                return None;
            }
            let sim = levenshtein::normalized_similarity(current_bytes, entry.as_bytes());
            if sim >= threshold {
                Some((idx as u32, sim))
            } else {
                None
            }
        })
        .collect();

    // 類似度の高い順にソート
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    // limit 個分だけインデックスを抽出
    results.into_iter()
        .take(limit)
        .map(|(idx, _)| idx)
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
pub fn only_tm_search(tm: JsValue, texts: JsValue, threshold: f64, counts: Option<i32>) -> JsValue {
    let c = counts.unwrap_or(5);
    let limit = if c <= 0 { usize::MAX } else { c as usize };

    // 1. JSからのデータをRustの型に変換（シリアライズ）
    let tm_list: Vec<String> = serde_wasm_bindgen::from_value(tm).unwrap();
    let text_list: Vec<String> = serde_wasm_bindgen::from_value(texts).unwrap();
    
    let results: Vec<Vec<u32>> = text_list.iter()
        .map(|txt| internal_tm_search(txt, &tm_list, threshold, limit))
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
pub fn analyze_all(tm: JsValue, texts: JsValue, tb: JsValue, threshold: f64, counts: Option<i32>) -> JsValue {
    let c = counts.unwrap_or(5);
    let limit = if c <= 0 { usize::MAX } else { c as usize };

    // 1. JSからのデータ (JsValue) をRustの型 (Vec<String>) に変換（シリアライズ）
    let tm_list: Vec<String> = serde_wasm_bindgen::from_value(tm).unwrap();
    let text_list: Vec<String> = serde_wasm_bindgen::from_value(texts).unwrap();
    let tb_list: Vec<String> = serde_wasm_bindgen::from_value(tb).unwrap();

    // 2. 用語集(TB)検索用の AhoCorasick インスタンスを作成
    let ac = AhoCorasick::new(&tb_list).unwrap();

    // 3. 各テキストに対して解析を実行
    let results: Vec<AnalysisResult> = text_list.iter().enumerate()
        .map(|(i, txt)| {
            // 内部関数を呼び出して、現在のテキストに対する解析ロジックを実行
            let tm_hits = internal_tm_search(txt, &tm_list, threshold, limit);
            let tb_hits = internal_tb_search(txt, &ac);

            // 前方一致（ファイル内重複）のチェック
            let mut prev_hits_with_sim: Vec<(u32, f64)> = if is_only_digits_and_symbols(txt) {
                Vec::new()
            } else {
                text_list[0..i].iter().enumerate()
                    .filter_map(|(prev_idx, prev_txt)| {
                        if is_only_digits_and_symbols(prev_txt) {
                            return None;
                        }
                        let current_bytes = txt.as_bytes();
                        let sim = levenshtein::normalized_similarity(current_bytes, prev_txt.as_bytes());
                        if sim >= threshold {
                            Some((prev_idx as u32, sim))
                        } else {
                            None
                        }
                    })
                    .collect()
            };

            // 類似度の高い順にソートし、limit個分抽出
            prev_hits_with_sim.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            let prev_hits: Vec<u32> = prev_hits_with_sim.into_iter()
                .take(limit)
                .map(|(idx, _)| idx)
                .collect();

            // 最終的な解析結果を構造体に詰めて返します。
            AnalysisResult {
                t: tm_hits,
                i: prev_hits,
                g: tb_hits,
            }
        })
        .collect();

    // 4. 解析結果 (Vec<AnalysisResult>) を JS が扱える JsValue に変換して返却
    serde_wasm_bindgen::to_value(&results).unwrap()
}


// 4. [ゆれチェック] グループ化ロジック (SheepComb/scripts/consistency_chunker.py の移植)
#[derive(serde::Deserialize, Serialize, Clone)]
pub struct ConsistencyItem {
    pub idx: u32,
    pub src: String,
    pub tgt: String,
}

#[wasm_bindgen]
pub fn get_consistency_groups(segments: JsValue, threshold: f64) -> JsValue {
    // 1. JSからのデータをデシリアライズ
    let mut items: Vec<ConsistencyItem> = serde_wasm_bindgen::from_value(segments).unwrap();
    let mut all_groups: Vec<Vec<ConsistencyItem>> = Vec::new();

    // 2. チャンキングアルゴリズム
    while !items.is_empty() {
        let seed = items.remove(0);
        let mut current_group = vec![seed.clone()];
        let mut next_remaining = Vec::new();

        let seed_src_bytes = seed.src.as_bytes();

        for item in items {
            // 原文が完全一致
            if seed.src == item.src {
                if seed.tgt == item.tgt {
                    // 訳文まで一致ならグループには入れない
                    continue;
                } else {
                    // 訳文が違う = ゆれ確定
                    current_group.push(item);
                }
            } else {
                // 類似度計算 (RapidFuzzのLevenshtein)
                // normalized_similarity は 0.0 ~ 1.0 なので 100倍する
                let score = levenshtein::normalized_similarity(seed_src_bytes, item.src.as_bytes()) * 100.0;
                if score >= threshold {
                    current_group.push(item);
                } else {
                    next_remaining.push(item);
                }
            }
        }
        
        // 最初のseed以外に類似・不一致があった場合、または単独でもグループとして残すなら追加
        // (Python版のロジックでは単独のseedも追加されるため、それに合わせる)
        all_groups.push(current_group);
        items = next_remaining;
    }

    // 3. JSにシリアライズして返す
    serde_wasm_bindgen::to_value(&all_groups).unwrap()
}
