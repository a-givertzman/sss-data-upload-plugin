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
    fn data(&mut self) -> Option<String> {
        self.data.take()
    }
    ///
    fn parse(&mut self) -> Result<(), Error> {
    //    dbg!(&self.data);
        let data = self.split_data()?;
        data.into_iter().for_each(|mut row| {
            if let Ok(second) = row
                .pop()
                .expect("TheoreticalFrame value error")
                .parse::<i32>()
            {
                self.parsed.push((
                    row.pop().expect("TheoreticalFrame key error").to_owned(),
                    (second as f64 * 0.001).to_string(),
                ));
            }
        });
        Ok(())
    }
    ///
    fn to_sql(&self, id: usize) -> Vec<String> {
        let mut result = Vec::new();
        result.push(format!("DELETE FROM theoretical_frame WHERE ship_id={id};\n\n"));
        let mut sql =
            "INSERT INTO theoretical_frame\n  (ship_id, frame_index, pos_x)\nVALUES\n".to_owned();
        self.parsed.iter().for_each(|line| {
            sql += &format!(" ({}, {}, {}),", id, line.0, line.1);
        });
        sql.pop();
        sql.push(';');
        result.push(sql);
        result
    }
}
