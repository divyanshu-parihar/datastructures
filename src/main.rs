mod collection;
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    mod vector {
        use std::io::error;

        use crate::collection::base::collectable;
        use crate::collection::vector::vector;
        #[test]
        fn vector_init() -> result<(), error> {
            let vector: vector<string> = vector::new();

            assert_eq!(vector.get_len(), 0);
            return ok(());
        }
    }
}
