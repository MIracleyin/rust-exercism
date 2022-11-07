mod saddle_points;
mod lsogram;
mod say;
mod run_length_encoding;
mod ISBN;
mod simple_linked_list;
mod sum_of_mul;
mod perfect_num;
mod grains;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
