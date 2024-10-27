//! Структура с данными для compartment 
use crate::error::Error;
use crate::Table;
/// Структура с данными для compartment
pub struct Compartment {
    data: Option<String>,
    parsed: Vec<(String, String, String, String, String)>,
}
///
impl Compartment {
    ///
    pub fn new(data: String) -> Self {
        Self {
            data: Some(data),
            parsed: Vec::new(),
        }
    }
}
///
impl Table for Compartment {
    ///
    fn data(&mut self) -> Option<String> {
        self.data.take()
    }
    ///
    fn parse(&mut self) -> Result<(), Error> {
     //   dbg!(&self.data);
        let mut data = self.split_data()?;
        self.limit_area = Some(data.remove(0)[0].to_owned());
        data.remove(0);
        self.parsed = data
            .into_iter()
            .filter_map(|line| {
                if line.len() == 5 {
                    Some((
                        line[0].to_owned(), // frame_real index
                        line[1].to_owned(), // Bending moment min [kN?m]
                        line[2].to_owned(), // Bending moment max [kN?m]
                        line[3].to_owned(), // Shear force min [kN]
                        line[4].to_owned(), // Shear force max [kN]
                    ))
                } else {
                    None
                }
            })
            .collect();
     //   dbg!(&self.parsed);
        Ok(())
    }
    ///
    fn to_sql(&mut self, id: usize) -> Vec<String> {
    //    dbg!(&self.parsed);
        let mut result = Vec::new();
        result.push(format!("DELETE FROM strength_force_limit WHERE ship_id={id};"));
        let limit_area = self.limit_area.take().expect("Compartment to_sql limit_area error");
        let mut sql = "INSERT INTO strength_force_limit ( \
            ship_id, \
            frame_real_index, \
            value, \
            limit_type, \
            limit_area, \
            force_type  \
        ) VALUES".to_owned();
        self.parsed.iter_mut().for_each(|line| {
            let frame_real_index = &line.0;
            sql += &format!(" ({id}, {frame_real_index}, {}, 'low', {limit_area}, 'bending_moment'),", line.1);
            sql += &format!(" ({id}, {frame_real_index}, {}, 'high', {limit_area}, 'bending_moment'),", line.2);
            sql += &format!(" ({id}, {frame_real_index}, {}, 'low', {limit_area}, 'shear_force'),", line.3);
            sql += &format!(" ({id}, {frame_real_index}, {}, 'high', {limit_area}, 'shear_force'),", line.4);
        });
        sql.pop();
        sql.push(';');
    //    dbg!(&sql);
        result.push(sql);
        result
    }
}
