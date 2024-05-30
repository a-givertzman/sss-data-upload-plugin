//! Структура с данными для load_constant
use crate::error::Error;
use crate::Table;

/// Структура с данными для load_constant
pub struct LoadConstant {
    data: Option<String>,
    parsed: Vec<(String, String, String)>,
}
///
impl LoadConstant {
    ///
    pub fn new(data: String) -> Self {
        Self {
            data: Some(data),
            parsed: Vec::new(),
        }
    }
}
///
impl Table for LoadConstant {
    ///
    fn data(&mut self) -> Option<String> {
        self.data.take()
    }
    ///
    fn parse(&mut self) -> Result<(), Error> {
    //    dbg!(&self.data);
        let mut data = self.split_data()?;
        data.remove(0);
        let mut parsed: Vec<(String, String, String)> = data
            .into_iter()
            .filter_map(|line| {
                if line.len() == 3 {
                    Some((line[0].to_owned(), line[1].to_owned(), line[2].to_owned(),))
                } else {
                    None
                }
            })
            .collect();
        self.parsed.append(&mut parsed);
     //   dbg!(&self.parsed);
        Ok(())
    }
    ///
    fn to_sql(&mut self, id: usize) -> Vec<String> {
   //     dbg!(&self.parsed);
        let mut sql = "INSERT INTO load_constant (ship_id, frame_start_index, frame_end_index, mass) VALUES".to_owned();
        self.parsed.iter_mut().for_each(|line| {
            sql += &format!(" ({}, {}, {}, {}),", id, line.0, line.1, line.2);
        });
        sql.pop();
        sql.push(';');
        vec![sql]
    }
}
