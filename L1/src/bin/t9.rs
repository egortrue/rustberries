// L1.9
use std::i64;

/// Устанавливает у числа (source) I-ый бит (bit) в значение (value)
fn set_bit(source: i64, bit: u8, value: u8) -> i64 {
    assert!(bit <= 63); // 0..63
    assert!(value == 0 || value == 1);

    let mask: i64 = 1 << bit;
    if value == 1 {
        source | mask
    } else {
        source & !mask
    }
}

// Немного тестирования с выводом
fn main() {
    let mut source;
    let mut bit;
    let mut value;
    let mut expect;
    let mut result;

    source = 31;
    bit = 0;
    value = 0; // sub
    expect = 31 - 1;
    result = set_bit(source, bit, value);
    println!("{source} ( {source:b} ) -> set bit {bit} to {value} -> {result} ( {result:b} )");
    assert_eq!(result, expect);

    source = 31;
    bit = 4;
    value = 0; // sub
    expect = 31 - 16;
    result = set_bit(source, bit, value);
    println!("{source} ( {source:b} ) -> set bit {bit} to {value} -> {result} ( {result:b} )");
    assert_eq!(result, expect);

    source = 127;
    bit = 3;
    value = 0; // sub
    expect = 127 - 8;
    result = set_bit(source, bit, value);
    println!("{source} ( {source:b} ) -> set bit {bit} to {value} -> {result} ( {result:b} )");
    assert_eq!(result, expect);

    source = 128;
    bit = 3;
    value = 1; // add
    expect = 128 + 8;
    result = set_bit(source, bit, value);
    println!("{source} ( {source:b} ) -> set bit {bit} to {value} -> {result} ( {result:b} )");
    assert_eq!(result, expect);

    source = i64::MAX;
    bit = 63;
    value = 1; // add to overflow
    expect = -1;
    result = set_bit(source, bit, value);
    println!("{source} ( {source:b} ) -> set bit {bit} to {value} -> {result} ( {result:b} )");
    assert_eq!(result, expect);

    source = i64::MIN;
    bit = 63;
    value = 0; // sub to overflow
    expect = 0;
    result = set_bit(source, bit, value);
    println!("{source} ( {source:b} ) -> set bit {bit} to {value} -> {result} ( {result:b} )");
    assert_eq!(result, expect);
}
