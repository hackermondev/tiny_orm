#[derive(Debug)]
pub enum ColumnOperator<T> {
    Equal(T),
    NotEqual(T),
    GreaterThan(T),
    LessThan(T),
}

#[derive(Debug)]
pub enum ArrayColumnOperator<T> {
    Contains(T),
    NotContains(T),
}
