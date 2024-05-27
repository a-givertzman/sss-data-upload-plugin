//! Структура с данными для theoretical_frame
use crate::error::Error;
use crate::Table;
/// Структура с данными для theoretical_frame
pub struct TheoreticalFrame {
    data: Option<String>,
    parsed: Vec<(String, String)>,
}
///
impl TheoreticalFrame {
    ///
    pub fn new(data: String) -> Self {
        Self {
            data: Some(data),
            parsed: Vec::new(),
        }
    }
}
///
impl Table for TheoreticalFrame {
    ///
    fn parse(&mut self) -> Result<(), Error> {
        let data = self.data.take().ok_or(Error::FromString(
            "TheoreticalFrame data error: no data!".to_owned(),
        ))?;
        let pairs: Vec<Vec<&str>> = data
            .split("\r\n")
            .map(|line| line.split(';').collect() )
            .collect();
        pairs.into_iter().for_each(|mut pair| {
            self.parsed.push((
                pair.pop().expect("TheoreticalFrame key error").to_owned(),
                pair.pop().expect("TheoreticalFrame value error").to_owned(),
            ));
        });
        Ok(())
    }
    ///
    fn to_sql(&mut self, id: usize) -> String {
        let mut sql =
            "INSERT INTO theoretical_frame (ship_id, frame_index, pos_x) VALUES".to_owned();
        self.parsed.iter_mut().for_each(|line| {
            sql += &format!("({}, {}, {}),", id, line.0, line.1);
        });
        sql.pop();
        sql.pop();
        sql.push(';');
        sql
    }
}
