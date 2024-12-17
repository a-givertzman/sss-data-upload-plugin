//! Структура с данными для bonjean_frame
use crate::error::Error;
use crate::Table;

/// Структура с данными для bonjean_frame
pub struct Pantocaren {
    data: Vec<Vec<String>>,
    /// Trim, m | T, м  | entry angle, deg | flooding angle, deg
    parsed: Vec<(f64, f64, f64, f64)>,
}
//
impl Pantocaren {
    //
    pub fn new(data: Vec<Vec<String>>) -> Self {
        Self {
            data,
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
impl Table for Pantocaren {
    //
    fn parse(&mut self) -> Result<(), Error> {
        println!("Pantocaren parse begin");
        let mut data = self.data.clone();
      //  dbg!(&data);
        let roll = data.remove(0);
     //   dbg!(&roll);
        data.remove(0);
        for row in data.into_iter() {
            let trim = row.get(0).ok_or(Error::FromString(
                "Pantocaren parse error: no trim".to_owned(),
            ))?.to_owned().parse::<f64>()?;
            let draft = row.get(1).ok_or(Error::FromString(
                "Pantocaren parse error: no draft".to_owned(),
            ))?.to_owned().parse::<f64>()?;
            for index in 2..roll.len() {
                let roll = roll.get(index).ok_or(Error::FromString(
                    format!("Pantocaren parse error: no roll for index {index}, roll: {:?}", roll),
                ))?.to_owned().parse::<f64>()?;
                let moment = row.get(index).ok_or(Error::FromString(
                    "Pantocaren parse error: no draft".to_owned(),
                ))?.to_owned().parse::<f64>()?;
                self.parsed.push((trim, draft, roll, moment));
            }
        };
      //  dbg!(&self.parsed);
        println!("Pantocaren parse ok");
        Ok(())
    }
    //
    fn to_sql(&self, id: usize) -> Vec<String> {
        vec![self.to_string(id)]
    }
    //
    fn to_file(&self, id: usize, name: &str) {
        std::fs::write(format!("../{name}/hidrostatic/pantocaren.sql"), self.to_string(id)).expect("Unable to write file pantocaren.sql");           
        std::thread::sleep(std::time::Duration::from_secs(1));  
    }
}