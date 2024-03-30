#[warn(dead_code)]
mod array;
mod item;
mod triple_work;
mod tuple;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}