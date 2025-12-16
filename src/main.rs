mod collection;
mod problems;
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    mod vector {
        use crate::collection::base::Collectable;
        use crate::collection::vector::NaiveVector;
        #[test]
        fn vector_init() -> () {
            let vector: NaiveVector<String> = NaiveVector::new();
            assert_eq!(vector.get_len(), 0);
        }
        #[test]
        fn vector_add() -> () {
            let mut vector: NaiveVector<u8> = NaiveVector::new();
            let _ = vector.add(1);
            assert_eq!(vector.get_len(), 1);
        }
        #[test]
        fn vector_add_capacity() -> () {
            let mut vector: NaiveVector<u8> = NaiveVector::new();
            let _ = vector.add(1);
            let _ = vector.add(2);
            assert_eq!(vector.get_len(), 2);
            assert_eq!(vector.get_capacity(), 2);
        }
    }
}
