use std::collections::HashMap;
use crate::error::Error;
use crate::ApiServer;
//use api_tools::client::api_query::*;
//use api_tools::client::api_request::ApiRequest;


mod ship_general;
mod table;
//mod frames_physical;
mod frames_theoretical;

pub use ship_general::*;
pub use table::*;
//pub use frames_physical::*;
pub use frames_theoretical::*;

/// Класс-коллекция таблиц. Проверяет данные и выполняет их запись
pub struct Parser {
    data: String,
    general: Option<General>,    
    parsed: HashMap<String, Box<dyn Table>>,
    api_server: ApiServer,
}
///
impl Parser {
    ///
    pub fn new(data: String) -> Self {
        Self {
            data,
            general: None,
            parsed: HashMap::<String, Box<dyn Table>>::new(),
            api_server: ApiServer::new("sss-computing".to_owned()),
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
                    self.general = Some(General::new(body));
                }
                text => {
                    self.parsed.insert(
                        text.to_owned(),
                        match text {
                            "frames_theoretical" => {
                                let table: Box::<dyn Table> = Box::new(TheoreticalFrame::new(body));
                                table
                            },
                            "frames_physical" => {
                                let table: Box::<dyn Table> = Box::new(TheoreticalFrame::new(body));
                                table
                            },
                            _ => Err(Error::FromString(format!("Unknown tag: {text}")))?,
                        },
                    );
                }
            }
        }
        println!("Parser convert end");
        Ok(())
    }
    /// Запись данных в БД
    pub fn write_to_db(mut self) -> Result<(), Error> {
        println!("Parser write_to_db begin");
        let ship_id = self.general.take().ok_or(Error::FromString("Parser write_to_db error: no general".to_owned()))?.process()?;
        self.parsed.into_iter().for_each(|mut table| {
            if let Err(error) = self.api_server.fetch(&table.1.to_sql(ship_id)) {
                println!("{}", format!("Parser write_to_db error:{}", error.to_string()));
            }
        });
 /*       let mut request = ApiRequest::new(
            "parent",
            "0.0.0.0:8080",
            "auth_token",
            ApiQuery::new(
                ApiQueryKind::Sql(ApiQuerySql::new("sss-computing", "")),
                false,
            ),
            true,
            false,
        );
        self.parsed.into_iter().for_each(|mut table| {
            if let Err(error) = request.fetch(
                &ApiQuery::new(
                    ApiQueryKind::Sql(ApiQuerySql::new("sss-computing", table.1.to_sql(ship_id))),
                    false,
                ),
                true,
            ) {
                println!("{}", format!("Parser write_to_db error:{}", error.to_string()));
            }
        });*/
        println!("Parser write_to_db end");
        Ok(())
    }
}
