//! Структура с данными для bonjean_frame
use crate::error::Error;
use crate::Table;

/// Структура с данными для bonjean_frame
pub struct HydrostaticCurves {
    data: Option<String>,
    /// Средняя осадка, зависимость от объемного водоизмещения,
    /// V, м3 | T, м  ship_id, key, value
    mean_draught: Vec<(f64, f64)>,   
    /// Отстояние центра величины погруженной части судна в
    /// зависимости от объемного водоизмещения,
    ///  V, м3 | Xc, м | Yc, м, | Zc, м  ship_id, key, value_x, value_y, value_z)
    center_draught: Vec<(f64, f64, f64, f64)>,
    /// Площадь ватерлинии,
    ///  V, м3 | S, м^2  ship_id, key, value
    waterline_area: Vec<(f64, f64)>,
    /// Абсцисса центра тяжести площади ватерлинии,
    ///  V, м3 | Xf, м  ship_id, key, value
    center_waterline: Vec<(f64, f64)>,
    /// Длинна судна по ватерлинии в зависимости от осадки
    /// d, м | z, м  ship_id, key, value
    waterline_length: Vec<(f64, f64)>,   
    /// Ширина судна по ватерлинии в зависимости от осадки
    /// d, м | z, м  ship_id, key, value
    waterline_breadth: Vec<(f64, f64)>,   
    /// Поперечный метацентрический радиус в зависимости от объемного водоизмещения
    /// V, м3 | ro, м  
    rad_trans: Vec<(f64, f64)>, 
    /// Продольный метацентрический радиус в зависимости от объемного водоизмещения
    ///  V, м3 | Ro, м
    rad_long: Vec<(f64, f64)>,  
}
///
impl HydrostaticCurves {
    ///
    pub fn new(data: String) -> Self {
        Self {
            data: Some(data),
            mean_draught: Vec::new(), 
            center_draught: Vec::new(),
            waterline_area: Vec::new(),
            center_waterline: Vec::new(),
            waterline_length: Vec::new(),
            waterline_breadth: Vec::new(),
            rad_trans: Vec::new(),
            rad_long: Vec::new(),
        }
    }
}
///
impl Table for HydrostaticCurves {
    ///
    fn data(&mut self) -> Option<String> {
        self.data.take()
    }
    ///
    fn parse(&mut self) -> Result<(), Error> {
    //    dbg!(&self.data);
        let data = self.split_data()?;
        let mut data:Vec<(usize, Vec<String>)> = data.into_iter().enumerate().collect();
        data.remove(0);
        let mut parsed: Vec<(usize, Vec<f64>)> = Vec::new();
        for (i, row) in data.into_iter() {
            let mut parsed_row = Vec::new();
            for v in &row {                
                if let Ok(v) = v.parse() {
                    parsed_row.push(v);
                } else {
                    return Err(Error::FromString(format!("HydrostaticCurves parse error: row:{i} {:?}", row)));
                }
            }
            parsed.push((i, parsed_row));
        } 

        for (i, row) in parsed.into_iter() {
        //    dbg!(&row);
            if row.len() != 23 {
                return Err(Error::FromString(format!("HydrostaticCurves parse error, {i} row.len() {} != 23", row.len())));
            }
            let draught = row[0];
            let volume = row[2];
            self.mean_draught.push((volume, draught));
            self.center_draught.push((volume, row[3], 0., row[4]));
            self.waterline_area.push((volume, row[5]));  
            self.center_waterline.push((volume, row[6]));  
            self.waterline_length.push((draught, row[17]));
            self.waterline_breadth.push((draught, row[18]));
            self.rad_trans.push((volume, row[9]));
            self.rad_long.push((volume, row[10]));
        }
    //    dbg!(&self.mean_draught);
        Ok(())
    }
    ///
    fn to_sql(&mut self, id: usize) -> Vec<String> {
        let mut result = Vec::new();
        result.append(&mut self.data_to_sql(&self.mean_draught, "mean_draught", id));
        {
            result.push(format!(" DELETE FROM center_draught WHERE ship_id={id};"));
            let mut center_draught = " INSERT INTO center_draught (ship_id, key, value_x, value_y, value_z) VALUES".to_owned();
            self.center_draught.iter().for_each(|(k, x, y, z)| {
                center_draught += &format!(" ({id}, {k}, {x}, {y}, {z}),");
            });
            center_draught.pop();
            center_draught.push(';');
        //    dbg!(&center_draught);
            result.push(center_draught);
        }
        result.append(&mut self.data_to_sql(&self.waterline_area, "waterline_area", id));
        result.append(&mut self.data_to_sql(&self.center_waterline, "center_waterline", id));
        result.append(&mut self.data_to_sql(&self.waterline_length, "waterline_length", id));
        result.append(&mut self.data_to_sql(&self.waterline_breadth, "waterline_breadth", id));
        result.append(&mut self.data_to_sql(&self.rad_trans, "rad_trans", id));
        result.append(&mut self.data_to_sql(&self.rad_long, "rad_long", id));
    //    dbg!(&result);
        result
    }
}
