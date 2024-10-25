//! Структура с данными для strength_force_limit
use crate::error::Error;
use crate::Table;

/// Структура с данными для strength_force_limit
pub struct StrengthForceLimit {
    data: Option<String>,
    limit_area: Option<String>,
    parsed: Vec<(String, String, String, String, String)>,
}
///
impl StrengthForceLimit {
    ///
    pub fn new(data: String) -> Self {
        Self {
            data: Some(data),
            limit_area: None,
            parsed: Vec::new(),
        }
    }
}
///
impl Table for StrengthForceLimit {
    ///
    fn data(&mut self) -> Option<String> {
        self.data.take()
    }
    ///
    fn parse(&mut self) -> Result<(), Error> {
     //   dbg!(&self.data);
        println!("StrengthForceLimit parse begin");
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
        println!("StrengthForceLimit parse ok");
        Ok(())
    }
    ///
    fn to_sql(&self, id: usize) -> Vec<String> {
    //    dbg!(&self.parsed);
        let mut result = Vec::new();
        result.push(format!("DELETE FROM strength_force_limit WHERE ship_id={id};\n\n"));
        let limit_area = self.limit_area.as_ref().expect("StrengthForceLimit to_sql limit_area error").to_owned();
        let mut sql = "INSERT INTO strength_force_limit ( \
            ship_id, \
            frame_real_index, \
            value, \
            limit_type, \
            limit_area, \
            force_type  \
        ) VALUES\n".to_owned();
        self.parsed.iter().for_each(|line| {
            let frame_real_index = &line.0;
            sql += &format!(" ({id}, {frame_real_index}, {}, 'low', '{limit_area}', 'bending_moment'),\n", line.1);
            sql += &format!(" ({id}, {frame_real_index}, {}, 'high', '{limit_area}', 'bending_moment'),\n", line.2);
            sql += &format!(" ({id}, {frame_real_index}, {}, 'low', '{limit_area}', 'shear_force'),\n", line.3);
            sql += &format!(" ({id}, {frame_real_index}, {}, 'high', '{limit_area}', 'shear_force'),\n", line.4);
        });
        sql.pop();
        sql.pop();
        sql.push(';');
    //    dbg!(&sql);
        result.push(sql);
        result
    }
}
