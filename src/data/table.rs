//! Проверка данных и создание запроса sql для заполнения таблицы в БД
use crate::error::Error;
///
pub trait Table {
    ///
    fn parse(&mut self) -> Result<(), Error>;
    ///
    fn to_sql(&mut self, id: usize) -> Vec<String>;
}