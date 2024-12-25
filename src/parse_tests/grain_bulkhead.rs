//! Структура с данными для трюмов
use crate::error::Error;
use crate::Table;

/// Структура с данными для трюмов
pub struct GrainBulkheads {
    data: Vec<Vec<String>>,
    /// Name  ENGL  GrainBulkheadsPlace
    parsed: Vec<(String, String)>,
}
//
impl GrainBulkheads {
    //
    pub fn new(data: Vec<Vec<String>>) -> Self {
        Self {
            data,
            parsed: Vec::new(),
        }
    }
    //
    fn to_string(&self, ship_id: usize) -> String {
        let mut result = String::new();
        self.parsed.iter().for_each(|(bulkhead_name, bulkhead_place_code)| {
            result += &format!("UPDATE bulkhead_place SET bulkhead_id=(SELECT id FROM bulkhead WHERE name_engl='{bulkhead_name}') WHERE ship_id={ship_id} AND code='{bulkhead_place_code}';\n");
        });
        result
    }
}
//
impl Table for GrainBulkheads {
    //
    fn parse(&mut self) -> Result<(), Error> {
        println!("GrainBulkheads parse begin");
        self.parsed = self.data.iter().map(|v|(v[0].clone(), v[1].clone())).collect();
        self.parsed.remove(0);
        println!("GrainBulkheads parse ok");
        //  dbg!(&self.parsed);
        Ok(())
    }
    //
    fn to_file(&self, ship_id: usize, name: &str) {
        std::fs::write(
            format!("../{name}/test/grain_bulkhead.sql"),
            self.to_string(ship_id),
        )
        .expect("Unable to write file /test/grain_bulkhead.sql");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    //
    fn to_sql(&self, ship_id: usize) -> Vec<String> {
        vec![self.to_string(ship_id)]
    }
}
