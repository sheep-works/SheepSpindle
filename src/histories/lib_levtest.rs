use wasm_bindgen::prelude::*;
use rapidfuzz::distance::levenshtein;

#[wasm_bindgen]
pub fn find_similar_pairs(input: JsValue, threshold: f64) -> JsValue {
    // 1. JSの配列をRustのベクトルに変換
    let data: Vec<String> = serde_wasm_bindgen::from_value(input).unwrap();
    
    // 2. 結果を格納するベクタ（配列の配列）
    // 例: [[], [0], [], [1, 2]] -> 1番目は0番目と似ている、3番目は1,2番目と似ている...
    let mut results: Vec<Vec<u32>> = Vec::with_capacity(data.len());

    for i in 0..data.len() {
        let mut matches = Vec::new();
        let current = data[i].as_bytes();

        for j in 0..i {
            let previous = data[j].as_bytes();
            
            // 類似度計算 (0.0 ~ 1.0)
            let score = levenshtein::normalized_similarity(current, previous);

            if score >= threshold {
                matches.push(j as u32);
            }
        }
        results.push(matches);
    }

    // 3. 複雑な構造も serde で JS に一発変換
    serde_wasm_bindgen::to_value(&results).unwrap()
}