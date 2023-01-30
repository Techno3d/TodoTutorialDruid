use std::{path::Path, fs};

use directories::BaseDirs;
use druid::Widget;
use serde::{Serialize, Deserialize};

use crate::data::{TodoState, TodoItem};

pub struct Saver;

impl Widget<TodoState> for Saver {
    fn event(&mut self, _ctx: &mut druid::EventCtx, _event: &druid::Event,_data: &mut TodoState, _env: &druid::Env) {
    }

    fn lifecycle(&mut self, _ctx: &mut druid::LifeCycleCtx, _event: &druid::LifeCycle, _data: &TodoState, _env: &druid::Env) {
    }

    fn update(&mut self, _ctx: &mut druid::UpdateCtx, old_data: &TodoState, data: &TodoState, _env: &druid::Env) {
        if data.todos != old_data.todos {
            if let Some(base_dirs) = BaseDirs::new() {
                let config = format!("{}/{}", base_dirs.config_dir().to_str().unwrap(), "MyTodo.json");
                let config_path = Path::new(&config);
                let tasks = TaskData {tasks: data.todos.clone().into_iter().collect() };
                fs::write(config_path, serde_json::to_string(&tasks).unwrap()).expect("Config path does not fully exist");
            }
        }
    }

    fn layout(&mut self, _ctx: &mut druid::LayoutCtx, _bc: &druid::BoxConstraints, _data: &TodoState, _env: &druid::Env) -> druid::Size {
        druid::Size { width: 0., height: 0. }
    }

    fn paint(&mut self, _ctx: &mut druid::PaintCtx, _data: &TodoState, _env: &druid::Env) {
    }
}

#[derive(Serialize, Deserialize)]
pub struct TaskData {
    pub tasks: Vec<TodoItem>,
}

pub fn read_stored() -> TaskData {
    if let Some(base_dirs) = BaseDirs::new() {
        let config = format!("{}/{}", base_dirs.config_dir().to_str().unwrap(), "MyTodo.json");
        let config_path = Path::new(&config);
        let data = match fs::read_to_string(config_path) {
            Ok(a) => a,
            Err(_) => return TaskData {tasks: Vec::new()},
        };
        match serde_json::from_str(&data) {
            Ok(a) => a,
            Err(e) => {
                eprintln!("The save data is corrupted or no longer in the format it should be in\nError {}", e);
                return TaskData { tasks: Vec::new() };
            },
        }
    } else {
        return TaskData { tasks: Vec::new() };
    }
}
