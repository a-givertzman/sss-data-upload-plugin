//! Проверка данных и создание запроса sql для заполнения таблицы в БД
use crate::error::Error;
///
pub trait Table {
    ///
    fn parse(&mut self) -> Result<(), Error>;
    ///
    fn to_sql(&mut self, id: usize) -> Vec<String>;
    ///
    fn data(&mut self) -> Option<String>;
    ///
    fn split_data(&mut self) -> Result<Vec<Vec<String>>, Error> {
        Ok(self
            .data()
            .take()
            .ok_or(Error::FromString(
                "Table split data error: no data!".to_owned(),
            ))?
            .replace(" ", "")
            .replace(",", ".")
            .split("\r\n")
            .map(|line| {
                line.split(';')
                    .filter(|s| s.len() > 0)
                    .map(|s| s.to_owned())
                    .collect::<Vec<String>>()
            })
            .filter(|s| s.len() > 0)
            .collect())
    }
}
