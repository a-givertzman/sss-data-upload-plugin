//! Структура с данными для min_metacentric_height_subdivision
use crate::error::Error;
use crate::Table;


// Структура с данными для min_metacentric_height_subdivision
pub struct MinMetacentricHeightSubdivision {
    data: String,
    parsed: Vec<(String, String)>,
}
//
impl MinMetacentricHeightSubdivision {
    //
    pub fn new(data: String,) -> Self {
        Self {
            data,
            parsed: Vec::new(),
        }
    }
    //
    pub fn to_string(&self, ship_id: usize) -> String {
        let mut result = format!("DELETE FROM min_metacentric_height_subdivision WHERE ship_id={ship_id};\n\n");
        result += "INSERT INTO min_metacentric_height_subdivision\n  (ship_id, key, value)\nVALUES\n";
        self.parsed
            .iter()
            .for_each(|(key, value)| {
                result +=
                    &format!("  ({ship_id}, {key}, {value}),\n");
            });
        result.pop();
        result.pop();
        result.push(';');
        result
    } 
}
//
impl Table for MinMetacentricHeightSubdivision {
    //
    fn parse(&mut self) -> Result<(), Error> {
     //   dbg!(&self.data);
        println!("MinMetacentricHeightSubdivision parse begin");
        let mut data = crate::split_data(&self.data)?;
        data.remove(0);
        self.parsed = data
            .into_iter()
            .filter_map(|line| {
                if line.len() == 2 {
                    Some((
                        line[0].to_owned(), // Draft [m]
                        line[1].to_owned(), // Min. metacentric height due to subdivision index [m]
                    ))
                } else {
                    None
                }
            })
            .collect();
     //   dbg!(&self.parsed);
        println!("MinMetacentricHeightSubdivision parse ok");
        Ok(())
    }
    //
    fn to_sql(&self, id: usize) -> Vec<String> {
        vec![self.to_string(id)]
    }
    //
    fn to_file(&self, id: usize, name: &str) {
        std::fs::write(format!("../{name}/hidrostatic/min_metacentric_height_subdivision.sql"), self.to_string(id)).expect("Unable to write file min_metacentric_height_subdivision.sql");           
        std::thread::sleep(std::time::Duration::from_secs(1));  
    }
}
