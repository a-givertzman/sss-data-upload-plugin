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
    fn data(&mut self) -> Option<String> {
        self.data.take()
    }
    ///
    fn parse(&mut self) -> Result<(), Error> {
    //    dbg!(&self.data);
        let data = self.split_data()?;
        data.into_iter().for_each(|mut row| {
     //       dbg!(&row);
            if let Ok(second) = row
                .pop()
                .expect("PhysicalFrame value error")
                .parse::<i32>()
            {
                self.parsed.push((
                    row.pop().expect("PhysicalFrame key error").to_owned(),
                    (second as f64 * 0.001).to_string(),
                ));
            }
        });
      //  dbg!(&self.parsed);
        Ok(())
    }
    ///
    fn to_sql(&mut self, id: usize) -> Vec<String> {
   //     dbg!(&self.parsed);
        let mut result = Vec::new();
        result.push(format!("DELETE FROM physical_frame WHERE ship_id={id};"));
        let mut sql = "INSERT INTO physical_frame (ship_id, frame_index, pos_x) VALUES".to_owned();
        self.parsed.iter_mut().for_each(|line| {
            sql += &format!(" ({}, {}, {}),", id, line.0, line.1);
        });
        sql.pop();
        sql.push(';');
        result.push(sql);
        result
    }
}
