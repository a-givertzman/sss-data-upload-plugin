//! Структура с данными для compartment_curve для частей трюмов
use crate::error::Error;
use crate::Table;

/// Структура с данными для compartment_curve для частей трюмов
pub struct HoldCurve {
    data: Vec<(String, String)>,
    parsed: Vec<(
        String,
        Vec<(f64, f64, f64, f64, f64, f64)>,
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
        parsed_data: Vec<(f64, f64, f64, f64, f64, f64)>,
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
        parsed_data: Vec<(f64, f64, f64, f64, f64, f64)>,
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
            let data: Vec<_> = data
                .into_iter()
                .filter(|line| line[0].to_owned().parse::<f64>().is_ok())
                .map(|line| {
                    (
                        line[0].trim().to_owned().parse::<f64>().expect(&format!("HoldCurve parse error: code:{code} line:{:?}", line)), // Depth of cargo [m]
                        line[1].trim().to_owned().parse::<f64>().expect(&format!("HoldCurve parse error: code:{code} line:{:?}", line)), // Volume [m3]
                        line[2].trim().to_owned().parse::<f64>().expect(&format!("HoldCurve parse error: code:{code} line:{:?}", line)), // LCG  [m]
                        line[3].trim().to_owned().parse::<f64>().expect(&format!("HoldCurve parse error: code:{code} line:{:?}", line)), // TCG  [m]
                        line[4].trim().to_owned().parse::<f64>().expect(&format!("HoldCurve parse error: code:{code} line:{:?}", line)), // VCG  [m]
                        line[5].trim().to_owned().parse::<f64>().expect(&format!("HoldCurve parse error: code:{code} line:{:?}", line)), // Transvers Grain Moments [m4]
                    )
                })
                .collect();
            // check values 
            if let Some(i) = (1..data.len()-1).filter(|&i| data[i].0 <= data[i-1].0 && data[i].1 < data[i-1].1 && data[i].4 < data[i-1].4 ).next() {
                let error = format!("HoldCurve parse error: wrong values: {:?}", data[i]);
                println!("{error}");
                return Err(Error::FromString(error));
            }
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
    fn to_file(&self, ship_id: usize, name: &str) {
        let data: String = self
            .parsed
            .iter()
            .map(|(code, data)| self.to_compartment_curve(ship_id, code.to_owned(), data.to_vec()))
            .collect();
        std::fs::write(format!("../{name}/hold/hold_curve.sql"), data).expect("Unable to write file hold_curve.sql");
        std::thread::sleep(std::time::Duration::from_secs(1));
        let data: String = self
            .parsed
            .iter()
            .map(|(code, data)| self.to_grain_moment(ship_id, code.to_owned(), data.to_vec()))
            .collect();
        std::fs::write(format!("../{name}/hold/grain_moment.sql"), data).expect("Unable to write file grain_moment.sql");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
