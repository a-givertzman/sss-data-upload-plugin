//! Структура с данными для compartment_curve
use crate::error::Error;
use crate::Table;

/// Структура с данными для compartment_curve
pub struct CompartmentCurve {
    data: Vec<(String, String)>,
    parsed: Vec<(String, Vec<(f64, f64, f64, f64, f64, f64, f64)>)>,
}
//
impl CompartmentCurve {
    //
    pub fn new(data: Vec<(String,String)>,) -> Self {
        Self {
            data,
            parsed: Vec::new(),
        }
    }
    //
    fn to_string(&self, ship_id: usize, space_name: String, parsed_data: Vec<(f64, f64, f64, f64, f64, f64, f64)>) -> String {
        let mut result: String = format!("DELETE FROM compartment_curve WHERE space_id IN (SELECT space_id FROM compartment WHERE name_rus='{space_name}' AND ship_id={ship_id});\n\n");
        result += &format!("INSERT INTO compartment_curve\n  (ship_id, space_id, level, volume, buoyancy_x, buoyancy_y, buoyancy_z, trans_inertia_moment_self, long_inertia_moment_self)\n");
        parsed_data.iter().for_each(|(level, volume, buoyancy_x, buoyancy_y, buoyancy_z, long_inertia_moment_self, trans_inertia_moment_self)| {
            result += &format!("SELECT {ship_id}, (SELECT space_id FROM compartment WHERE name_rus='{space_name}' AND ship_id={ship_id}), {level}, {volume}, {buoyancy_x}, {buoyancy_y}, {buoyancy_z}, {trans_inertia_moment_self}, {long_inertia_moment_self} UNION ALL\n");
        });
        result = result.drain(..result.len()-11).collect();
        result += ";\n\n\n";   
     //   result += &format!("FROM compartment as c\nWHERE\n  c.space_id IN (SELECT space_id FROM compartment WHERE name_rus='{space_name}' AND ship_id={ship_id});\n\n");
        result
    }
}
//
impl Table for CompartmentCurve {
    ///
    fn parse(&mut self) -> Result<(), Error> {
        println!("CompartmentCurve parse begin");
        for (name, data) in self.data.iter() {
            let mut data = crate::split_data(data)?;
            data.remove(0);
            let mut data: Vec<(f64, f64, f64, f64, f64, f64, f64)> = data
                .into_iter()
                .filter_map(|line| {
                    if line.len() >= 7 {
                        Some((
                            line[0].to_owned().parse::<f64>().expect(&format!("CompartmentCurve parse error: name:{name} line:{:?}", line)), // level
                            line[1].to_owned().parse::<f64>().expect(&format!("CompartmentCurve parse error: name:{name} line:{:?}", line)), // volume
                            line[2].to_owned().parse::<f64>().expect(&format!("CompartmentCurve parse error: name:{name} line:{:?}", line)), // buoyancy_x
                            line[3].to_owned().parse::<f64>().expect(&format!("CompartmentCurve parse error: name:{name} line:{:?}", line)), // buoyancy_y
                            line[4].to_owned().parse::<f64>().expect(&format!("CompartmentCurve parse error: name:{name} line:{:?}", line)), // buoyancy_z
                            line[6].to_owned().parse::<f64>().expect(&format!("CompartmentCurve parse error: name:{name} line:{:?}", line)), // long_inertia_moment_self (y)
                            line[5].to_owned().parse::<f64>().expect(&format!("CompartmentCurve parse error: name:{name} line:{:?}", line)), // trans_inertia_moment_self (x)
                        ))
                    } else {
                        None
                    }
                })
                .collect();

                //replace zero in buoyancy
                if let Some((index, non_zero_line)) = data.iter().enumerate().filter(|(_, v)| v.2 != 0. || v.3 != 0. || v.4 != 0. ).next().map(|(i, v)| (i, (v.2, v.3, v.4))) {
                //    dbg!(name, index, non_zero_line);
                        (0..index).for_each(|i| {
                        data[i].2 = non_zero_line.0;
                        data[i].3 = non_zero_line.1;
                        data[i].4 = non_zero_line.2;
                    });
                }

                self.parsed.push((name.clone(), data));
            //  dbg!(&self.parsed);
        }
        println!("CompartmentCurve parse ok");
        Ok(())
    }
    ///
    fn to_sql(&self, ship_id: usize) -> Vec<String> {
        self.parsed.iter().map(|(name, data)| self.to_string(ship_id, name.to_owned(), data.to_vec())).collect()
    }
    //
    fn to_file(&self, ship_id: usize, name: &str) {
        let data: String = self.parsed.iter().map(|(name, data)| self.to_string(ship_id, name.to_owned(), data.to_vec())).collect();
        std::fs::write(format!("../{name}/loads/compartment_curve.sql"), data)
            .expect("Unable to write file compartment_curve.sql");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
