//! Структура с данными для bonjean_frame
use crate::error::Error;
use crate::Table;
use std::fs;

/// Структура с данными для bonjean_frame
pub struct BonjeanFrame {
    data: Option<String>,
    draft: Vec<f64>,
    pos_x: Vec<f64>,
    area: Vec<Vec<f64>>,
}
//
impl BonjeanFrame {
    //
    pub fn new(data: String) -> Self {
        Self {
            data: Some(data),
            draft: Vec::new(),
            pos_x: Vec::new(),
            area: Vec::new(),
        }
    }
    //
    pub fn bonjean_frame(&self, id: usize) -> String {
        let mut result = format!("DELETE FROM bonjean_frame WHERE ship_id={id};\n\n");
        result += "INSERT INTO bonjean_frame\n  (ship_id, frame_index, pos_x)\nVALUES\n";
        self.pos_x.iter().enumerate().for_each(|(i, x)| {
            result += &format!("  ({}, {}, {}),\n", id, i, x);
        });
        result.pop();
        result.pop();
        result.push(';');
        result
    }
    //
    pub fn frame_area(&self, id: usize) -> String {
        let mut result = format!("DELETE FROM frame_area WHERE ship_id={id};\n\n");
        self.pos_x.iter().enumerate().for_each(|(frame_index, _)| {
            result += "INSERT INTO frame_area\n  (ship_id, frame_index, draft, area)\nVALUES\n";
            self.area[frame_index].iter().enumerate().for_each(|(draft_index, area)| {
                let draft = self.draft[draft_index];
                result += &format!("  ({id}, {frame_index}, {draft}, {area}),\n");
            } );  
            result.pop();
            result.pop();
            result.push(';');     
        });
        result
    }
}
//
impl Table for BonjeanFrame {
    ///
    fn data(&mut self) -> Option<String> {
        self.data.take()
    }
    ///
    fn parse(&mut self) -> Result<(), Error> {
     //   dbg!(&self.data);
        let mut data = self.split_data()?;
        let delta = data.remove(0).pop().ok_or(Error::FromString(format!("BonjeanFrame parse delta error")))?.parse::<f64>()?;
        let mut draft = data.remove(0);
        draft.remove(0);
        self.draft = draft.into_iter().filter_map(|s| s.parse::<f64>().ok() ).collect::<Vec<f64>>();
        data.remove(0);
        for mut row in data.into_iter() {
       //     dbg!(&row);
            self.pos_x.push(row.remove(0).parse::<f64>()?*delta);
            if self.draft.len() != row.len() {
                return Err(Error::FromString(format!("BonjeanFrame parse error, draft.len() {} != row.len() {}", self.draft.len(), row.len())));
            }
            let mut row_values = Vec::new();
            for value in row {
                row_values.push(value.parse::<f64>()?);
            }
            self.area.push(row_values);
        }
    //    dbg!(&self.pos_x);
    //    dbg!(&self.draft);
    //    dbg!(&self.area);
        Ok(())
    }
    //
    fn to_file(&self, id: usize) {
        fs::write("bonjean_frame.sql", self.bonjean_frame(id)).expect("Unable to write file bonjean_frame.sql"); 
        std::thread::sleep(std::time::Duration::from_secs(1));     
        fs::write("frame_area.sql", self.frame_area(id)).expect("Unable to write file frame_area.sql");    
        std::thread::sleep(std::time::Duration::from_secs(10));      
    }
    //
    fn to_sql(&self, id: usize) -> Vec<String> {
        vec![self.bonjean_frame(id), self.frame_area(id)]       
    }
}
