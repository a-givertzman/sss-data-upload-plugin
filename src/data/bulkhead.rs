//! Структура с данными для bulkhead
use crate::error::Error;
use crate::Table;


// Структура с данными для bulkhead
pub struct Bulkhead {
    data: String,
    parsed: Vec<(String, String, String)>,
}
//
impl Bulkhead {
    //
    pub fn new(data: String,) -> Self {
        Self {
            data,
            parsed: Vec::new(),
        }
    }
    //
    pub fn to_string(&self, ship_id: usize) -> String {
        let mut result = format!("DELETE FROM bulkhead WHERE ship_id={ship_id};\n\n");
        result += "INSERT INTO bulkhead\n  (ship_id, name_rus, name_engl, mass, category_id)\nVALUES\n";
        self.parsed
            .iter()
            .for_each(|(name_rus, name_engl, mass)| {
                result +=
                    &format!("  ({ship_id}, '{name_rus}', '{name_engl}', {mass}, 22),\n");
            });
        result.pop();
        result.pop();
        result.push(';');
        result
    } 
}
//
impl Table for Bulkhead {
    //
    fn parse(&mut self) -> Result<(), Error> {
     //   dbg!(&self.data);
        println!("Bulkhead parse begin");
        let mut data = crate::split_data(&self.data)?;
        data.remove(0);
        self.parsed = data
            .into_iter()
            .filter_map(|line| {
                if line.len() >= 3 {
                    Some((
                        line[0].to_owned(), // Name  RUS
                        line[1].to_owned(), // Name  ENGL
                        line[2].to_owned(), // Mass [t]
                    ))
                } else {
                    None
                }
            })
            .collect();
     //   dbg!(&self.parsed);
        println!("Bulkhead parse ok");
        Ok(())
    }
    //
    fn to_sql(&self, id: usize) -> Vec<String> {
        vec![self.to_string(id)]
    }
    //
    fn to_file(&self, id: usize, name: &str) {
        std::fs::write(format!("../{name}/hold/bulkhead.sql"), self.to_string(id)).expect("Unable to write file bulkhead.sql");           
        std::thread::sleep(std::time::Duration::from_secs(1));  
    }
}
