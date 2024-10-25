//! Структура с данными для bonjean_frame
use crate::error::Error;
use crate::Table;

/// Структура с данными для bonjean_frame
pub struct Pantokaren {
    data: Option<String>,
    /// Trim, m | T, м  | entry angle, deg | flooding angle, deg
    parsed: Vec<(f64, f64, f64, f64)>,
}
//
impl Pantokaren {
    //
    pub fn new(data: String) -> Self {
        Self {
            data: Some(data),
            parsed: Vec::new(),
        }
    }
    //
    pub fn to_string(&self, ship_id: usize) -> String {
        let mut result = format!("DELETE FROM pantocaren WHERE ship_id={ship_id};\n\n");
        result += "INSERT INTO pantocaren\n  (ship_id, trim, draught, roll, moment)\nVALUES\n";
        self.parsed
            .iter()
            .for_each(|(trim, draught, roll, moment)| {
                result +=
                    &format!("  ({ship_id}, {trim}, {draught}, {roll}, {moment}),\n");
            });
        result.pop();
        result.pop();
        result.push(';');
        result
    }   
}
//
impl Table for Pantokaren {
    //
    fn data(&mut self) -> Option<String> {
        self.data.take()
    }
    //
    fn parse(&mut self) -> Result<(), Error> {
        println!("Pantokaren parse begin");
    //    dbg!(&self.data);
        let mut data = self.split_data()?;
    //    dbg!(&data);
        let roll = data.remove(0);
    //    dbg!(&roll);
        data.remove(0);
        for row in data.into_iter() {
            let trim = row.get(0).ok_or(Error::FromString(
                "Pantokaren parse error: no trim".to_owned(),
            ))?.to_owned().parse::<f64>()?;
            let draft = row.get(1).ok_or(Error::FromString(
                "Pantokaren parse error: no draft".to_owned(),
            ))?.to_owned().parse::<f64>()?;
            for index in 2..roll.len() {
                let roll = roll.get(index).ok_or(Error::FromString(
                    format!("Pantokaren parse error: no roll for index {index}, roll: {:?}", roll),
                ))?.to_owned().parse::<f64>()?;
                let moment = row.get(index).ok_or(Error::FromString(
                    "Pantokaren parse error: no draft".to_owned(),
                ))?.to_owned().parse::<f64>()?;
                self.parsed.push((trim, draft, roll, moment));
            }
        };
      //  dbg!(&self.parsed);
        println!("Pantokaren parse ok");
        Ok(())
    }
    //
    fn to_sql(&self, id: usize) -> Vec<String> {
        vec![self.to_string(id)]
    }
    //
    fn to_file(&self, id: usize) {
        std::fs::write("pantokaren.sql", self.to_string(id)).expect("Unable to write file pantokaren.sql");           
        std::thread::sleep(std::time::Duration::from_secs(1));  
    }
}