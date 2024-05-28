//! Структура с данными для physical_frame
use crate::error::Error;
use crate::Table;

/// Структура с данными для physical_frame
pub struct PhysicalFrame {
    data: Option<String>,
    parsed: Vec<(String, String)>,
}
///
impl PhysicalFrame {
    ///
    pub fn new(data: String) -> Self {
        Self {
            data: Some(data),
            parsed: Vec::new(),
        }
    }
}
///
impl Table for PhysicalFrame {
    ///
    fn parse(&mut self) -> Result<(), Error> {
        dbg!(&self.data);
        let data = self.data.take().ok_or(Error::FromString(
            "PhysicalFrame data error: no data!".to_owned(),
        ))?;
        let data = data
            .replace(" ", "");
        let pairs: Vec<Vec<&str>> = data
            .split("\r\n")
            .filter_map(|line| { if line.len() > 2 {
                Some(line.split(';').collect())
        } else {
            None
        } } ).collect();
        pairs.into_iter().for_each(|mut pair| {
            dbg!(&pair);
            if let Ok(second) = pair.pop().expect("PhysicalFrame value error").parse::<i32>() {
                self.parsed.push((
                    pair.pop().expect("PhysicalFrame key error").to_owned(),
                    (second as f64 * 0.001).to_string(),
                ));
            }
        });
        dbg!(&self.parsed);
        Ok(())
    }
    ///
    fn to_sql(&mut self, id: usize) -> String {
        dbg!(&self.parsed);
        let mut sql =
            "INSERT INTO physical_frame (ship_id, frame_index, pos_x) VALUES".to_owned();
        self.parsed.iter_mut().for_each(|line| {
            sql += &format!(" ({}, {}, {}),", id, line.0, line.1);
        });
        sql.pop();
        sql.push(';');
        sql
    }
}
