//! Структура с данными для compartment_curve
use calamine::{Data, DataType, Range};

use crate::error::Error;
use crate::Table;

/// Структура с данными для compartment_curve
pub struct CompartmentCurve {
    data: Vec<(String, Range<Data>)>,
    parsed: Vec<(String, Vec<(f64, f64, f64, f64, f64, f64, f64)>)>,
}
//
impl CompartmentCurve {
    //
    pub fn new(data: Vec<(String, Range<Data>)>) -> Self {
        Self {
            data,
            parsed: Vec::new(),
        }
    }
    //
    fn to_string(
        &self,
        ship_id: usize,
        space_id: String,
        parsed_data: Vec<(f64, f64, f64, f64, f64, f64, f64)>,
    ) -> String {
        let mut result: String = format!(
            "DELETE FROM compartment_curve WHERE space_id='{space_id}' AND ship_id={ship_id};\n\n"
        );
        result += &format!("INSERT INTO compartment_curve\n  (ship_id, space_id, level, volume, buoyancy_x, buoyancy_y, buoyancy_z, trans_inertia_moment_self, long_inertia_moment_self)\nVALUES\n");
        parsed_data.iter().for_each(|(level, volume, buoyancy_x, buoyancy_y, buoyancy_z, long_inertia_moment_self, trans_inertia_moment_self)| {
            result += &format!("  ({ship_id}, '{space_id}', {level}, {volume}, {buoyancy_x}, {buoyancy_y}, {buoyancy_z}, {trans_inertia_moment_self}, {long_inertia_moment_self}),\n");
        });
        result.pop();
        result.pop();
        result.push_str(";\n\n"); 
        //   result += &format!("FROM compartment as c\nWHERE\n  c.space_id IN (SELECT space_id FROM compartment WHERE name_rus='{space_name}' AND ship_id={ship_id});\n\n");
        result
    }
}
//
impl Table for CompartmentCurve {
    ///
    fn parse(&mut self) -> Result<(), Error> {
        println!("CompartmentCurve parse begin");
        for (code, data) in self.data.iter() {
            if !code.contains("(") || !code.contains(")") {
                continue;
            }
            let code = code.clone();
            let space_id = code
                .split("(")
                .collect::<Vec<_>>()
                .last()
                .copied()
                .ok_or(format!(
                    "CompartmentCurve parse code error: no ( code:{}",
                    code
                ))?
                .split(")")
                .collect::<Vec<_>>()
                .first()
                .copied()
                .ok_or(format!(
                    "CompartmentCurve parse code error: no ) code:{}",
                    code
                ))?;
            let mut data: Vec<&[Data]> = data.rows().filter(|v| !v.is_empty()).collect();
            data.remove(0);
            let mut parsed_data = Vec::new();
            let parse = |line: &[Data], i: usize| -> Result<f64, Error> {
                line.get(i)
                    .ok_or(format!(
                        "CompartmentCurve parse data error: code:{code} line:{:?}",
                        line
                    ))?
                    .as_f64()
                    .ok_or(format!("CompartmentCurve parse data error:  code:{code} line:{:?}", line).into())
            };
            for line in data {
                if line.len() < 7 {
                    dbg!(code, line);
                    panic!("CompartmentCurve line.len() < 7");
                }
                let line = (
                    parse(line, 0)?,
                    parse(line, 1)?,
                    parse(line, 2)?,
                    parse(line, 3)?,
                    parse(line, 4)?,
                    parse(line, 6)?,
                    parse(line, 5)?,
                );
                parsed_data.push(line);
            }
            //replace zero in buoyancy
            if let Some((index, non_zero_line)) = parsed_data
                .iter()
                .enumerate()
                .filter(|(_, v)| v.2 != 0. || v.3 != 0. || v.4 != 0.)
                .next()
                .map(|(i, v)| (i, (v.2, v.3, v.4)))
            {
                //    dbg!(name, index, non_zero_line);
                (0..index).for_each(|i| {
                    parsed_data[i].2 = non_zero_line.0;
                    parsed_data[i].3 = non_zero_line.1;
                    parsed_data[i].4 = non_zero_line.2;
                });
            }
            // check values
            if let Some(i) = (1..parsed_data.len() - 1)
                .filter(|&i| {
                    parsed_data[i].0 <= parsed_data[i - 1].0
                        && parsed_data[i].1 < parsed_data[i - 1].1
                        && parsed_data[i].4 < parsed_data[i - 1].4
                })
                .next()
            {
                let error = format!(
                    "CompartmentCurve parse error: wrong values: {:?}",
                    parsed_data[i]
                );
                println!("{error}");
                return Err(Error::FromString(error));
            }
            self.parsed.push((space_id.to_string(), parsed_data));
            //  dbg!(&self.parsed);
        }
        println!("CompartmentCurve parse ok");
        Ok(())
    }
    ///
    fn to_sql(&self, ship_id: usize) -> Vec<String> {
        self.parsed
            .iter()
            .map(|(name, data)| self.to_string(ship_id, name.to_owned(), data.to_vec()))
            .collect()
    }
    //
    fn to_file(&self, ship_id: usize, name: &str) {
        let data: String = self
            .parsed
            .iter()
            .map(|(name, data)| self.to_string(ship_id, name.to_owned(), data.to_vec()))
            .collect();
        std::fs::write(format!("../{name}/loads/compartment_curve.sql"), data)
            .expect("Unable to write file compartment_curve.sql");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
