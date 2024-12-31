use crate::{file_manager::FileManager, xml_handler::XmlHandler, config::Config};
use eframe::egui;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use crate::xml_handler::XmlValue;

#[derive(PartialEq)]
enum ViewState {
    Categories,
    FileList(String),  // Holds the category name
}

pub struct ModManagerApp {
    file_manager: Arc<Mutex<FileManager>>,
    xml_handler: XmlHandler,
    config: Config,
    selected_file: Option<PathBuf>,
    current_value: String,
    xml_values: Option<Vec<XmlValue>>,
    view_state: ViewState,
}

impl ModManagerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let config = Config::load().unwrap_or_default();
        Self {
            file_manager: Arc::new(Mutex::new(FileManager::new(&config.game_path))),
            xml_handler: XmlHandler::new(),
            config,
            selected_file: None,
            current_value: String::new(),
            xml_values: None,
            view_state: ViewState::Categories,
        }
    }

    fn render_categories(&mut self, ui: &mut egui::Ui) {
        ui.heading("Categories");

        ui.add_space(10.0);
        
        // Grid for categories with icons/images
        egui::Grid::new("categories_grid")
            .spacing([20.0, 20.0])
            .show(ui, |ui| {
                // Characters category
                if ui.button("Characters").clicked() {
                    self.view_state = ViewState::FileList("Characters".to_string());
                }
                ui.label("Edit character stats and abilities");
                ui.end_row();

                // Levels category
                if ui.button("Levels TODO").clicked() {
                    self.view_state = ViewState::FileList("Levels".to_string());
                }
                ui.label("Modify level layouts and properties");
                ui.end_row();

                // Guns category
                if ui.button("Guns TODO").clicked() {
                    self.view_state = ViewState::FileList("Guns".to_string());
                }
                ui.label("Adjust weapon stats and behavior");
                ui.end_row();

                // Bullets category
                if ui.button("Bullets").clicked() {
                    self.view_state = ViewState::FileList("Bullets".to_string());
                }
                ui.label("Adjust bullet stats and behavior");
                ui.end_row();
            });
    }

    fn render_file_list(&mut self, category: &str, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("<- Back to Categories").clicked() {
                self.view_state = ViewState::Categories;
                self.selected_file = None;
                self.xml_values = None;
            }

            ui.heading(category);

            // Add New button for Characters category
            if category == "Characters" {
                ui.add_space(10.0);

                // Simply adding a new character file to this directory doesn't actually add a new character :P
                // if ui.button("+ New Character").clicked() {
                //     match self.file_manager.lock().unwrap().create_new_character(category) {
                //         Ok(new_file) => {
                //             self.selected_file = Some(new_file);
                //             self.xml_values = None;
                //         }
                //         Err(err) => {
                //             eprintln!("Error creating new character: {:?}", err);
                //             // TODO: Show error in UI
                //         }
                //     }
                // }

                ui.add_space(10.0);
                if ui.button("🔓 Unlock All Characters").clicked() {
                    if let Ok(files) = self.file_manager.lock().unwrap().get_category_files(category) {
                        for file in files {
                            // Load each character file
                            if let Ok(mut values) = self.xml_handler.load_file(&file) {
                                // Find and modify PointsToUnlock value
                                if let Some(value) = values.iter_mut().find(|v| v.name == "PointsToUnlock") {
                                    value.value = "0".to_string();
                                    // Save changes back to file
                                    if let Err(err) = self.xml_handler.save_changes(&file, &values) {
                                        eprintln!("Error saving changes to {:?}: {:?}", file, err);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });

        ui.add_space(10.0);

        // File list for the selected category
        egui::ScrollArea::vertical()
            .id_source("file_list_scroll")
            .max_height(200.0)
            .show(ui, |ui| {
                if let Ok(files) = self.file_manager.lock().unwrap().get_category_files(category) {
                    for file in files {
                        if ui.button(file.file_name().unwrap().to_string_lossy().to_string()).clicked() {
                            self.selected_file = Some(file.clone());
                            self.xml_values = None;
                        }
                    }
                }
            });
    }
}

impl eframe::App for ModManagerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
        
        egui::ScrollArea::vertical()
        .id_source("main_scroll")
        .show(ui, |ui| {
            ui.heading("Game Mod Manager");
            
            ui.horizontal(|ui| {
                if ui.button("Select Game Directory").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.config.game_path = path;
                        self.config.save().ok();
                        *self.file_manager.lock().unwrap() = FileManager::new(&self.config.game_path);
                    }
                }
                ui.label("EX: D:\\Steam\\steamapps\\common\\Zombie Estate 2\\Data");
            });

            ui.add_space(20.0);
            
            // self.render_file_list(ui);
            let category = match &self.view_state {
                ViewState::FileList(category) => Some(category.clone()),
                ViewState::Categories => None,
            };
            // Render based on view state
            if category.is_some() {
                self.render_file_list(&category.unwrap(), ui);
            } else {
                self.render_categories(ui);
            }
            
            ui.add_space(10.0);

            if let Some(selected_file) = &self.selected_file {
                ui.group(|ui| {
                    ui.heading("XML Editor");
                    
                    ui.horizontal(|ui| {
                        if ui.button("Load XML").clicked() {
                            match self.xml_handler.load_file(selected_file) {
                                Ok(values) => {
                                    self.xml_values = Some(values);
                                }
                                Err(err) => {
                                    eprintln!("Error loading XML: {:?}", err);
                                }
                            }
                        }
                        
                        ui.label(format!("Selected: {}", selected_file.to_string_lossy()));
                    });
                    
                    // Show values table if loaded
                    if let Some(values) = &mut self.xml_values {
                        egui::ScrollArea::vertical()
                            .id_source("xml_values_scroll")
                            .max_height(300.0)
                            .show(ui, |ui| {
                                // Add table headers
                                egui::Grid::new("xml_grid")
                                    .num_columns(3)
                                    .spacing([40.0, 4.0])
                                    .striped(true)
                                    .show(ui, |ui| {
                                        ui.label("Section");
                                        ui.label("Name");
                                        ui.label("Value");
                                        ui.end_row();

                                        for value in values.iter_mut() {
                                            ui.label(&value.path);
                                            ui.label(&value.name);
                                            let mut current_value = value.value.clone();
                                            if ui.text_edit_singleline(&mut current_value).changed() {
                                                value.value = current_value;
                                                // TODO: Implement saving changes
                                                if let Err(e) = self.xml_handler
                                                    .modify_value(&value.path, &value.name, &value.value) {
                                                    eprintln!("Error modifying value: {:?}", e);
                                                }
                                            }
                                            ui.end_row();
                                        }
                                    });
                            });
                    }
                    
                    if ui.button("💾 Save Changes").clicked() {
                        if let Some(values) = &self.xml_values {
                            match self.xml_handler.save_changes(selected_file, values) {
                                Ok(_) => {
                                    // Show success message
                                    ui.label("✅ Changes saved successfully!");
                                }
                                Err(err) => {
                                    eprintln!("Error saving changes: {:?}", err);
                                    // Show error message in UI
                                    ui.label("❌ Failed to save changes");
                                }
                            }
                        }
                    }
                });
            }

            ui.add_space(10.0);

            
        });
        });
    }
}