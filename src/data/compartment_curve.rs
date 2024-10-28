//! Структура с данными для compartment_curve
use crate::error::Error;
use crate::Table;

/// Структура с данными для compartment_curve
pub struct CompartmentCurve {
    space_name: String,
    data: Option<String>,
    parsed: Vec<(String, String, String, String, String, String, String)>,
}
//
impl CompartmentCurve {
    //
    pub fn new(space_name: String, data: String) -> Self {
        Self {
            space_name,
            data: Some(data),
            parsed: Vec::new(),
        }
    }
    //
    fn to_string(&self, ship_id: usize) -> String {
        let mut result = format!("DELETE FROM compartment_curve WHERE space_id IN (SELECT space_id FROM compartment WHERE name='{}' AND ship_id={});\n\n", self.space_name, ship_id);
        result += &format!("INSERT INTO compartment_curve\n
          (ship_id, space_id, level, volume, buoyancy_x, buoyancy_y, buoyancy_z, trans_inertia_moment_self, long_inertia_moment_self)\nSELECT\n");
        self.parsed.iter().for_each(|(level, volume, buoyancy_x, buoyancy_y, buoyancy_z, long_inertia_moment_self, trans_inertia_moment_self)| {
            result += &format!("  {ship_id}, c.space_id, {level}, {volume}, {buoyancy_x}, {buoyancy_y}, {buoyancy_z}, {trans_inertia_moment_self}, {long_inertia_moment_self},\n");
        });
        result += &format!("FROM compartment as c\nWHERE\n  c.space_id IN (SELECT space_id FROM compartment WHERE name='{}' AND ship_id={};\n", self.space_name, ship_id);
        result
    }
}
//
impl Table for CompartmentCurve {
    ///
    fn data(&mut self) -> Option<String> {
        self.data.take()
    }
    ///
    fn parse(&mut self) -> Result<(), Error> {
        println!("CompartmentCurve parse begin");
        let mut data = self.split_data()?;
        data.remove(0);
        self.parsed = data
            .into_iter()
            .filter_map(|line| {
                if line.len() == 5 {
                    Some((
                        line[0].to_owned(), // level
                        line[1].to_owned(), // volume
                        line[2].to_owned(), // buoyancy_x
                        line[3].to_owned(), // buoyancy_y
                        line[4].to_owned(), // buoyancy_z
                        line[5].to_owned(), // long_inertia_moment_self
                        line[6].to_owned(), // trans_inertia_moment_self
                    ))
                } else {
                    None
                }
            })
            .collect();
        //  dbg!(&self.parsed);
        println!("CompartmentCurve parse ok");
        Ok(())
    }
    ///
    fn to_sql(&self, ship_id: usize) -> Vec<String> {
        vec![self.to_string(ship_id)]
    }
    //
    fn to_file(&self, ship_id: usize) {
        std::fs::write("compartment_curve.sql", self.to_string(ship_id))
            .expect("Unable to write file compartment_curve.sql");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
