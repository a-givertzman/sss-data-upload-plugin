//! Структура с данными для hold_group
use crate::error::Error;
use crate::Table;


// Структура с данными для hold_group
pub struct HoldGroup {
    data: String,
    parsed: Vec<(String, String, String)>,
}
//
impl HoldGroup {
    //
    pub fn new(data: String,) -> Self {
        Self {
            data,
            parsed: Vec::new(),
        }
    }
    //
    pub fn to_string(&self, ship_id: usize) -> String {
        let mut result = format!("DELETE FROM hold_group WHERE ship_id={ship_id};\n\n");
        result += "INSERT INTO hold_group\n  (ship_id, space_id, name_rus, name_engl)\nVALUES\n";
        self.parsed
            .iter()
            .for_each(|(space_id, name_rus, name_engl)| {
                result +=
                    &format!("  ({ship_id}, {space_id}, '{name_rus}', '{name_engl}'),\n");
            });
        result.pop();
        result.pop();
        result.push(';');
        result
    } 
}
//
impl Table for HoldGroup {
    //
    fn parse(&mut self) -> Result<(), Error> {
     //   dbg!(&self.data);
        println!("HoldGroup parse begin");
        let mut data = crate::split_data(&self.data)?;
        data.remove(0);
        self.parsed = data
            .into_iter()
            .filter_map(|line| {
                if line.len() >= 4 {
                    Some((
                        line[1].to_owned(), // Code
                        line[2].to_owned(), // Name  RUS
                        line[3].to_owned(), // Name  ENGL
                    ))
                } else {
                    None
                }
            })
            .collect();
     //   dbg!(&self.parsed);
        println!("HoldGroup parse ok");
        Ok(())
    }
    //
    fn to_sql(&self, id: usize) -> Vec<String> {
        vec![self.to_string(id)]
    }
    //
    fn to_file(&self, id: usize) {
        std::fs::write("hold_group.sql", self.to_string(id)).expect("Unable to write file hold_group.sql");           
        std::thread::sleep(std::time::Duration::from_secs(1));  
    }
}
