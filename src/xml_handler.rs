use quick_xml::events::{Event, BytesStart};
use quick_xml::Reader;
use quick_xml::Writer;
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct XmlValue {
    pub path: String,      // Full path to the value (e.g., "Properties/Speed")
    pub name: String,      // Name of the field (e.g., "Speed")
    pub value: String,     // The actual value (e.g., "3.60")
}

pub struct XmlHandler {
}

impl XmlHandler {
    pub fn new() -> Self {
        Self {
            // reader: Reader::from_file("/dev/null").unwrap(),
            
        }
    }

    pub fn load_file(&mut self, path: &Path) -> Result<Vec<XmlValue>> {
        let mut reader = Reader::from_file(path)?;
        reader.trim_text(true);
        
        let mut buf = Vec::new();
        let mut values = Vec::new();
        let mut current_path = Vec::new();
        
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Eof) => break,
                Ok(Event::Start(e)) => {
                    let name = std::str::from_utf8(e.name().as_ref())?.to_string();
                    current_path.push(name);
                },
                Ok(Event::Text(e)) => {
                    if !current_path.is_empty() {
                        let text = e.unescape()?.into_owned();
                        // Only add if the text isn't just whitespace
                        if !text.trim().is_empty() {
                            values.push(XmlValue {
                                path: current_path[..current_path.len()-1].join("/"),
                                name: current_path.last().unwrap().clone(),
                                value: text,
                            });
                        }
                    }
                },
                Ok(Event::End(_)) => {
                    current_path.pop();
                },
                Err(e) => return Err(e.into()),
                _ => (),
            }
        }
        Ok(values)
    }

    pub fn modify_value(&mut self, path: &str, name: &str, new_value: &str) -> Result<()> {
        // TODO: Implement saving changes
        Ok(())
    }
}