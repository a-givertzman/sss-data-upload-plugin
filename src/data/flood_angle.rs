//! Структура с данными для flooding_angle
use crate::error::Error;
use crate::Table;

/// Структура с данными для flooding_angle
pub struct FloodAngle {
    data: Vec<Vec<String>>,
    /// Trim, m | T, м  | angle, deg 
    parsed: Vec<(String, String, String)>,
}
///
impl FloodAngle {
    ///
    pub fn new(data: Vec<Vec<String>>) -> Self {
        Self {
            data,
            parsed: Vec::new(),
        }
    }
    //
    pub fn to_string(&self, ship_id: usize) -> String {
        let mut result = format!("DELETE FROM flooding_angle WHERE ship_id={ship_id};\n\n");
        result += "INSERT INTO flooding_angle\n  (ship_id, trim, draught, value)\nVALUES\n";
        self.parsed
            .iter()
            .for_each(|(trim, draught, value)| {
                result += &format!("  ({ship_id}, {trim}, {draught}, {value}),\n");
            });
        result.pop();
        result.pop();
        result.push(';');
        result
    }
}
//
impl Table for FloodAngle {
    //
    fn parse(&mut self) -> Result<(), Error> {
        println!("FloodAngle parse begin");
        let mut data = self.data.clone();
        data.remove(0);
        data.into_iter().filter_map(|v| match (v.get(0), v.get(1), v.get(2)) {
            (Some(t), Some(d), Some(a)) => Some((t.to_owned(), d.to_owned(), a.to_owned())),
            _ => None,
        }).for_each(|v| self.parsed.push(v));
        //  dbg!(&self.parsed);
        println!("FloodAngle parse ok");
        Ok(())
    }
    //
    fn to_sql(&self, id: usize) -> Vec<String> {
        vec![self.to_string(id)]
    }
    //
    fn to_file(&self, id: usize, name: &str) {
        std::fs::write(format!("../{name}/hidrostatic/flooding_angle.sql"), self.to_string(id))
            .expect("Unable to write file flooding_angle.sql");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
