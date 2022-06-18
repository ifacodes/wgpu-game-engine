use uuid::Uuid;

pub struct Handle<T> {
    name: String,
    uuid: Uuid,
    val: Option<T>,
}
