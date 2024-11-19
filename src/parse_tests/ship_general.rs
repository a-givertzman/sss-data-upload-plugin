//! Общие данные по судну
use std::collections::HashMap;
use crate::error::Error;
use crate::Table;

/// Общие данные по судну
pub struct ShipGeneral {
    data: Vec<Vec<String>>,
    /// name	value
    parsed: HashMap<String, String>,
}
//
impl ShipGeneral {
    //
    pub fn new(data: Vec<Vec<String>>) -> Self {
        Self {
            data,
            parsed: HashMap::new(), 
        }
    }
    //
    pub fn to_string(&self, ship_id: usize) -> String {
        let limit_area = self.parsed.get("Акватория").expect("Test ShipGeneral to_string no value for limit_area!");
        let limit_area = match limit_area.as_str() {
            "Море" => "sea",
            "Порт" => "harbor",
            text => panic!("{}", format!("Test ShipGeneral to_string limit_area wrong value:{text}!")),
        };
        let mut result= format!("UPDATE ship SET limit_area='{limit_area}' WHERE ship_id={ship_id};\n");

        let water_density = self.parsed.get("Плотность забортной воды [т/м^3]").expect("Test ShipGeneral to_string no value for water_density!");
        result += &format!("UPDATE ship_parameters SET value={water_density} WHERE key='Water Density' AND ship_id={ship_id};\n");
       
        let icing = self.parsed.get("Обледенение").expect("Test ShipGeneral to_string no value for icing!");
        result += &format!("UPDATE ship SET icing_type_id=(SELECT id FROM ship_icing WHERE icing_type='{icing}') WHERE ship_id={ship_id};\n"); 

        let wetting_timber = self.parsed.get("Намокание палубного лесного груза %").expect("Test ShipGeneral to_string no value for wetting_timber!");
        let wetting_timber = wetting_timber.parse::<f64>().map_or("NULL".to_owned(), |v| v.to_string() );        
        result += &format!("UPDATE ship_parameters SET value='{wetting_timber}' WHERE key='Wetting of deck timber' AND ship_id={ship_id};\n");

        result
    }
}
//
impl Table for ShipGeneral  {
    //
    fn parse(&mut self) -> Result<(), Error> {
        println!("ShipGeneral parse begin");
        self.parsed = self.data.iter().map(|v|(v[0].clone(), v[1].clone())).collect();
        println!("ShipGeneral parse ok");
        //  dbg!(&self.parsed);
        Ok(())
    }
    //
    fn to_sql(&self, id: usize) -> Vec<String> {
        vec![self.to_string(id)]
    }
    //
    fn to_file(&self, id: usize, name: &str) {
        std::fs::write(format!("../{name}/test/ship_general.sql"), self.to_string(id))
            .expect("Unable to write file /test/ship_general.sql");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
