//! Класс-коллекция таблиц. Проверяет данные и выполняет их запись
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::error::Error;
use crate::ApiServer;
use crate::HydrostaticCurves;
use crate::StrengthForceLimit;
use crate::Table;
use crate::LoadConstant;
use crate::General;
use crate::PhysicalFrame;
use crate::TheoreticalFrame;
use crate::BonjeanFrame;
//use api_tools::client::api_query::*;
//use api_tools::client::api_request::ApiRequest;

/// Класс-коллекция таблиц. Проверяет данные и выполняет их запись
pub struct Parser {
    data: String,
    api_server: Rc<RefCell<ApiServer>>,
    general: Option<General>,    
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
                text => {
                    let mut table: Box::<dyn Table> = match text {
                        "frames_theoretical" => Box::new(TheoreticalFrame::new(body)),
                        "frames_physical" => Box::new(PhysicalFrame::new(body)),
                        "bonjean" => Box::new(BonjeanFrame::new(body)),
                        "weight_distribution" => Box::new(LoadConstant::new(body)),
                        "strength_limits_sea" => Box::new(StrengthForceLimit::new("sea\r\n".to_owned() + &body)),
                        "strength_limits_harbor" => Box::new(StrengthForceLimit::new("harbor\r\n".to_owned() + &body)),
        //                "compartments" => Box::new( ::new(body)),
                        "hydrostatic_curves" => Box::new(HydrostaticCurves::new(body)),  
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
        self.parsed.into_iter().next().map(|(_, mut table)| {
            table.to_sql(ship_id).into_iter().for_each(|sql| full_sql += &sql );
        });
        full_sql += " END$$;";
        dbg!(&full_sql);
        if let Err(error) = self.api_server.borrow_mut().fetch(&full_sql) {
            println!("{}", format!("Parser write_to_db error:{}", error.to_string()));
        }
        println!("Parser write_to_db end");
        Ok(())
    }
}
