//! Структура с данными для load_line и screw
use crate::error::Error;
use crate::Table;
use std::fs;
use std::thread::sleep;

/// Структура с данными для load_line и screw
pub struct LoadLine {
    data: String,
    /// Осадки
    /// id | name_rus | name_engl | X | Y | Z
    load_line: Vec<(String, String, String, String, String, String)>,
    /// Винты
    /// id | name_rus | name_engl | X | Y | Z | D
    screw: Vec<(String, String, String, String, String, String, String)>,
}
///
impl LoadLine {
    ///
    pub fn new(data: String) -> Self {
        Self {
            data,
            load_line: Vec::new(),
            screw: Vec::new(),
        }
    }
    //
    fn load_line(&self, ship_id: usize) -> String {
        let mut result = format!("DELETE FROM load_line WHERE ship_id={ship_id};\n\n");
        result += &format!("INSERT INTO load_line\n  (ship_id, parameter_id, name_rus, name_engl, x, y, z)\nVALUES\n");
        self.load_line
            .iter()
            .for_each(|(parameter_id, name_rus, name_engl, x, y, z)| {
                result += &format!(
                    "  ({ship_id}, {parameter_id}, '{name_rus}', '{name_engl}', {x}, {y}, {z}),\n"
                );
            });
        result.pop();
        result.pop();
        result.push(';');
        //    dbg!(&result);
        result
    }
    //
    fn screw(&self, ship_id: usize) -> String {
        let mut result = format!("DELETE FROM screw WHERE ship_id={ship_id};\n\n");
        result += &format!("INSERT INTO screw\n  (ship_id, parameter_id, name_rus, name_engl, x, y, z, d)\nVALUES\n");
        self.screw.iter().for_each(|(parameter_id, name_rus, name_engl, x, y, z, d)| {
            result += &format!("  ({ship_id}, {parameter_id}, '{name_rus}', '{name_engl}', {x}, {y}, {z}, {d}),\n");
        });
        result.pop();
        result.pop();
        result.push(';');
        //    dbg!(&result);
        result
    }
}
///
impl Table for LoadLine {
    //
    fn parse(&mut self) -> Result<(), Error> {
        println!("LoadLine parse begin");
        //    dbg!(&self.data);
        let mut data = crate::split_data(&self.data)?;
        data.remove(0);
        for row in data {
            if row.len() < 7 {
                return Err(Error::FromString(format!(
                    "LoadLine error: row.len() < 7, row:{:?}",
                    row
                )));
            }
            if row.last().unwrap().parse::<f64>().is_ok() {
                self.screw.push((
                    row[0].clone(),
                    row[1].clone(),
                    row[2].clone(),
                    row[3].clone(),
                    row[4].clone(),
                    row[5].clone(),
                    row[6].clone(),
                ));
            } else {
                self.load_line.push((
                    row[0].clone(),
                    row[1].clone(),
                    row[2].clone(),
                    row[3].clone(),
                    row[4].clone(),
                    row[5].clone(),
                ));
            }
        }
        println!("LoadLine parse ok");
        Ok(())
    }
    //
    fn to_sql(&self, id: usize) -> Vec<String> {
        vec![self.load_line(id), self.screw(id)]
    }
    //
    fn to_file(&self, id: usize, name: &str) {
        fs::write(
            format!("../{name}/draft/load_line.sql"),
            self.load_line(id),
        )
        .expect("Unable to write file load_line.sql");
        sleep(std::time::Duration::from_secs(1));
        fs::write(
            format!("../{name}/draft/screw.sql"),
            self.screw(id),
        )
        .expect("Unable to write file screw.sql");
        sleep(std::time::Duration::from_secs(1));
    }
}
