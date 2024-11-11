//! Класс-коллекция таблиц. Проверяет данные и выполняет их запись
use crate::error::Error;
use crate::parse_tests::ShipGeneral;
use crate::Angle;
use crate::ApiServer;
use crate::BonjeanFrame;
use crate::Bulkhead;
use crate::BulkheadPlace;
use crate::Compartment;
use crate::CompartmentCurve;
use crate::General;
use crate::HoldCurve;
use crate::HoldGroup;
use crate::HoldPart;
use crate::HorizontalSurf;
use crate::HydrostaticCurves;
use crate::LoadConstant;
use crate::MinMetacentricHeightSubdivision;
use crate::Pantocaren;
use crate::PhysicalFrame;
use crate::StrengthForceLimit;
use crate::Table;
use crate::TheoreticalFrame;
use crate::VerticalAreaStrength;
use crate::Windage;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use calamine::Range;
use calamine::{Reader, open_workbook, Xlsx, Data};
//use api_tools::client::api_query::*;
//use api_tools::client::api_request::ApiRequest;

/// Класс-коллекция таблиц. Проверяет данные и выполняет их запись
pub struct Parser {
    data: String,
    api_server: Rc<RefCell<ApiServer>>,
    general: Option<General>,
    physical_frame: Option<Rc<PhysicalFrame>>,
    parsed: HashMap<String, Box<dyn Table>>,
}
///
impl Parser {
    ///
    pub fn new(data: String, api_server: Rc<RefCell<ApiServer>>) -> Self {
        Self {
            data,
            api_server,
            general: None,
            physical_frame: None,
            parsed: HashMap::<String, Box<dyn Table>>::new(),
        }
    }
    /// Конвертация и проверка данных
    pub fn convert_data(&mut self) -> Result<(), Error> {
        println!("Parser convert begin");
        let json_data: serde_json::Value = serde_json::from_str(&self.data)?;
        //  println!("Data: {}", json_data);
        let fields = json_data
            .get("fields")
            .ok_or(Error::FromString(format!("No fields in data!")))?;
        //    println!("fields: {}", fields);
        let fields = fields
            .as_array()
            .ok_or(Error::FromString(format!("fields no array!")))?;
        let compartment_curve_data: Vec<_> = fields
        .iter()
        .filter(|f| f.get("tag").unwrap().as_str().unwrap() == "compartment_curve")
        .map(|f| {
            (
                f.get("name").unwrap().as_str().unwrap().to_string(),
                f.get("body").unwrap().as_str().unwrap().to_string(),
            )
        })
        .collect();
        let hold_curve_data: Vec<_> = fields
        .iter()
        .filter(|f| f.get("tag").unwrap().as_str().unwrap() == "hold_curve")
        .map(|f| {
            (
                f.get("code").unwrap().as_str().unwrap().to_string(),
                f.get("body").unwrap().as_str().unwrap().to_string(),
            )
        })
        .collect();
        let fields: HashMap<String, String> = fields
            .iter()
            .map(|f| {
                (
                    f.get("tag").unwrap().as_str().unwrap().to_string(),
                    f.get("body").unwrap().as_str().unwrap().to_string(),
                )
            })
            .collect();

        let general = General::new(
            fields.get("general").ok_or(Error::FromString(format!(
                "Parser convert error: no general!"
            )))?.to_string(),
            Rc::clone(&self.api_server),
        );
   //     general.parse()?;
        self.general = Some(general);

        let mut physical_frame = PhysicalFrame::new(fields.get("physical_frame").ok_or(
            Error::FromString(format!("Parser convert error: no physical_frame!")),
        )?.to_string());
        physical_frame.parse()?;
        self.physical_frame = Some(Rc::new(physical_frame));

        let mut compartment = Box::new(Compartment::new(
            fields.get("compartment").ok_or(Error::FromString(format!(
                "Parser convert error: no compartment!"
            )))?.to_string(),
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
        self.parsed.insert("compartment".to_owned(), compartment);

        let mut compartment_curve = Box::new(CompartmentCurve::new(compartment_curve_data));
        compartment_curve.parse()?;
        self.parsed.insert("compartment_curve".to_owned(), compartment_curve);

        let mut hold_group = Box::new(HoldGroup::new(fields.get("hold_group").ok_or(
            Error::FromString(format!("Parser convert error: no hold_group!")),
        )?.to_string()));
        hold_group.parse()?;
        self.parsed.insert("hold_group".to_owned(), hold_group);

        let mut hold_part = Box::new(HoldPart::new(fields.get("hold_part").ok_or(
            Error::FromString(format!("Parser convert error: no hold_part!")),
        )?.to_string()));
        hold_part.parse()?;
        self.parsed.insert("hold_part".to_owned(), hold_part);

        let mut hold_curve = Box::new(HoldCurve::new(hold_curve_data));
        hold_curve.parse()?;
        self.parsed.insert("hold_curve".to_owned(), hold_curve);

        for (tag, body) in fields {
            match tag.as_str()  {
                text => {
                    let mut table: Box<dyn Table> = match text {
                        "frames_theoretical" => Box::new(TheoreticalFrame::new(body)),
                        "bonjean" => Box::new(BonjeanFrame::new(body)),
                        "load_constant" => Box::new(LoadConstant::new(
                            body,
                            Rc::clone(&self.physical_frame.clone().ok_or(Error::FromString(
                                format!("Parser LoadConstant error: physical_frame"),
                            ))?),
                        )),
                        "strength_limits_sea" => {
                            Box::new(StrengthForceLimit::new("sea\r\n".to_owned() + &body))
                        }
                        "strength_limits_harbor" => {
                            Box::new(StrengthForceLimit::new("harbor\r\n".to_owned() + &body))
                        }
                        "hydrostatic_curves" => Box::new(HydrostaticCurves::new(body)),
                        "pantocaren" => Box::new(Pantocaren::new(body)),
                        "angle" => Box::new(Angle::new(body)),
                        "windage" => Box::new(Windage::new(body)),
                        "vertical_area_strength" => Box::new(VerticalAreaStrength::new(body)),
                        "horizontal_surf" => Box::new(HorizontalSurf::new(body)),
                        "min_metacentric_height_subdivision" => Box::new(MinMetacentricHeightSubdivision::new(body)),
                        "bulkhead" => Box::new(Bulkhead::new(body)),
                        "bulkhead_place" => Box::new(BulkheadPlace::new(body)),
                        "general" => continue,
                        "physical_frame" => continue,
                        "compartment" => continue,
                        "compartment_curve" => continue,
                        "hold_group" => continue,
                        "hold_part" => continue,
                        "hold_curve" => continue,
                        _ => Err(Error::FromString(format!("Unknown tag: {text}")))?,
                    };
                    table.parse()?;
                    self.parsed.insert(text.to_owned(), table);
                }
            }
        }
        println!("Parser convert end");
        Ok(())
    }
    /// Конвертация тестов
    pub fn convert_tests(&mut self, path: &str) -> Result<(), Error> {
        let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");
        let workbook: Vec<(String, Range<Data>)> = workbook.worksheets().into_iter().filter(|(tag, range)| range.used_cells().count() > 0 ).collect();
        for (tag, data) in workbook {
            let data: Vec<&[Data]> = data.rows().filter(|v| v.is_empty()).collect();
            match tag.to_lowercase().as_str()  {
                text => {
                    let mut table: Box<dyn Table> = match text {
                        "general" => Box::new(ShipGeneral::new(data)),
                        _ => continue,
                    };
                }
            }
        }
        dbg!(workbook);
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
        self.parsed.iter().next().map(|(_, table)| {
            table
                .to_sql(ship_id)
                .iter()
                .for_each(|sql| full_sql += &sql);
        });
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
    /// Запись данных в виде скриптов для БД
    pub fn write_to_file(self) -> Result<(), Error> {
        println!("Parser write_to_file begin");
        if let Some(general) = self.general {
            let ship_id = general.ship_id()?;
            let ship_name = general.ship_name()?;

            std::fs::create_dir_all(format!("../{ship_name}/area/"))?;
            std::fs::create_dir_all(format!("../{ship_name}/frames/"))?;
            std::fs::create_dir_all(format!("../{ship_name}/hidrostatic/"))?;
            std::fs::create_dir_all(format!("../{ship_name}/hold/"))?;
            std::fs::create_dir_all(format!("../{ship_name}/loads/"))?;
            std::fs::create_dir_all(format!("../{ship_name}/tests/"))?;
            general.to_file(ship_id, &ship_name);
            if let Some(physical_frame) = self.physical_frame {
                physical_frame.to_file(ship_id, &ship_name);
            }
            for (table_name, table) in self.parsed {
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
        println!("Parser write_to_file end");
        Ok(())
    }
}
