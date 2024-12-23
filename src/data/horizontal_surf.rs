//! Структура с данными горизонтальных поверхностей
use crate::error::Error;
use crate::Table;

/// Структура с данными горизонтальных поверхностей
pub struct HorizontalSurf {
    data: Vec<Vec<String>>,
    /// AREA [m2]	VCG [m]	LCG [m]	TCG [m]	X1 [m]	X2 [m]
    parsed: Vec<Vec<String>>,
}
//
impl  HorizontalSurf {
    //
    pub fn new(data: Vec<Vec<String>>,) -> Self {
        Self {
            data,
            parsed: Vec::new(), 
        }
    }
    //
    pub fn horizontal_area_stability(&self, ship_id: usize) -> String {
        let mut result = format!("DELETE FROM horizontal_area_stability WHERE ship_id={ship_id};\n\n");
        result += "INSERT INTO horizontal_area_stability\n  (ship_id, name, value, shift_x, shift_y, shift_z)\nVALUES\n";
        self.parsed
            .iter()
            .for_each(|row| {
                result += &format!("  ({ship_id}, '{}', {}, {}, {}, {}),\n", row[0], row[1], row[3], row[2], row[4]);
            });
        result.pop();
        result.pop();
        result.push(';');
        result
    }
    //
    pub fn horizontal_area_strength(&self, ship_id: usize) -> String {
        let mut result = format!("DELETE FROM horizontal_area_strength WHERE ship_id={ship_id};\n\n");
        result += "INSERT INTO horizontal_area_strength\n  (ship_id, name, value, bound_x1, bound_x2)\nVALUES\n";
        self.parsed
            .iter()
            .for_each(|row| {
                result += &format!("  ({ship_id}, '{}', {}, {}, {}),\n", row[0], row[1], row[5], row[6]);
            });
        result.pop();
        result.pop();
        result.push(';');
        result
    }
}
//
impl Table for HorizontalSurf  {
    //
    fn parse(&mut self) -> Result<(), Error> {
        println!("HorizontalSurf parse begin");
        let mut data: Vec<Vec<String>> = self.data.clone().into_iter().filter(|s| s.len() >= 7).collect();
        data.remove(0);
        for row in data.into_iter() {  
            if row.len() != 7 {
                return Err(Error::FromString(format!("HorizontalSurf parse error: row.len() != 7, row={:?}", row)));
            }
            self.parsed.push(row);
        }
        println!("HorizontalSurf parse ok");
        //  dbg!(&self.parsed);
        Ok(())
    }
    //
    fn to_sql(&self, id: usize) -> Vec<String> {
        vec![self.horizontal_area_stability(id), self.horizontal_area_strength(id)]
    }
    //
    fn to_file(&self, id: usize, name: &str) {
        std::fs::write(format!("../{name}/area/horizontal_area_stability.sql"), self.horizontal_area_stability(id))
            .expect("Unable to write file horizontal_area_stability.sql");
        std::thread::sleep(std::time::Duration::from_secs(1));
        std::fs::write(format!("../{name}/area/horizontal_area_strength.sql"), self.horizontal_area_strength(id))
            .expect("Unable to write file horizontal_area_strength.sql");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
