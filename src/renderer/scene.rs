use super::shape::Shape;

pub trait Scene: Send + Sync {
    fn get_shapes(&self) -> &Vec<std::sync::Arc<dyn Shape>>;
}
