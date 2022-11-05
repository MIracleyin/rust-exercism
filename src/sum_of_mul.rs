// 倍数之和
// 给定一个数字,找出另外的特定数字的所有唯一倍数的总和,但不包括第一个数字.

// 如果我们列出20以下，3或5的倍数的所有自然数,我们得到3,5,6,9,10,12,15和18.

// 这些倍数的总和是78.

use core::num;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut nums: Vec<u32> = Vec::new();
    for i in 1..limit {
        for factor in factors.iter() {
            if i % factor == 0 {
                if nums.contains(&i) {
                    continue;
                } else {
                    nums.push(i);
                }
            }
        }
    }
    nums.iter().sum()
}


#[test]
fn multiples_one() {
   assert_eq!(0, sum_of_multiples(1, &[3, 5]))
}

#[test]
//#[ignore]
fn multiples_two() {
    assert_eq!(3, sum_of_multiples(4, &[3, 5]))
}

#[test]
//#[ignore]
fn multiples_three() {
   assert_eq!(23, sum_of_multiples(10, &[3, 5]))
}

#[test]
//#[ignore]
fn multiples_four() {
   assert_eq!(2318, sum_of_multiples(100, &[3, 5]))
}

#[test]
//#[ignore]
fn multiples_five() {
   assert_eq!(233168, sum_of_multiples(1000, &[3, 5]))
}

#[test]
//#[ignore]
fn multiples_six() {
   assert_eq!(51, sum_of_multiples(20, &[7, 13, 17]))
}

#[test]
//#[ignore]
fn multiples_seven() {
   assert_eq!(30, sum_of_multiples(15, &[4, 6]))
}

#[test]
//#[ignore]
fn multiples_eight() {
   assert_eq!(4419, sum_of_multiples(150, &[5, 6, 8]))
}

#[test]
//#[ignore]
fn multiples_nine() {
   assert_eq!(275, sum_of_multiples(51, &[5, 25]))
}

#[test]
//#[ignore]
fn multiples_ten() {
   assert_eq!(2203160, sum_of_multiples(10000, &[43, 47]))
}

#[test]
//#[ignore]
fn multiples_eleven() {
   assert_eq!(4950, sum_of_multiples(100, &[1]))
}

#[test]
//#[ignore]
fn multiples_twelve() {
   assert_eq!(0, sum_of_multiples(10000, &[]))
}
