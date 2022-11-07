// 谷物
// 计算棋盘上的小麦粒数,假设每个方格的数量增加一倍.

// 曾经有一位睿智的仆人拯救了王子的生命。国王承诺支付仆人梦寐以求的一切。知道国王喜欢国际象棋,仆人告诉国王他想吃小麦粒，在棋盘的第一个正方形上放一粒小麦。而接下来的方格是两粒.四粒小麦放在第三格,依此类推.

// 棋盘上有 64 个方格.

// 编写代码，用来显示:

// 每个方格上有多少谷物,和
// 谷物总数

pub fn square(s: u32) -> u64 {
    if s == 0 || s > 64 {
        panic!("Square must be between 1 and 64")
    }
    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    (1..=64).map(|i| square(i)).sum()
    // (1..65).fold(0, |acc, s| acc + square(s))
}

#[test]
fn square_one() {
    assert_eq!(square(1), 1);
}

#[test]
//#[ignore]
fn square_two() {
    assert_eq!(square(2), 2);
}

#[test]
//#[ignore]
fn square_three() {
    assert_eq!(square(3), 4);
}

#[test]
//#[ignore]
fn square_four() {
    assert_eq!(square(4), 8);
}

#[test]
//#[ignore]
fn square_sixteen() {
    assert_eq!(square(16), 32_768);
}

#[test]
//#[ignore]
fn square_thirty_two() {
    assert_eq!(square(32), 2_147_483_648);
}

#[test]
//#[ignore]
fn square_sixty_four() {
    assert_eq!(square(64), 9_223_372_036_854_775_808);
}

#[test]
//#[ignore]
#[should_panic(expected = "Square must be between 1 and 64")]
fn square_zero_panics() {
    square(0);
}

#[test]
//#[ignore]
#[should_panic(expected = "Square must be between 1 and 64")]
fn square_sixty_five_panics() {
    square(65);
}

#[test]
//#[ignore]
fn total_sums_all_squares() {
    assert_eq!(total(), 18_446_744_073_709_551_615);
}
