//! Структура с данными для draft_mark
use crate::error::Error;
use crate::Table;
use std::fs;
use std::thread::sleep;

/// Структура с данными для draft_mark
pub struct DraftMark {
    data: String,
    /// Координаты отметок заглубления на корпусе судна относительно центра, м
    /// Z | X | Y
    parsed: Vec<(String, String, String, String, String)>,
}
///
impl DraftMark {
    ///
    pub fn new(data: String) -> Self {
        Self {
            data,
            parsed: Vec::new(),
        }
    }
    //
    fn to_string(&self, ship_id: usize) -> String {
        let mut result = format!("DELETE FROM draft_mark WHERE ship_id={ship_id};\n\n");
        result += &format!("INSERT INTO draft_mark\n  (ship_id, mark_id, name, x, y, z)\nVALUES\n");
        self.parsed.iter().for_each(|(mark_id, name, x, y, z)| {
            result += &format!("  ({ship_id}, {mark_id}, '{name}', {x}, {y}, {z}),\n");
        });
        result.pop();
        result.pop();
        result.push(';');
        //    dbg!(&result);
        result
    }
}
///
impl Table for DraftMark {
    //
    fn parse(&mut self) -> Result<(), Error> {
        println!("DraftMark parse begin");
        //    dbg!(&self.data);
        let mut data = crate::split_data(&self.data)?;
        let mark_id: Vec<String> = data.remove(0).into_iter().filter(|s| s.len() > 0).collect();
        let names: Vec<String> = data.remove(0).into_iter().filter(|s| s.len() > 0).collect();
        data.remove(0);
        data.iter().for_each(|v| {
            mark_id
                .iter()
                .zip(names.iter())
                .enumerate()
                .for_each(|(i, (mark_id, name))| {
                    self.parsed.push((
                        mark_id.to_string(),
                        name.to_string(),
                        v[1 + i * 2].clone(),
                        v[2 + i * 2].clone(),
                        v[0].clone(),
                    ))
                });
        });
        //    dbg!(&self.mean_draught);
        println!("DraftMark parse ok");
        Ok(())
    }
    //
    fn to_sql(&self, id: usize) -> Vec<String> {
        let mut result = Vec::new();
        result.push(self.to_string(id));
        //    dbg!(&result);
        result
    }
    //
    fn to_file(&self, id: usize, name: &str) {
        fs::write(
            format!("../{name}/draft/draft_mark.sql"),
            self.to_string(id),
        )
        .expect("Unable to write file draft_mark.sql");
        sleep(std::time::Duration::from_secs(1));
    }
}
