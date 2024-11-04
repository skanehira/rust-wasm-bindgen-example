use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub enum MyEnum {
    One = "one",
    Two = "two",
    Three = "three",
}

#[wasm_bindgen]
pub fn get_enum() -> MyEnum {
    MyEnum::One
}
