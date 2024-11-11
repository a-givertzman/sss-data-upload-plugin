//! Проверка данных и создание запросов sql для заполнения таблиц в БД
use crate::error::Error;
pub fn split_data(data: &str) -> Result<Vec<Vec<String>>, Error> {
    Ok(data
 //       .replace(" ", "")
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
/// Проверка данных и создание запросов sql для заполнения таблиц в БД
pub trait Table {
    // 
    fn parse(&mut self) -> Result<(), Error>;
    //
    fn to_sql(&self, id: usize) -> Vec<String>;
    //
    fn to_file(&self, id: usize, name: &str) {}
    //
    fn data_to_sql(&self, data: &Vec<(f64, f64)>, table_name: &str, ship_id: usize) -> Vec<String> {
        let mut result = Vec::new();
        result.push(format!(" DELETE FROM {table_name} WHERE ship_id={ship_id};\n\n"));
        let mut sql = format!(" INSERT INTO {table_name}\n  (ship_id, key, value)\nVALUES\n");
        data.iter().for_each(|(k, v)| {
            sql += &format!("  ({ship_id}, {k}, {v}),\n");
        });
        sql.pop();
        sql.pop();
        sql.push(';');
    //    dbg!(&sql);
        result.push(sql);
        result
    }
}
