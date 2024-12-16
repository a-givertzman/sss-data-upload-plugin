//! Структура с данными для strength_force_limit
use std::rc::Rc;

use crate::error::Error;
use crate::Table;

use super::PhysicalFrame;

/// Структура с данными для strength_force_limit
pub struct StrengthForceLimit {
    data: String,
    physical_frame: Rc<PhysicalFrame>,
    limit_area: Option<String>,
    parsed: Vec<(String, String, String, String, String)>,
}
//
impl StrengthForceLimit {
    //
    pub fn new(data: String, physical_frame: Rc<PhysicalFrame>) -> Self {
        Self {
            data,
            physical_frame,
            limit_area: None,
            parsed: Vec::new(),
        }
    }
    //
    fn to_string(&self, id: usize) -> String {
        //    dbg!(&self.parsed);
        let mut result = format!("DELETE FROM strength_force_limit WHERE ship_id={id};\n\n");
        let limit_area = self
            .limit_area
            .as_ref()
            .expect("StrengthForceLimit to_sql limit_area error")
            .to_owned();
        result += "INSERT INTO strength_force_limit\n  ( \
                ship_id, \
                frame_x, \
                value, \
                limit_type, \
                limit_area, \
                force_type  \
            )\nVALUES\n";
        self.parsed.iter().for_each(|line| {
            let frame_real_index = &line.0;
            result += &format!(
                "  ({id}, {frame_real_index}, {}, 'low', '{limit_area}', 'bending_moment'),\n",
                line.1
            );
            result += &format!(
                " ({id}, {frame_real_index}, {}, 'high', '{limit_area}', 'bending_moment'),\n",
                line.2
            );
            result += &format!(
                " ({id}, {frame_real_index}, {}, 'low', '{limit_area}', 'shear_force'),\n",
                line.3
            );
            result += &format!(
                " ({id}, {frame_real_index}, {}, 'high', '{limit_area}', 'shear_force'),\n",
                line.4
            );
        });
        result.pop();
        result.pop();
        result.push(';');
        //    dbg!(&sql);
        result
    }
}
//
impl Table for StrengthForceLimit {
    //
    fn parse(&mut self) -> Result<(), Error> {
        //   dbg!(&self.data);
        println!("StrengthForceLimit parse begin");
        let mut data = crate::split_data(&self.data)?;
        self.limit_area = Some(data.remove(0)[0].to_owned());
        data.remove(0);
        for line in data.into_iter() {
            self.parsed.push((
                self.physical_frame.value(&line[0])?.to_string(), // Fr
                line[1].trim().to_owned(),                               // Bending moment min [kN?m]
                line[2].trim().to_owned(),                               // Bending moment max [kN?m]
                line[3].trim().to_owned(),                               // Shear force min [kN]
                line[4].trim().to_owned(),                               // Shear force max [kN]
            ));
        }
        //   dbg!(&self.parsed);
        println!("StrengthForceLimit parse ok");
        Ok(())
    }
    //
    fn to_file(&self, id: usize, name: &str) {
        std::fs::write(
            format!("../{name}/strength_force_limit_{}.sql", self.limit_area.clone().unwrap_or("".to_string())),
            self.to_string(id),
        )
        .expect("Unable to write file strength_force_limit.sql");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    //
    fn to_sql(&self, id: usize) -> Vec<String> {
        vec![self.to_string(id)]
    }
}
