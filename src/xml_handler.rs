use quick_xml::{events::{Event, BytesStart, BytesText}, Writer, Reader};
use std::path::Path;
use std::fs::{self, File};
use std::io::{Write, Seek, SeekFrom};
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

    pub fn save_changes(&self, path: &Path, values: &[XmlValue]) -> Result<()> {
        // Read the original file content
        let content = fs::read_to_string(path)?;
        
        // Create a new XML reader
        let mut reader = Reader::from_str(&content);
        reader.trim_text(true);
        
        // Create a string buffer for the output
        let mut writer_buf = Vec::new();
        let mut writer = Writer::new(&mut writer_buf);
        
        let mut buf = Vec::new();
        let mut current_path = Vec::new();
        
        // Process the XML document
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Eof) => break,
                Ok(Event::Start(e)) => {
                    let name = std::str::from_utf8(e.name().as_ref())?.to_string();
                    current_path.push(name.clone());
                    writer.write_event(Event::Start(e))?;
                },
                Ok(Event::Text(e)) => {
                    if !current_path.is_empty() {
                        let current_name = current_path.last().unwrap();
                        let current_full_path = current_path[..current_path.len()-1].join("/");
                        
                        // Check if this text node should be updated
                        if let Some(value) = values.iter().find(|v| 
                            v.name == *current_name && v.path == current_full_path
                        ) {
                            // Write the new value instead of the original
                            writer.write_event(Event::Text(BytesText::new(&value.value)))?;
                        } else {
                            writer.write_event(Event::Text(e))?;
                        }
                    }
                },
                Ok(Event::End(e)) => {
                    writer.write_event(Event::End(e))?;
                    current_path.pop();
                },
                Ok(e) => {
                    writer.write_event(e)?;
                },
                Err(e) => return Err(e.into()),
            }
        }
        
        // Write the updated content back to the file
        let mut file = File::create(path)?;
        file.write_all(&writer_buf)?;
        
        Ok(())
    }

    pub fn modify_value(&mut self, path: &str, name: &str, new_value: &str) -> Result<()> {
        // TODO: Implement saving changes
        Ok(())
    }
}