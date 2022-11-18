use std::collections::HashMap;

static VALID_NUCLEOTIDES: &'static str = "ACGT";

fn valid(c: char) -> Result<char, char> {
    if VALID_NUCLEOTIDES.contains(c) {
        Ok(c)
    } else {
        Err(c)
    }
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    valid(nucleotide)?;
    let mut count = 0;
    
    for c in dna.chars() {
        if valid(c)? == nucleotide {
            count += 1;
        }
    }

    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map: HashMap<char, usize> = VALID_NUCLEOTIDES.chars().map(|c| (c, 0)).collect();

    for nuc in dna.chars() {
        if let Some(n) = map.get_mut(&nuc) {
            *n += 1;
        } else {
            return Err(nuc);
        }
    }
    Ok(map)
}

// use std::collections::HashMap;

fn check_dna(s: &str, pairs: &[(char, usize)]) {
    // The reason for the awkward code in here is to ensure that the failure
    // message for assert_eq! is as informative as possible. A simpler
    // solution would simply check the length of the map, and then
    // check for the presence and value of each key in the given pairs vector.
    let mut m: HashMap<char, usize> = nucleotide_counts(s).unwrap();
    for &(k, v) in pairs.iter() {
        assert_eq!((k, m.remove(&k)), (k, Some(v)));
    }
    // may fail with a message that clearly shows all extra pairs in the map
    assert_eq!(m.iter().collect::<Vec<(&char, &usize)>>(), vec![]);
}

#[test]
fn count_returns_result() {
    assert!(count('A', "").is_ok());
}

#[test]
//#[ignore]
fn test_count_empty() {
    assert_eq!(count('A', ""), Ok(0));
}

#[test]
//#[ignore]
fn count_invalid_nucleotide() {
    assert_eq!(count('X', "A"), Err('X'));
}

#[test]
//#[ignore]
fn count_invalid_dna() {
    assert_eq!(count('A', "AX"), Err('X'));
}

#[test]
//#[ignore]
fn test_count_repetitive_cytosine() {
    assert_eq!(count('C', "CCCCC"), Ok(5));
}

#[test]
//#[ignore]
fn test_count_only_thymine() {
    assert_eq!(count('T', "GGGGGTAACCCGG"), Ok(1));
}

#[test]
//#[ignore]
fn counts_returns_result() {
    assert!(nucleotide_counts("ACGT").is_ok());
}

#[test]
//#[ignore]
fn test_nucleotide_count_empty() {
    check_dna("", &[('A', 0), ('T', 0), ('C', 0), ('G', 0)]);
}

#[test]
//#[ignore]
fn test_nucleotide_count_only_guanine() {
    check_dna("GGGGGGGG", &[('A', 0), ('T', 0), ('C', 0), ('G', 8)]);
}

#[test]
//#[ignore]
fn test_nucleotide_count_counts_all() {
    check_dna(
        "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAA\
        GAGTGTCTGATAGCAGC",
        &[('A', 20), ('T', 21), ('C', 12), ('G', 17)],
    );
}

#[test]
//#[ignore]
fn counts_invalid_nucleotide_results_in_err() {
    assert_eq!(nucleotide_counts("GGXXX"), Err('X'));
}
