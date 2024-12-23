//! Структура с данными для hold_part
use crate::error::Error;
use crate::Table;


// Структура с данными для hold_part
pub struct HoldPart {
    data: Vec<Vec<String>>,
    parsed: Vec<(String, String, String, String, String, String, String, String)>,
}
//
impl HoldPart {
    //
    pub fn new(data: Vec<Vec<String>>,) -> Self {
        Self {
            data,
            parsed: Vec::new(),
        }
    }
    //
    pub fn hold_part(&self, ship_id: usize) -> String {
        let mut result = format!("DELETE FROM hold_part WHERE ship_id={ship_id};\n\n");
        result += "INSERT INTO hold_part\n  (ship_id, space_id, code, group_space_id, group_index, left_bulkhead_code, right_bulkhead_code, bound_x1, bound_x2)\nVALUES\n";
        self.parsed
            .iter()
            .for_each(|(space_id, code, group_space_id, group_index, left_bulkhead_code, right_bulkhead_code, bound_x1, bound_x2)| {
                result += &format!("  ({ship_id}, {space_id}, '{code}', {group_space_id}, {group_index}, ");
                if left_bulkhead_code.len() > 0 {
                    result += &format!(" '{left_bulkhead_code}',");
                } else {
                    result += " NULL,";
                }
                if right_bulkhead_code.len() > 0 {
                    result += &format!(" '{right_bulkhead_code}',");
                } else {
                    result += " NULL,";
                }
                result += &format!(" {bound_x1}, {bound_x2}),\n");
            });
        result.pop();
        result.pop();
        result.push(';');
        result
    } 
    //
    pub fn hold_part_id(&self, ship_id: usize) -> String {
        let mut result = format!("DELETE FROM hold_part_id WHERE ship_id={ship_id};\n\n");
        result += "INSERT INTO hold_part_id\n  (ship_id, space_id, code)\nVALUES\n";
        self.parsed
            .iter()
            .for_each(|(space_id, code, ..)| {
                result += &format!("  ({ship_id}, {space_id}, '{code}'),\n");
            });
        result.pop();
        result.pop();
        result.push(';');
        result
    } 
}
//
impl Table for HoldPart {
    //
    fn parse(&mut self) -> Result<(), Error> {
        println!("HoldPart parse begin");
        let mut data: Vec<Vec<String>> = self.data.clone().into_iter().filter(|s| s.len() >= 6).collect();
        data.remove(0);
        self.parsed = data
            .into_iter()
            .filter_map(|line| {
                if line.len() >= 9 {
                    let mut left_bulkhead_code = line[5].to_owned(); // left_hold
                    left_bulkhead_code.retain(|c| c.is_alphanumeric());
                    let mut right_bulkhead_code = line[6].to_owned(); // right_hold
                    right_bulkhead_code.retain(|c| c.is_alphanumeric());
                    Some((
                        line[1].to_owned(), // Space ID
                        line[2].to_owned(), // Code
                        line[3].to_owned(), // Parent Compartment Code
                        line[4].to_owned(), // group_index
                        left_bulkhead_code, 
                        right_bulkhead_code,
                        line[7].to_owned(), // X1 [m]
                        line[8].to_owned(), // X2 [m]
                    ))
                } else {
                    None
                }
            })
            .collect();
     //   dbg!(&self.parsed);
        println!("HoldPart parse ok");
        Ok(())
    }
    //
    fn to_sql(&self, id: usize) -> Vec<String> {
        vec![self.hold_part_id(id), self.hold_part(id)]
    }
    //
    fn to_file(&self, id: usize, name: &str) {
        std::fs::write(format!("../{name}/hold/hold_part_id.sql"), self.hold_part_id(id)).expect("Unable to write file hold_part_id.sql");           
        std::thread::sleep(std::time::Duration::from_secs(1));  
        std::fs::write(format!("../{name}/hold/hold_part.sql"), self.hold_part(id)).expect("Unable to write file hold_part.sql");           
        std::thread::sleep(std::time::Duration::from_secs(1));  
    }
}
