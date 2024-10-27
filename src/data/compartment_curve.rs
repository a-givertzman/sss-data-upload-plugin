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
        let mut result = format!("DELETE FROM compartment_curve WHERE id IN (SELECT id FROM compartment WHERE name='{}');\n\n", self.space_name);
        result += &format!("INSERT INTO compartment_curve\n
          (ship_id, level, volume, buoyancy_x, buoyancy_y, buoyancy_z, trans_inertia_moment_self, long_inertia_moment_self)\nSELECT\n");
        self.parsed.iter().for_each(|(level, volume, buoyancy_x, buoyancy_y, buoyancy_z, trans_inertia_moment_self, long_inertia_moment_self)| {
            result += &format!("  ({ship_id}, id as space_id, {level}, {volume}, {buoyancy_x}, {buoyancy_y}, {buoyancy_z}, {trans_inertia_moment_self}, {long_inertia_moment_self}),\n");
        });
        result += &format!("FROM compartment WHERE name='{}';", self.space_name);
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
        for row in data.into_iter() {
            let level = row
                .get(0)
                .ok_or(Error::FromString(
                    "CompartmentCurve error: no level in row".to_owned(),
                ))?
                .to_owned();
            let volume = row
                .get(1)
                .ok_or(Error::FromString(
                    "CompartmentCurve error: no volume in row".to_owned(),
                ))?
                .to_owned();
            let buoyancy_x = row
                .get(2)
                .ok_or(Error::FromString(
                    "CompartmentCurve error: no buoyancy_x in row".to_owned(),
                ))?
                .to_owned();
            let buoyancy_y = row
                .get(3)
                .ok_or(Error::FromString(
                    "CompartmentCurve error: no buoyancy_y in row".to_owned(),
                ))?
                .to_owned();
            let buoyancy_z = row
                .get(4)
                .ok_or(Error::FromString(
                    "CompartmentCurve error: no buoyancy_z in row".to_owned(),
                ))?
                .to_owned();
            let trans_inertia_moment_self = row
                .get(5)
                .ok_or(Error::FromString(
                    "CompartmentCurve error: no trans_inertia_moment_self in row".to_owned(),
                ))?
                .to_owned();
            let long_inertia_moment_self = row
                .get(6)
                .ok_or(Error::FromString(
                    "CompartmentCurve error: no long_inertia_moment_self in row".to_owned(),
                ))?
                .to_owned();
            self.parsed.push((
                level,
                volume,
                buoyancy_x,
                buoyancy_y,
                buoyancy_z,
                trans_inertia_moment_self,
                long_inertia_moment_self,
            ));
        }
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
