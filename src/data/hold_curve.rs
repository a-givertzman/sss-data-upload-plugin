//! Структура с данными для compartment_curve для частей трюмов
use calamine::{Data, DataType, Range};

use crate::error::Error;
use crate::Table;

/// Структура с данными для compartment_curve для частей трюмов
pub struct HoldCurve {
    data: Vec<(String, Range<Data>)>,
    parsed: Vec<(
        String,
        Vec<(f64, f64, f64, f64, f64, f64)>,
    )>,
}
//
impl HoldCurve {
    //
    pub fn new(data: Vec<(String, Range<Data>)>) -> Self {
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
            let code = code.clone();
            let code = if !code.contains("(") && !code.contains(")") {
                code
            } else {
                code
                .split("(")
                .collect::<Vec<_>>()
                .last()
                .copied()
                .ok_or(format!("HoldCurve parse code error: no ( code:{}", code))?
                .split(")")
                .collect::<Vec<_>>()
                .first()
                .copied()
                .ok_or(format!("HoldCurve parse code error: no ) code:{}", code))?.to_owned()
            };
            let mut data: Vec<&[Data]> = data.rows().filter(|v| !v.is_empty()).collect();
            data.remove(0);
            let mut parsed_data = Vec::new();
            let parse = |line: &[Data], i: usize| -> Result<f64, Error> {
                line.get(i)
                    .ok_or(format!(
                        "HoldCurve parse data error:  code:{code} line:{:?}",
                        line
                    ))?
                    .as_f64()
                    .ok_or(format!("HoldCurve parse data error:  code:{code} line:{:?}", line).into())
            };
            for line in data {
                if line.len() < 6 {
                    dbg!(code, line);
                    panic!("HoldCurve line.len() < 6");
                }
                let line = (
                    parse(line, 0)?,
                    parse(line, 1)?,
                    parse(line, 2)?,
                    parse(line, 3)?,
                    parse(line, 4)?,
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
            if let Some(i) = (1..parsed_data.len()-1).filter(|&i| parsed_data[i].0 <= parsed_data[i-1].0 && parsed_data[i].1 < parsed_data[i-1].1 && parsed_data[i].4 < parsed_data[i-1].4 ).next() {
                let error = format!("HoldCurve parse error: wrong values: {:?}", parsed_data[i]);
                println!("{error}");
                return Err(Error::FromString(error));
            }

            self.parsed.push((code.to_string(), parsed_data));
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
