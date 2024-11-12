//! Структура с данными запасов
use crate::error::Error;
use crate::Table;

/// Структура с данными запасов
pub struct Stores {
    data: Vec<Vec<String>>,
    /// Name	Waight [t]	LCG [m]	TCG [m]	VCG [m]	X1	X2
    parsed: Vec<(String, String, String, String, String, String, String)>,
}
//
impl Stores {
    //
    pub fn new(data: Vec<Vec<String>>) -> Self {
        Self {
            data,
            parsed: Vec::new(),
        }
    }
    //
    fn to_string(&self, ship_id: usize) -> String {
        let mut result = "INSERT INTO cargo\n  (ship_id, name, mass, bound_x1, bound_x2, mass_shift_x, mass_shift_y, mass_shift_z, category_id)\n  VALUES\n".to_string();
        self.parsed.iter().for_each(|(name, mass, dx, dy, dz, x1, x2)| {
            result += &format!(" ({ship_id}, '{name}', {mass}, {x1}, {x2}, {dx}, {dy}, {dz}, 9),\n");
        });
        result.pop();
        result.pop();
        result.push(';');
        result
    }
}
//
impl Table for Stores {
    //
    fn parse(&mut self) -> Result<(), Error> {
        println!("Stores parse begin");
        self.parsed = self.data.iter().map(|v|(v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone(), v[4].clone(), v[5].clone(), v[6].clone())).collect();
        self.parsed.remove(0);
        println!("Stores parse ok");
        //  dbg!(&self.parsed);
        Ok(())
    }
    //
    fn to_file(&self, ship_id: usize, name: &str) {
        std::fs::write(
            format!("../{name}/test/stores.sql"),
            self.to_string(ship_id),
        )
        .expect("Unable to write file /test/stores.sql");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    //
    fn to_sql(&self, ship_id: usize) -> Vec<String> {
        vec![self.to_string(ship_id)]
    }
}
