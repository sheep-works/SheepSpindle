use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn bench_wasm(input: JsValue) -> u32 {
    let data: Vec<String> = serde_wasm_bindgen::from_value(input).unwrap();
    let mut count = 0;

    for i in 0..data.len() {
        for j in 0..i {
            if data[i].contains(&data[j]) || data[j].contains(&data[i]) {
                count += 1;
                break;
            }
        }
    }
    count
}