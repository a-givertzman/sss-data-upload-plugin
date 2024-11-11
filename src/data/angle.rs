//! Структура с данными для bonjean_frame
use crate::error::Error;
use crate::Table;

/// Структура с данными для bonjean_frame
pub struct Angle {
    data: String,
    /// Trim, m | T, м  | entry angle, deg | flooding angle, deg
    parsed: Vec<(f64, f64, f64, f64)>,
}
///
impl Angle {
    ///
    pub fn new(data: String) -> Self {
        Self {
            data,
            parsed: Vec::new(),
        }
    }
    //
    pub fn entry_angle(&self, ship_id: usize) -> String {
        let mut result = format!("DELETE FROM entry_angle WHERE ship_id={ship_id};\n\n");
        result += "INSERT INTO entry_angle\n  (ship_id, trim, draught, value)\nVALUES\n";
        self.parsed
            .iter()
            .for_each(|(trim, draught, value, _)| {
                result += &format!("  ({ship_id}, {trim}, {draught}, {value}),\n");
            });
        result.pop();
        result.pop();
        result.push(';');
        result
    }
    //
    pub fn flooding_angle(&self, ship_id: usize) -> String {
        let mut result = format!("DELETE FROM flooding_angle WHERE ship_id={ship_id};\n\n");
        result += "INSERT INTO flooding_angle\n  (ship_id, trim, draught, value)\nVALUES\n";
        self.parsed
            .iter()
            .for_each(|(trim, draught, _, value)| {
                result += &format!("  ({ship_id}, {trim}, {draught}, {value}),\n");
            });
        result.pop();
        result.pop();
        result.push(';');
        result
    }
}
//
impl Table for Angle {
    //
    fn parse(&mut self) -> Result<(), Error> {
        println!("Angle parse begin");
        let mut data = crate::split_data(&self.data)?;
        data.remove(0);
        for row in data.into_iter() {
            let trim = row
                .get(0)
                .ok_or(Error::FromString("Angle parse error: no trim".to_owned()))?
                .to_owned()
                .parse::<f64>()?;
            let draft = row
                .get(1)
                .ok_or(Error::FromString("Angle parse error: no draft".to_owned()))?
                .to_owned()
                .parse::<f64>()?;
            let entry_angle = row
                .get(2)
                .ok_or(Error::FromString(
                    "Angle parse error: no entry_angle".to_owned(),
                ))?
                .to_owned()
                .parse::<f64>()?;
            let flooding_angle = row
                .get(3)
                .ok_or(Error::FromString(
                    "Angle parse error: no flooding_angle".to_owned(),
                ))?
                .to_owned()
                .parse::<f64>()?;
            self.parsed.push((trim, draft, entry_angle, flooding_angle));
        }
        //  dbg!(&self.parsed);
        println!("Angle parse ok");
        Ok(())
    }
    //
    fn to_sql(&self, id: usize) -> Vec<String> {
        vec![self.entry_angle(id), self.flooding_angle(id)]
    }
    //
    fn to_file(&self, id: usize, name: &str) {
        std::fs::write(format!("../{name}/hidrostatic/entry_angle.sql"), self.entry_angle(id))
            .expect("Unable to write file entry_angle.sql");
        std::thread::sleep(std::time::Duration::from_secs(1));
        std::fs::write(format!("../{name}/hidrostatic/flooding_angle.sql"), self.flooding_angle(id))
            .expect("Unable to write file flooding_angle.sql");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
