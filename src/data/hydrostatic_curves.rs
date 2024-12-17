//! Структура с данными для bonjean_frame
use crate::error::Error;
use crate::Table;
use std::fs;
use std::thread::sleep;

/// Структура с данными для bonjean_frame
pub struct HydrostaticCurves {
    data: Vec<Vec<String>>,
    /// Средняя осадка, зависимость от объемного водоизмещения,
    /// Trim, m | V, м3 | T, м  ship_id, key, value
    mean_draught: Vec<(f64, f64, f64)>,
    /// Отстояние центра величины погруженной части судна в
    /// зависимости от объемного водоизмещения,
    ///  Trim, m | V, м3 | Xc, м | Yc, м, | Zc, м  ship_id, key, value_x, value_y, value_z)
    center_draught: Vec<(f64, f64, f64, f64, f64)>,
    /// Площадь ватерлинии,
    ///  Trim, m | V, м3 | S, м^2  ship_id, key, value
    waterline_area: Vec<(f64, f64, f64)>,
    /// Абсцисса центра тяжести площади ватерлинии,
    ///  Trim, m | V, м3 | Xf, м  ship_id, key, value
    center_waterline: Vec<(f64, f64, f64)>,
    /// Ширина судна по ватерлинии в зависимости от осадки
    /// Trim, m | d, м | z, м  ship_id, key, value
    waterline_breadth: Vec<(f64, f64, f64)>,
    /// Поперечный метацентрический радиус в зависимости от объемного водоизмещения
    /// Trim, m | V, м3 | ro, м  
    rad_trans: Vec<(f64, f64, f64)>,
    /// Продольный метацентрический радиус в зависимости от объемного водоизмещения
    ///  Trim, m | V, м3 | Ro, м
    rad_long: Vec<(f64, f64, f64)>,
}
///
impl HydrostaticCurves {
    ///
    pub fn new(data: Vec<Vec<String>>) -> Self {
        Self {
            data,
            mean_draught: Vec::new(),
            center_draught: Vec::new(),
            waterline_area: Vec::new(),
            center_waterline: Vec::new(),
            waterline_breadth: Vec::new(),
            rad_trans: Vec::new(),
            rad_long: Vec::new(),
        }
    }
    //
    pub fn center_draught(&self, ship_id: usize) -> String {
        let mut result = format!("DELETE FROM center_draught WHERE ship_id={ship_id};\n\n");
        result += "INSERT INTO center_draught\n  (ship_id, trim, volume, value_x, value_y, value_z)\nVALUES\n";
        self.center_draught
            .iter()
            .for_each(|(trim, volume, value_x, value_y, value_z)| {
                result +=
                    &format!("  ({ship_id}, {trim}, {volume}, {value_x}, {value_y}, {value_z}),\n");
            });
        result.pop();
        result.pop();
        result.push(';');
        result
    }
    //
    fn waterline_breadth(
        &self,
        ship_id: usize,
    ) -> String {
        let mut result = format!("DELETE FROM waterline_breadth WHERE ship_id={ship_id};\n\n");
        result +=
            &format!("INSERT INTO waterline_breadth\n  (ship_id, trim, draught, value)\nVALUES\n");
            self.waterline_breadth.iter().for_each(|(trim, draught, value)| {
            result += &format!("  ({ship_id}, {trim}, {draught}, {value}),\n");
        });
        result.pop();
        result.pop();
        result.push(';');
        //    dbg!(&result);
        result
    }
    //
    fn hydrostatic_to_sql(
        &self,
        data: &Vec<(f64, f64, f64)>,
        table_name: &str,
        ship_id: usize,
    ) -> String {
        let mut result = format!("DELETE FROM {table_name} WHERE ship_id={ship_id};\n\n");
        result +=
            &format!("INSERT INTO {table_name}\n  (ship_id, trim, volume, value)\nVALUES\n");
        data.iter().for_each(|(trim, volume, value)| {
            result += &format!("  ({ship_id}, {trim}, {volume}, {value}),\n");
        });
        result.pop();
        result.pop();
        result.push(';');
        //    dbg!(&result);
        result
    }
}
///
impl Table for HydrostaticCurves {
    //
    fn parse(&mut self) -> Result<(), Error> {
        println!("HydrostaticCurves parse begin");
        //    dbg!(&self.data);
        let data = self.data.clone();
        let mut data: Vec<(usize, Vec<String>)> = data.into_iter().enumerate().collect();
        data.remove(0);
        let mut parsed: Vec<(usize, Vec<f64>)> = Vec::new();
        for (i, row) in data.into_iter() {
            let mut parsed_row = Vec::new();
            for v in &row {
                if let Ok(v) = v.parse() {
                    parsed_row.push(v);
                } else {
                    return Err(Error::FromString(format!(
                        "HydrostaticCurves parse error: row:{i} {:?}",
                        row
                    )));
                }
            }
            parsed.push((i, parsed_row));
        }

        for (i, row) in parsed.into_iter() {
            //    dbg!(&row);
            if row.len() <= 19 {
                return Err(Error::FromString(format!(
                    "HydrostaticCurves parse error, {i} row.len() {} <= 19",
                    row.len()
                )));
            }
            let trim = row[0];
            let draught = row[1];
            let volume = row[3];
            self.mean_draught.push((trim, volume, draught));
            self.center_draught.push((trim, volume, row[4], 0., row[5]));
            self.waterline_area.push((trim, volume, row[6]));
            self.center_waterline.push((trim, volume, row[7]));
            self.waterline_breadth.push((trim, draught, row[19]));
            self.rad_trans.push((trim, volume, row[10]));
            self.rad_long.push((trim, volume, row[11]));
        }
        //    dbg!(&self.mean_draught);
        println!("HydrostaticCurves parse ok");
        Ok(())
    }
    //
    fn to_sql(&self, id: usize) -> Vec<String> {
        let mut result = Vec::new();
        result.push(self.center_draught(id));
        result.push(self.waterline_breadth(id));
        result.push(self.hydrostatic_to_sql(&self.waterline_area, "waterline_area", id));
        result.push(self.hydrostatic_to_sql(&self.center_waterline, "center_waterline", id));
        result.push(self.hydrostatic_to_sql(&self.mean_draught, "mean_draught", id));
        result.push(self.hydrostatic_to_sql(&self.rad_trans, "rad_trans", id));
        result.push(self.hydrostatic_to_sql(&self.rad_long, "rad_long", id));
        //    dbg!(&result);
        result
    }
    //
    fn to_file(&self, id: usize, name: &str) {
        fs::write(format!("../{name}/hidrostatic/center_draught.sql"), self.center_draught(id))
            .expect("Unable to write file center_draught.sql");
        sleep(std::time::Duration::from_secs(1));
        fs::write(format!("../{name}/hidrostatic/waterline_breadth.sql"), self.waterline_breadth(id))
            .expect("Unable to write file waterline_breadth.sql");
        sleep(std::time::Duration::from_secs(1));
        fs::write(
            format!("../{name}/hidrostatic/waterline_area.sql"),
            self.hydrostatic_to_sql(&self.waterline_area, "waterline_area", id),
        )
        .expect("Unable to write file waterline_area.sql");
        sleep(std::time::Duration::from_secs(1));
        fs::write(
            format!("../{name}/hidrostatic/center_waterline.sql"),
            self.hydrostatic_to_sql(&self.center_waterline, "center_waterline", id),
        )
        .expect("Unable to write file center_waterline.sql");
        sleep(std::time::Duration::from_secs(1));
        fs::write(
            format!("../{name}/hidrostatic/mean_draught.sql"),
            self.hydrostatic_to_sql(&self.mean_draught, "mean_draught", id),
        )
        .expect("Unable to write file mean_draught.sql");
        sleep(std::time::Duration::from_secs(1));
        fs::write(
            format!("../{name}/hidrostatic/rad_trans.sql"),
            self.hydrostatic_to_sql(&self.rad_trans, "rad_trans", id),
        )
        .expect("Unable to write file rad_trans.sql");
        sleep(std::time::Duration::from_secs(1));
        fs::write(
            format!("../{name}/hidrostatic/rad_long.sql"),
            self.hydrostatic_to_sql(&self.rad_long, "rad_long", id),
        )
        .expect("Unable to write file rad_long.sql");
        sleep(std::time::Duration::from_secs(1));
    }
}
