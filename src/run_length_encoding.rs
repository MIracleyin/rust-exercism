use std::cmp;

pub fn encode(source: &str) -> String {
    source
        .chars() // to a iter()
        .fold(
            (String::new(), ' ', 0, 1),
            |(mut acc, last, last_n, pos), c| {
                // acc = where answer is accumulated
                // last = last character read
                // last_n = accum count for last
                // pos save char index whatever will add 1
                if c == last { // if same as last one 
                    if pos == source.len() { // if is last char
                        // if is end of string
                        acc += (last_n + 1).to_string().as_str();
                        acc.push(c);
                    }
                    (acc, last, last_n + 1, pos + 1)
                } else { // a new char
                    if last_n > 1 {
                        acc += last_n.to_string().as_str(); // add number
                    }
                    if last_n > 0 {
                        acc.push(last); // add char
                    }
                    if pos == source.len() { // if a new char but end
                        acc.push(c); 
                    }
                    (acc, c, 1, pos + 1) // is a new char, so reset last, and backup last_n 
                }
            },
        )
        .0 // accumulator

         
}
 
 pub fn decode(source: &str) -> String {
    source
        .chars()
        .fold(
            (String::new(), 0),
            |(mut acc, last_n), c| {
                if let Some(d) = c.to_digit(10) { // if can't match number
                    (acc, 10 * last_n + d) // last_n keep number of rank 
                } else {
                    acc += c.to_string().repeat(cmp::max(last_n, 1) as usize).as_str();
                    (acc, 0) // if last char add to string reset last_n
                }
            }
        ).0
}
 



#[test]
fn test_encode_empty_string() {
   assert_eq!("", encode(""));
}

#[test]
//#[ignore]
fn test_encode_single_characters() {
   assert_eq!("XYZ", encode("XYZ"));
}

#[test]
//#[ignore]
fn test_encode_string_with_no_single_characters() {
   assert_eq!("2A3B4C", encode("AABBBCCCC"));
}

#[test]
//#[ignore]
fn test_encode_single_characters_mixed_with_repeated_characters() {
   assert_eq!(
       "12WB12W3B24WB",
       encode("WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB")
   );
}

#[test]
//#[ignore]
fn test_encode_multiple_whitespace_mixed_in_string() {
   assert_eq!("2 hs2q q2w2 ", encode("  hsqq qww  "));
}

#[test]
//#[ignore]
fn test_encode_lowercase_characters() {
   assert_eq!("2a3b4c", encode("aabbbcccc"));
}

// decoding tests

#[test]
//#[ignore]
fn test_decode_empty_string() {
   assert_eq!("", decode(""));
}

#[test]
//#[ignore]
fn test_decode_single_characters_only() {
   assert_eq!("XYZ", decode("XYZ"));
}

#[test]
//#[ignore]
fn test_decode_string_with_no_single_characters() {
   assert_eq!("AABBBCCCC", decode("2A3B4C"));
}

#[test]
//#[ignore]
fn test_decode_single_characters_with_repeated_characters() {
   assert_eq!(
       "WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB",
       decode("12WB12W3B24WB")
   );
}

#[test]
//#[ignore]
fn test_decode_multiple_whitespace_mixed_in_string() {
   assert_eq!("  hsqq qww  ", decode("2 hs2q q2w2 "));
}

#[test]
//#[ignore]
fn test_decode_lower_case_string() {
   assert_eq!("aabbbcccc", decode("2a3b4c"));
}

// consistency test

#[test]
//#[ignore]
fn test_consistency() {
   assert_eq!("zzz ZZ  zZ", decode(encode("zzz ZZ  zZ").as_str()));
}
