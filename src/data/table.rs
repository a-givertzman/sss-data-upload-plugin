//! Проверка данных и создание запросов sql для заполнения таблиц в БД
use crate::error::Error;
/// Проверка данных и создание запросов sql для заполнения таблиц в БД
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
    ///
    fn data_to_sql(&self, data: &Vec<(f64, f64)>, table_name: &str, ship_id: usize) -> Vec<String> {
        let mut result = Vec::new();
        result.push(format!(" DELETE FROM {table_name} WHERE ship_id={ship_id};"));
        let mut sql = format!(" INSERT INTO {table_name} (ship_id, key, value) VALUES");
        data.iter().for_each(|(k, v)| {
            sql += &format!(" ({ship_id}, {k}, {v}),");
        });
        sql.pop();
        sql.push(';');
    //    dbg!(&sql);
        result.push(sql);
        result
    }
}
