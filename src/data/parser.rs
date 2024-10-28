//! Класс-коллекция таблиц. Проверяет данные и выполняет их запись
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::error::Error;
use crate::Angle;
use crate::ApiServer;
use crate::Compartment;
use crate::CompartmentCurve;
use crate::HydrostaticCurves;
use crate::Pantokaren;
use crate::StrengthForceLimit;
use crate::Table;
use crate::LoadConstant;
use crate::General;
use crate::PhysicalFrame;
use crate::TheoreticalFrame;
use crate::BonjeanFrame;
use crate::Windage;
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
    pub fn new(data: String, api_server: Rc<RefCell<ApiServer>>,) -> Self {
        Self {
            data,
            api_server,
            general: None,
            physical_frame: None,
            parsed: HashMap::<String, Box<dyn Table>>::new(),            
        }
    }
    /// Конвертация и проверка данных
    pub fn convert(&mut self) -> Result<(), Error> {
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
        for field in fields {
            let tag = field
                .get("tag")
                .ok_or(Error::FromString(format!("No tag, field:{field}")))?
                .as_str()
                .ok_or(Error::FromString(format!("Unknown tag in field:{field}")))?;
            let body = field
                .get("body")
                .ok_or(Error::FromString(format!("No body, field:{field}")))?
                .as_str()
                .ok_or(Error::FromString(format!("Unknown body in field:{field}")))?
                .to_owned();
            match tag {
                "general" => {
                    let mut general = General::new(body, Rc::clone(&self.api_server));
                    general.parse()?;
                    self.general = Some(general);
                }
                "physical_frame" => {
                    let mut physical_frame = PhysicalFrame::new(body);
                    physical_frame.parse()?;
                    self.physical_frame = Some(Rc::new(physical_frame));
                }
                text => {
                    let mut table: Box::<dyn Table> = match text {
                        "frames_theoretical" => Box::new(TheoreticalFrame::new(body)),
                        "bonjean" => Box::new(BonjeanFrame::new(body)),
                        "load_constant" => Box::new(LoadConstant::new(body, Rc::clone(&self.physical_frame.clone().ok_or(Error::FromString(format!("Parser LoadConstant error: physical_frame")))?))),
                        "strength_limits_sea" => Box::new(StrengthForceLimit::new("sea\r\n".to_owned() + &body)),
                        "strength_limits_harbor" => Box::new(StrengthForceLimit::new("harbor\r\n".to_owned() + &body)),
        //                "compartments" => Box::new( ::new(body)),
                        "hydrostatic_curves" => Box::new(HydrostaticCurves::new(body)),  
                        "pantokaren" => Box::new(Pantokaren::new(body)),
                        "angle" => Box::new(Angle::new(body)), 
                        "windage" => Box::new(Windage::new(body)), 
                        "compartment" => Box::new(Compartment::new(body, Rc::clone(&self.physical_frame.clone().ok_or(Error::FromString(format!("Parser Compartment error: physical_frame")))?))),
                        "compartment_curve" => {
                            let name = field
                            .get("name")
                            .ok_or(Error::FromString(format!("No name, field:{field}")))?
                            .as_str()
                            .ok_or(Error::FromString(format!("Unknown name in field:{field}")))?
                            .to_owned();
                            Box::new(CompartmentCurve::new(name, body))
                        }, 
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
    /// Запись данных в БД
    pub fn write_to_db(mut self) -> Result<(), Error> {
        println!("Parser write_to_db begin");
        let mut general = self.general.take().ok_or(Error::FromString("Parser write_to_db error: no general".to_owned()))?;
        let ship_id = general.ship_id()?;
        let mut full_sql = "DO $$ BEGIN ".to_owned();
        general.to_sql(ship_id).iter().for_each(|sql| {
            full_sql += sql;
        });
        self.parsed.iter().next().map(|(_, table)| {
            table.to_sql(ship_id).iter().for_each(|sql| full_sql += &sql );
        });
        full_sql += " END$$;";
     //   dbg!(&full_sql);
        if let Err(error) = self.api_server.borrow_mut().fetch(&full_sql) {
            println!("{}", format!("Parser write_to_db error:{}", error.to_string()));
        }
        println!("Parser write_to_db end");
        Ok(())
    }
    /// Запись данных в виде скриптов для БД
    pub fn write_to_file(self) -> Result<(), Error> {
        println!("Parser write_to_file begin");
        if let Some(general) = self.general {
            let ship_id = general.ship_id()?;
            general.to_file(ship_id);  
            if let Some(physical_frame) = self.physical_frame {
                physical_frame.to_file(ship_id);
            }
            for (table_name, table) in self.parsed {
                println!("{table_name} to_file begin");
                std::thread::sleep(std::time::Duration::from_secs(1));
                table.to_file(ship_id);
                println!("{table_name} to_file end");
            };
        }
        else { 
            return Err(Error::FromString("Parser write_to_file error: no general".to_owned()));
        }
        println!("Parser write_to_file end");
        Ok(())
    }
}
