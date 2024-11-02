//! Структура с данными для compartment_curve для частей трюмов
use crate::error::Error;
use crate::Table;

/// Структура с данными для compartment_curve для частей трюмов
pub struct HoldCurve {
    data: Vec<(String, String)>,
    parsed: Vec<(
        String,
        Vec<(String, String, String, String, String, String)>,
    )>,
}
//
impl HoldCurve {
    //
    pub fn new(data: Vec<(String, String)>) -> Self {
        Self {
            data,
            parsed: Vec::new(),
        }
    }
    //
    fn to_compartment_curve(
        &self,
        ship_id: usize,
        code: String,
        parsed_data: Vec<(String, String, String, String, String, String)>,
    ) -> String {
        let mut result: String = format!("DELETE FROM compartment_curve WHERE space_id IN (SELECT space_id FROM hold_part WHERE code='{code}' AND ship_id={ship_id});\n\n");
        result += &format!("INSERT INTO compartment_curve\n  (ship_id, space_id, level, volume, buoyancy_x, buoyancy_y, buoyancy_z)\n");
        parsed_data.iter().for_each(|(level, volume, buoyancy_x, buoyancy_y, buoyancy_z, _)| {
            result += &format!("SELECT  {ship_id}, (SELECT space_id FROM hold_part_id WHERE code='{code}' AND ship_id={ship_id}), {level}, {volume}, {buoyancy_x}, {buoyancy_y}, {buoyancy_z} UNION ALL\n");
        });
        result = result.drain(..result.len() - 11).collect();
        result += ";\n\n\n";
        //   result += &format!("FROM compartment as c\nWHERE\n  c.space_id IN (SELECT space_id FROM compartment WHERE code='{code}' AND ship_id={ship_id});\n\n");
        result
    }

    //
    fn to_grain_moment(
        &self,
        ship_id: usize,
        code: String,
        parsed_data: Vec<(String, String, String, String, String, String)>,
    ) -> String {
        let mut result: String = format!("DELETE FROM grain_moment WHERE space_id IN (SELECT space_id FROM hold_part WHERE code='{code}' AND ship_id={ship_id});\n\n");
        result += &format!("INSERT INTO grain_moment\n  (ship_id, space_id, level, moment)\n");
        parsed_data.iter().for_each(|(level, _, _, _, _, moment)| {
            result += &format!("SELECT  {ship_id}, (SELECT space_id FROM hold_part_id WHERE code='{code}' AND ship_id={ship_id}), {level}, {moment} UNION ALL\n");
        });
        result = result.drain(..result.len() - 11).collect();
        result += ";\n\n\n";
        //   result += &format!("FROM compartment as c\nWHERE\n  c.space_id IN (SELECT space_id FROM compartment WHERE code='{code}' AND ship_id={ship_id});\n\n");
        result
    }
}
//
impl Table for HoldCurve {
    ///
    fn parse(&mut self) -> Result<(), Error> {
        println!("HoldCurve parse begin");
        for (code, data) in self.data.iter() {
            let mut data = crate::split_data(data)?;
            data.remove(0);
            let data = data
                .into_iter()
                .filter(|line| line[0].to_owned().parse::<f64>().is_ok())
                .map(|line| {
                    (
                        line[0].to_owned(), // Depth of cargo [m]
                        line[1].to_owned(), // Volume [m3]
                        line[2].to_owned(), // LCG  [m]
                        line[3].to_owned(), // TCG  [m]
                        line[4].to_owned(), // VCG  [m]
                        line[5].to_owned(), // Transvers Grain Moments [m4]
                    )
                })
                .collect();

            self.parsed.push((code.clone(), data));
            //  dbg!(&self.parsed);
        }
        println!("HoldCurve parse ok");
        Ok(())
    }
    //
    fn to_sql(&self, ship_id: usize) -> Vec<String> {
        self.parsed
            .iter()
            .map(|(code, data)| {
                self.to_compartment_curve(ship_id, code.to_owned(), data.to_vec())
                    + &self.to_grain_moment(ship_id, code.to_owned(), data.to_vec())
            })
            .collect()
    }
    //
    fn to_file(&self, ship_id: usize) {
        let data: String = self
            .parsed
            .iter()
            .map(|(code, data)| self.to_compartment_curve(ship_id, code.to_owned(), data.to_vec()))
            .collect();
        std::fs::write("hold_curve.sql", data).expect("Unable to write file hold_curve.sql");
        std::thread::sleep(std::time::Duration::from_secs(1));
        let data: String = self
            .parsed
            .iter()
            .map(|(code, data)| self.to_grain_moment(ship_id, code.to_owned(), data.to_vec()))
            .collect();
        std::fs::write("grain_moment.sql", data).expect("Unable to write file grain_moment.sql");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
