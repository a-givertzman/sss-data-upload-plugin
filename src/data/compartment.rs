//! Структура с данными для compartment 
use std::rc::Rc;
use crate::error::Error;
use crate::Table;
use super::PhysicalFrame;

/// Структура с данными для compartment
pub struct Compartment {
    data: String,
    physical_frame: Rc<PhysicalFrame>,
    parsed: Vec<(String, String, String, String, String, String, String, String)>,
}
//
impl Compartment {
    //
    pub fn new(data: String, physical_frame: Rc<PhysicalFrame>) -> Self {
        Self {
            data,
            physical_frame,
            parsed: Vec::new(),
        }
    }
    //
    fn to_string(&self, ship_id: usize) -> String {
        let mut result = format!("DELETE FROM compartment WHERE ship_id={ship_id};\n\n");
        result +=
            "INSERT INTO compartment\n  (ship_id, space_id, name_rus, ab_rus, name_engl, ab_engl, bound_x1, bound_x2, category_id)\nVALUES\n";
        self.parsed.iter().for_each(|(space_id, name_rus, ab_rus, name_engl, ab_engl, bound_x1, bound_x2, category_id)| {
            result += &format!("  ({ship_id}, {space_id}, '{name_rus}', '{ab_rus}', '{name_engl}', '{ab_engl}', {bound_x1}, {bound_x2}, {category_id}),\n");
        });
        result.pop();
        result.pop();
        result.push(';');
        result
    }
}
//
impl Table for Compartment {
    //
    fn parse(&mut self) -> Result<(), Error> {
        println!("Compartment parse begin");
        //    dbg!(&self.data);
        let mut data = crate::split_data(&self.data)?;
        data.remove(0);
        for row in data {
        //   dbg!(&row);
            let category_id = row.get(0).ok_or(Error::FromString(
                "Compartment error: no category_id in row".to_owned(),
            ))?.to_string();
            let space_id = row.get(1).ok_or(Error::FromString(
                "Compartment error: no space_id in row".to_owned(),
            ))?.to_string();
            let name_rus = row.get(2).ok_or(Error::FromString(
                "Compartment error: no space_id in row".to_owned(),
            ))?.to_string();
            let ab_rus = row.get(3).ok_or(Error::FromString(
                "Compartment error: no space_id in row".to_owned(),
            ))?.to_string();
            let name_engl = row.get(4).ok_or(Error::FromString(
                "Compartment error: no space_id in row".to_owned(),
            ))?.to_string(); 
            let ab_engl = row.get(5).ok_or(Error::FromString(
                "Compartment error: no space_id in row".to_owned(),
            ))?.to_string(); 
            let fr_min = row.get(6).ok_or(Error::FromString(
                "Compartment error: no space_id in row".to_owned(),
            ))?;
            let fr_max = row.get(7).ok_or(Error::FromString(
                "Compartment error: no space_id in row".to_owned(),
            ))?;
            let bound_x1 = self.physical_frame.value(fr_min)?.to_string();
            let bound_x2 = self.physical_frame.value(fr_max)?.to_string();
            self.parsed.push((space_id, name_rus, ab_rus, name_engl, ab_engl, bound_x1, bound_x2, category_id));
        }
        //  dbg!(&self.parsed);
        println!("Compartment parse ok");
        Ok(())
    }
    //
    fn to_file(&self, ship_id: usize, name: &str) {
        std::fs::write(format!("../{name}/loads/load_base.sql"), self.to_string(ship_id))
            .expect("Unable to write file compartment.sql");
        std::thread::sleep(std::time::Duration::from_secs(1));  
    }
    //
    fn to_sql(&self, ship_id: usize) -> Vec<String> {
        vec![self.to_string(ship_id)]
    }
}
