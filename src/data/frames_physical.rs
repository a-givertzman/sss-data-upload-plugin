//! Структура с данными для physical_frame
use crate::error::Error;
use crate::Table;
/// Структура с данными для physical_frame
pub struct PhysicalFrame {
    data: String,
    parsed: Vec<(&str, &str)>,
}
///
impl PhysicalFrame {
    ///
    pub fn new(data: String) -> Self {
        Self {
            data,
            parsed: Vec::new(),
        }
    }
}
///
impl Table for PhysicalFrame {
    ///
    fn parse(&mut self) -> Result<(), Error> {
        let data = self.data.take().ok_or(Error::FromString(
            "PhysicalFrame data error: no data!".to_owned(),
        ))?;
        let pairs: Vec<Vec<&str>> = data
            .split("\r\n")
            .map(|line| line.split(';').collect() )
            .collect();
        pairs.into_iter().for_each(|mut pair| {
            self.parsed.push((
                pair.pop().expect("PhysicalFrame key error").to_owned(),
                pair.pop().expect("PhysicalFrame value error").to_owned(),
            ));
        });
        Ok(())
    }
    fn parse(&mut self) -> Result<(), Error> {
        let data = self
            .data
            .split("\r\n")
            .filter(|s| s.len() > 0)
            .collect::<Vec<&str>>();
        for line in data {
            let pair = line.split(';').collect::<Vec<&str>>();
            if pair.len() != 2 {
                return Err(Error::FromString(format!(
                    "PhysicalFrame key/value error: {line}"
                )));
            }
            if pair[0].len() == 0 {
                return Err(Error::FromString(format!(
                    "PhysicalFrame key error: {line}"
                )));
            }
            if pair[1].len() == 0 {
                return Err(Error::FromString(format!(
                    "PhysicalFrame value error: {line}"
                )));
            }
            self.parsed.push((pair[0], pair[1]));
        }
        Ok()
    }
    ///
    fn to_sql(&mut self, id: usize) -> String {
        let mut sql =
            "INSERT INTO physical_frame (ship_id, frame_index, pos_x) VALUES".to_owned();
        self.parsed.iter().for_each(|line| {
            sql += &format!("({}, {}, {}),", id, line.0, line.1);
        });
        sql.pop();
        sql.pop();
        sql.push(';');
        sql     
    }
}
