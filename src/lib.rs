mod saddle_points;
mod lsogram;
mod say;
mod run_length_encoding;
#[warn(non_snake_case)]
mod ISBN;
mod simple_linked_list;


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
