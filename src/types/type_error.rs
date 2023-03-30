#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    TypeError,
    UndeclaredFunction,
    UndeclaredVariable,
}
