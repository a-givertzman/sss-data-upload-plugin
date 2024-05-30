//! Структура с данными для bonjean_frame
use crate::error::Error;
use crate::Table;

/// Структура с данными для bonjean_frame
pub struct BonjeanFrame {
    data: Option<String>,
    delta: f64,
    draft: Vec<f64>,
    pos_x: Vec<f64>,
    area: Vec<Vec<f64>>,
}
///
impl BonjeanFrame {
    ///
    pub fn new(data: String) -> Self {
        Self {
            data: Some(data),
            delta: 0.,
            draft: Vec::new(),
            pos_x: Vec::new(),
            area: Vec::new(),
        }
    }
}
///
impl Table for BonjeanFrame {
    ///
    fn data(&mut self) -> Option<String> {
        self.data.take()
    }
    ///
    fn parse(&mut self) -> Result<(), Error> {
     //   dbg!(&self.data);
        let mut data = self.split_data()?;
        self.delta = data.remove(0).pop().ok_or(Error::FromString(format!("BonjeanFrame parse delta error")))?.parse::<f64>()?;
        let mut draft = data.remove(0);
        draft.remove(0);
        self.draft = draft.into_iter().filter_map(|s| s.parse::<f64>().ok() ).collect::<Vec<f64>>();
        data.remove(0);
        for mut row in data.into_iter() {
     //       dbg!(&row);
            self.pos_x.push(row.pop().ok_or(Error::FromString(format!("BonjeanFrame parse pos_x error, string:{:?}", row)))?.parse::<f64>()?);
            if self.draft.len() != row.len() {
                return Err(Error::FromString(format!("BonjeanFrame parse error, draft.len() {} != row.len() {}", self.draft.len(), row.len())));
            }
            let mut row_values = Vec::new();
            for value in row {
                row_values.push(value.parse::<f64>()?);
            }
            self.area.push(row_values);
        }
    //    dbg!(&self.area);
        Ok(())
    }
    ///
    fn to_sql(&mut self, id: usize) -> Vec<String> {
        let mut result = Vec::new();
        let mut frame_sql = "INSERT INTO bonjean_frame (ship_id, frame_index, pos_x) VALUES".to_owned();
        self.pos_x.iter().enumerate().for_each(|(i, x)| {
            frame_sql += &format!(" ({}, {}, {}),", id, i, x*self.delta);
        });
        frame_sql.pop();
        frame_sql.push(';');
        result.push(frame_sql);

        self.pos_x.iter().enumerate().for_each(|(frame_index, _)| {
            let mut area_sql = "INSERT INTO frame_area (ship_id, frame_index, draft, area) VALUES".to_owned();
            self.area[frame_index].iter().enumerate().for_each(|(draft_index, area)| {
                let draft = self.draft[draft_index];
                area_sql += &format!(" ({id}, {frame_index}, {draft}, {area}),");
            } );  
            area_sql.pop();
            area_sql.push(';'); 
            result.push(area_sql);          
        });
        result
    }
}
