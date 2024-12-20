//! Класс-коллекция таблиц. Проверяет данные и выполняет их запись
use crate::error::Error;
use crate::parse_tests;
use crate::parse_tests::BulkCargo;
use crate::parse_tests::ShipGeneral;
use crate::ApiServer;
use crate::BonjeanFrame;
use crate::Bulkhead;
use crate::BulkheadPlace;
use crate::Compartment;
use crate::CompartmentCurve;
use crate::DraftMark;
use crate::EntryAngle;
use crate::FloodAngle;
use crate::General;
use crate::HoldCurve;
use crate::HoldGroup;
use crate::HoldPart;
use crate::HorizontalSurf;
use crate::HydrostaticCurves;
use crate::LoadConstant;
use crate::LoadLine;
use crate::MinMetacentricHeightSubdivision;
use crate::Pantocaren;
use crate::PhysicalFrame;
use crate::StrengthForceLimit;
use crate::Table;
//use crate::TheoreticalFrame;
use crate::VerticalAreaStrength;
use crate::Windage;
use calamine::Range;
use calamine::{open_workbook, Data, Reader, Xlsx};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
//use api_tools::client::api_query::*;
//use api_tools::client::api_request::ApiRequest;

/// Класс-коллекция таблиц. Проверяет данные и выполняет их запись
pub struct Parser {
    data_path: String,
    test_path: String,
    file_name_prefix: String,
    api_server: Rc<RefCell<ApiServer>>,
    general: Option<General>,
    physical_frame: Option<Rc<PhysicalFrame>>,
    parsed_data: HashMap<String, Box<dyn Table>>,
    parsed_tests: HashMap<String, HashMap<String, Box<dyn Table>>>,
}
///
impl Parser {
    ///
    pub fn new(path: &str, file_name_prefix: &str, api_server: Rc<RefCell<ApiServer>>) -> Self {
        Self {
            data_path: path.to_owned() + "datasets/",
            test_path: path.to_owned() + "test/",
            file_name_prefix: file_name_prefix.to_owned(),
            api_server,
            general: None,
            physical_frame: None,
            parsed_data: HashMap::<String, Box<dyn Table>>::new(),
            parsed_tests: HashMap::<String, HashMap<String, Box<dyn Table>>>::new(),
        }
    }
    /// Конвертация и проверка данных
    pub fn convert_data(&mut self) -> Result<(), Error> {
        println!("Parser convert begin");
        {
            let data = self.get_hash_map("", "General.xlsx");
            {
                let data = data.get("General").ok_or(Error::FromString(format!(
                    "Parser convert error: no general!"
                )))?;
                let mut table = General::new(data.to_owned(), Rc::clone(&self.api_server));
                table.parse()?;
                self.general = Some(table);
            }
            {
                let data = data.get("Fr").ok_or(Error::FromString(format!(
                    "Parser convert error: no physical frame!"
                )))?;
                let mut table = PhysicalFrame::new(data.to_owned());
                table.parse()?;
                self.physical_frame = Some(Rc::new(table));
            }
        }

        let mut compartment = Box::new(Compartment::new(
            self.get_hash_map("Stability", "Compartments.xlsx")
                .get("Compartments")
                .ok_or(Error::FromString(format!(
                    "Parser convert error: no Compartments!"
                )))?
                .to_owned(),
            Rc::clone(
                &self
                    .physical_frame
                    .clone()
                    .ok_or(Error::FromString(format!(
                        "Parser Compartment error: physical_frame"
                    )))?,
            ),
        ));
        compartment.parse()?;
        self.parsed_data
            .insert("compartment_curve".to_owned(), compartment);

        {
            let data = self.get_hash_map("Loads", "CargoData.xlsx");
            {
                let data = data
                    .get("CargoCompartments")
                    .ok_or(Error::FromString(format!(
                        "Parser convert error: no CargoCompartments!"
                    )))?;
                let mut table = Box::new(HoldGroup::new(data.to_owned()));
                table.parse()?;
                self.parsed_data
                    .insert("CargoCompartments".to_owned(), table);
            }
            {
                let data = data
                    .get("CargoCompartmentsParts")
                    .ok_or(Error::FromString(format!(
                        "Parser convert error: no CargoCompartmentsParts!"
                    )))?;
                let mut table = Box::new(HoldPart::new(data.to_owned()));
                table.parse()?;
                self.parsed_data
                    .insert("CargoCompartmentsParts".to_owned(), table);
            }
            {
                let data = data
                    .get("GrainBulkheadsPlace")
                    .ok_or(Error::FromString(format!(
                        "Parser convert error: no GrainBulkheadsPlace!"
                    )))?;
                let mut table = Box::new(BulkheadPlace::new(data.to_owned()));
                table.parse()?;
                self.parsed_data
                    .insert("GrainBulkheadsPlace".to_owned(), table);
            }
            {
                let data = data
                    .get("GrainBulkheadsPlace")
                    .ok_or(Error::FromString(format!(
                        "Parser convert error: no GrainBulkheadsPlace!"
                    )))?;
                let mut table = Box::new(Bulkhead::new(data.to_owned()));
                table.parse()?;
                self.parsed_data
                    .insert("GrainBulkheadsPlace".to_owned(), table);
            }
        }

        {
            let data = self.get_hash_map("Strength", "Strength.xlsx");
            {
                let data = data.get("Bonjan").ok_or(Error::FromString(format!(
                    "Parser convert error: no BonjeanFrame!"
                )))?;
                let mut table = Box::new(BonjeanFrame::new(data.to_owned()));
                table.parse()?;
                self.parsed_data.insert("BonjeanFrame".to_owned(), table);
            }
            {
                let data = data.get("Waight").ok_or(Error::FromString(format!(
                    "Parser convert error: no Waight!"
                )))?;
                let mut table =
                    Box::new(LoadConstant::new(
                        data.to_owned(),
                        Rc::clone(&self.physical_frame.clone().ok_or(Error::FromString(
                            format!("Parser LoadConstant error: physical_frame"),
                        ))?),
                    ));
                table.parse()?;
                self.parsed_data.insert("Waight".to_owned(), table);
            }
            {
                let data = data.get("StrengthLimSea").ok_or(Error::FromString(format!(
                    "Parser convert error: no StrengthLimSea!"
                )))?;
                let mut table =
                    Box::new(StrengthForceLimit::new(
                        "sea",
                        data.to_owned(),
                        Rc::clone(&self.physical_frame.clone().ok_or(Error::FromString(
                            format!("Parser StrengthForceLimit error: physical_frame"),
                        ))?),
                    ));
                table.parse()?;
                self.parsed_data.insert("StrengthLimSea".to_owned(), table);
            }
            {
                let data = data.get("StrengthLimSea").ok_or(Error::FromString(format!(
                    "Parser convert error: no StrengthLimSea!"
                )))?;
                let mut table =
                    Box::new(StrengthForceLimit::new(
                        "harbor",
                        data.to_owned(),
                        Rc::clone(&self.physical_frame.clone().ok_or(Error::FromString(
                            format!("Parser StrengthForceLimit error: physical_frame"),
                        ))?),
                    ));
                table.parse()?;
                self.parsed_data.insert("StrengthLimSea".to_owned(), table);
            }
        }

        let mut draft = Box::new(DraftMark::new(
            self.get_hash_map("Draft", "DraftScales.xlsx")
                .get("DraftScales")
                .ok_or(Error::FromString(format!(
                    "Parser convert error: no DraftScales!"
                )))?
                .to_owned(),
        ));
        draft.parse()?;
        self.parsed_data.insert("Draft".to_owned(), draft);

        let mut load_line = Box::new(LoadLine::new(
            self.get_hash_map("Draft", "LoadLine.xlsx")
                .get("DraftCriteria ID")
                .ok_or(Error::FromString(format!(
                    "Parser convert error: no LoadLine!"
                )))?
                .to_owned(),
        ));
        load_line.parse()?;
        self.parsed_data.insert("LoadLine".to_owned(), load_line);

        {
            let data = self.get_hash_map("Stability", "Wintage.xlsx");
            {
                let data = data.get("Windage").ok_or(Error::FromString(format!(
                    "Parser convert error: no Windage!"
                )))?;
                let mut table = Box::new(Windage::new(data.to_owned()));
                table.parse()?;
                self.parsed_data.insert("Windage".to_owned(), table);
            }
            {
                let data = data.get("Windage X").ok_or(Error::FromString(format!(
                    "Parser convert error: no Windage X!"
                )))?;
                let mut table = Box::new(VerticalAreaStrength::new(data.to_owned()));
                table.parse()?;
                self.parsed_data.insert("Windage X".to_owned(), table);
            }
            {
                let data = data
                    .get("Horizontal surf")
                    .ok_or(Error::FromString(format!(
                        "Parser convert error: no Horizontal surf!"
                    )))?;
                let mut table = Box::new(HorizontalSurf::new(data.to_owned()));
                table.parse()?;
                self.parsed_data.insert("Horizontal surf".to_owned(), table);
            }
        }

        let mut min_metacentric_height_subdivision =
            Box::new(MinMetacentricHeightSubdivision::new(
                self.get_hash_map("Stability", "hminsi.xlsx")
                    .get("h min si")
                    .ok_or(Error::FromString(format!(
                        "Parser convert error: no hminsi!"
                    )))?
                    .to_owned(),
            ));
        min_metacentric_height_subdivision.parse()?;
        self.parsed_data
            .insert("hminsi".to_owned(), min_metacentric_height_subdivision);

        let mut compartment_curve = Box::new(CompartmentCurve::new(
            self.get_range_vec("Stability", "Tank_curve.xlsx"),
        ));
        compartment_curve.parse()?;
        self.parsed_data
            .insert("compartment_curve".to_owned(), compartment_curve);

        let mut hold_curve = Box::new(HoldCurve::new(
            self.get_range_vec("Loads", "Holds_curve.xlsx"),
        ));
        hold_curve.parse()?;
        self.parsed_data.insert("hold_curve".to_owned(), hold_curve);

        let data = self.get_hash_map("Stability", "Pantokarens_Angle_HydrostaticCurves.xlsx");
        for (tag, data) in data {
            let mut table: Box<dyn Table> = match tag.to_lowercase().as_str() {
                "angle" => Box::new(FloodAngle::new(data)),
                "deckangle" => Box::new(EntryAngle::new(data)),
                "hydrostaticcurves" => Box::new(HydrostaticCurves::new(data)),
                "pantokarens" => Box::new(Pantocaren::new(data)),
                _ => Err(Error::FromString(format!(
                    "Unknown tag in hydrostatic_data: {tag}"
                )))?,
            };
            table.parse()?;
            self.parsed_data.insert(tag.to_owned(), table);
        }

        println!("Parser convert end");
        Ok(())
    }
    /// Конвертация тестов
    pub fn convert_tests(&mut self) -> Result<(), Error> {
        let paths = std::fs::read_dir(self.test_path.clone())?;
        for path in paths {
            let path = path?.path().to_str().unwrap().to_owned();
            let mut data: Xlsx<_> =
                open_workbook(&path).expect(&format!("Cannot open file {}", path));
            let data: Vec<(String, Range<Data>)> = data
                .worksheets()
                .into_iter()
                .filter(|(_, range)| range.used_cells().count() > 0)
                .collect();
            if data.is_empty() {
                continue;
            }
            let mut tables = HashMap::new();
            for (tag, data) in data {
                let data: Vec<&[Data]> = data.rows().filter(|v| !v.is_empty()).collect();
                let data = data
                    .iter()
                    .map(|v| v.iter().map(|v| v.to_string()).collect())
                    .collect();
                match tag.to_lowercase().as_str() {
                    text => {
                        let mut table: Box<dyn Table> = match text {
                            "general" => Box::new(ShipGeneral::new(data)),
                            "compartments" => Box::new(parse_tests::Compartment::new(data)),
                            "bulkcargo" => Box::new(BulkCargo::new(data)),
                            "grainbulkheads" => Box::new(parse_tests::GrainBulkheads::new(data)),
                            "stores" => Box::new(parse_tests::Stores::new(data)),
                            _ => continue,
                            //   _ => Err(Error::FromString(format!("Unknown tag: {text}")))?,
                        };
                        table.parse()?;
                        tables.insert(text.to_owned(), table);
                    }
                }
            }
            let filename = path.split('/').last().unwrap().split('.').next().unwrap();
            self.parsed_tests
                .insert(filename.split('.').next().unwrap().to_owned(), tables);
        }
        Ok(())
    }
    /// Запись данных в БД
    pub fn write_to_db(mut self) -> Result<(), Error> {
        println!("Parser write_to_db begin");
        let general = self.general.take().ok_or(Error::FromString(
            "Parser write_to_db error: no general".to_owned(),
        ))?;
        let ship_id = general.ship_id()?;
        let mut full_sql = "DO $$ BEGIN ".to_owned();
        general.to_sql(ship_id).iter().for_each(|sql| {
            full_sql += sql;
        });
        self.parsed_data.iter().next().map(|(_, table)| {
            table
                .to_sql(ship_id)
                .iter()
                .for_each(|sql| full_sql += &sql);
        });
        /*      self.parsed_tests.iter().next().map(|(_, table)| {
            table
                .to_sql(ship_id)
                .iter()
                .for_each(|sql| full_sql += &sql);
        });*/
        full_sql += " END$$;";
        //   dbg!(&full_sql);
        if let Err(error) = self.api_server.borrow_mut().fetch(&full_sql) {
            println!(
                "{}",
                format!("Parser write_to_db error:{}", error.to_string())
            );
        }
        println!("Parser write_to_db end");
        Ok(())
    }
    /// Запись данных и тестов в виде скриптов для БД
    pub fn write_to_file(mut self) -> Result<(), Error> {
        println!("Parser write_to_file begin");
        if let Some(general) = self.general.clone() {
            let ship_id = general.ship_id()?;
            let ship_name = general.ship_name()?;
            let _ = std::fs::remove_dir_all(format!("../{ship_name}/"));
            std::fs::create_dir_all(format!("../{ship_name}/"))?;
            std::fs::create_dir_all(format!("../{ship_name}/area/"))?;
            std::fs::create_dir_all(format!("../{ship_name}/draft/"))?;
            std::fs::create_dir_all(format!("../{ship_name}/frames/"))?;
            std::fs::create_dir_all(format!("../{ship_name}/hidrostatic/"))?;
            std::fs::create_dir_all(format!("../{ship_name}/hold/"))?;
            std::fs::create_dir_all(format!("../{ship_name}/loads/"))?;
            std::fs::create_dir_all(format!("../{ship_name}/test/"))?;
            self.write_data_to_file(ship_id, &ship_name)?;
            self.write_tests_to_file(ship_id, &ship_name)?;
        } else {
            return Err(Error::FromString(
                "Parser write_to_file error: no general".to_owned(),
            ));
        }
        println!("Parser write_to_file end");
        Ok(())
    }
    /// Запись данных в виде скриптов для БД
    fn write_data_to_file(&mut self, ship_id: usize, ship_name: &str) -> Result<(), Error> {
        println!("Parser write_data_to_file begin");
        if let Some(general) = self.general.clone() {
            general.to_file(ship_id, &ship_name);
            if let Some(physical_frame) = self.physical_frame.take() {
                physical_frame.to_file(ship_id, &ship_name);
            }
            for (table_name, table) in &self.parsed_data {
                println!("{table_name} to_file begin");
                std::thread::sleep(std::time::Duration::from_secs(1));
                table.to_file(ship_id, &ship_name);
                println!("{table_name} to_file end");
            }
        } else {
            return Err(Error::FromString(
                "Parser write_to_file error: no general".to_owned(),
            ));
        }
        println!("Parser write_data_to_file end");
        Ok(())
    }
    /// Запись данных в виде скриптов для БД
    pub fn write_tests_to_file(&mut self, ship_id: usize, ship_name: &str) -> Result<(), Error> {
        println!("Parser write_tests_to_file begin");
        for (file_name, tables) in &self.parsed_tests {
            println!("{file_name} tests to_file begin",);
            let mut tests = String::new();
            tables
                .get("general")
                .ok_or(Error::FromString(format!(
                    "Parser write_to_file tests error: no table general!"
                )))?
                .to_sql(ship_id)
                .iter()
                .for_each(|s| tests += &s);
            tests += "\n\n";
            tables
                .get("compartments")
                .ok_or(Error::FromString(format!(
                    "Parser write_to_file tests error: no table general!"
                )))?
                .to_sql(ship_id)
                .iter()
                .for_each(|s| tests += &s);
            tests += "\n\n";
            tables
                .get("stores")
                .ok_or(Error::FromString(format!(
                    "Parser write_to_file tests error: no table general!"
                )))?
                .to_sql(ship_id)
                .iter()
                .for_each(|s| tests += &s);
            tests += "\n\n";
            tables
                .get("grainbulkheads")
                .ok_or(Error::FromString(format!(
                    "Parser write_to_file tests error: no table general!"
                )))?
                .to_sql(ship_id)
                .iter()
                .for_each(|s| tests += &s);
            tests += "\n\n";
            tables
                .get("bulkcargo")
                .ok_or(Error::FromString(format!(
                    "Parser write_to_file tests error: no table general!"
                )))?
                .to_sql(ship_id)
                .iter()
                .for_each(|s| tests += &s);
            tests += "\n\n";
            std::fs::write(format!("../{ship_name}/test/{file_name}.sql"), tests)
                .expect("Unable to write file /test/data.sql");
            println!("{file_name} tests to_file end",);
        }

        println!("Parser write_tests_to_file end");
        Ok(())
    }
    //
    fn get_hash_map(&self, sub_path: &str, filename: &str) -> HashMap<String, Vec<Vec<String>>> {
        let path = self.data_path.clone() + sub_path + "/" + &self.file_name_prefix + filename;
        let mut data: Xlsx<_> = open_workbook(&path).expect(&format!("Cannot open file {}", path));
        data.worksheets()
            .into_iter()
            .filter(|(_, range)| range.used_cells().count() > 0)
            .map(|(tag, data)| {
                (
                    tag,
                    data.rows()
                        .filter(|v| !v.is_empty())
                        .map(|v| v.iter().map(|v| v.to_string()).collect::<Vec<String>>())
                        .collect::<Vec<Vec<String>>>(),
                )
            })
            .collect::<HashMap<String, Vec<Vec<String>>>>()
    }
    //
    fn get_range_vec(&self, sub_path: &str, filename: &str) -> Vec<(String, Range<Data>)> {
        let path = self.data_path.clone() + sub_path + "/" + &self.file_name_prefix + filename;
        let mut data: Xlsx<_> = open_workbook(&path).expect(&format!("Cannot open file {}", path));
        data.worksheets()
            .into_iter()
            .filter(|(_, range)| range.used_cells().count() > 0)
            .collect()
    }
}
