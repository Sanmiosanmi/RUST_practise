pub enum Result<T,E>{
    Ok(T),// THE OK variant indicate a success. it stores an associated piece of data of geeneric type T
    Err(E),
}
