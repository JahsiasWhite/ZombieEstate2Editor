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
                if ui.button("Levels").clicked() {
                    self.view_state = ViewState::FileList("Levels".to_string());
                }
                ui.label("Modify level layouts and properties");
                ui.end_row();

                // Guns category
                if ui.button("Guns").clicked() {
                    self.view_state = ViewState::FileList("Guns".to_string());
                }
                ui.label("Adjust weapon stats and behavior");
                ui.end_row();
            });
    }

    fn render_file_list(&mut self, category: &str, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("← Back to Categories").clicked() {
                self.view_state = ViewState::Categories;
                self.selected_file = None;
                self.xml_values = None;
            }
            ui.heading(category);
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
            ui.horizontal(|ui| {
                ui.heading("Game Mod Manager");
                if ui.button("Select Game Directory").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.config.game_path = path;
                        self.config.save().ok();
                        *self.file_manager.lock().unwrap() = FileManager::new(&self.config.game_path);
                    }
                }
            });

            ui.add_space(10.0);
            
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
                });
            }

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if ui.button("Create Backup").clicked() {
                    self.file_manager.lock().unwrap().create_backup().ok();
                }
                if ui.button("Restore Backup").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .set_directory(&self.file_manager.lock().unwrap().get_backup_path())
                        .pick_folder() 
                    {
                        self.file_manager.lock().unwrap().restore_backup(&path).ok();
                    }
                }
            });
        });
        });
    }
}