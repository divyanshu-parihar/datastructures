pub trait Collectable<T, E> {
    fn add(&mut self, element: T) -> Result<(), E>;
    fn remove(&mut self, itentifier: T) -> Result<(), E>;
}
