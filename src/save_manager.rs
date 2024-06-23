use bevy::{prelude::*, tasks::{AsyncComputeTaskPool, Task}};
use crate::cell::{CellCoordinates,CellState,CellBundle};
use futures_lite::future;

use std::{fs, path::PathBuf};
use rfd::FileDialog;

pub struct SavePlugin;

impl Plugin for SavePlugin{
    fn build(&self, app: &mut App) {
        app
        .insert_state(FileManipulation::None)
        .add_systems(Update, keyboard_input)
        .add_systems(Update, dialog.run_if(in_state(FileManipulation::DialogSave)))
        .add_systems(Update, dialog.run_if(in_state(FileManipulation::DialogLoad)))
        .add_systems(Update, create_save.run_if(in_state(FileManipulation::Save)))
        .add_systems(Update, load_save.run_if(in_state(FileManipulation::Load)))
        .add_systems(Update, poll);
        // .add_systems(Update, poll);
    }
}

fn keyboard_input(buttons: Res<ButtonInput<KeyCode>>,mut manipulation_type:ResMut<NextState<FileManipulation>>){
    if buttons.pressed(KeyCode::ControlLeft){
        if buttons.just_pressed(KeyCode::KeyL){
            manipulation_type.set(FileManipulation::DialogLoad);
        }
        else if buttons.just_pressed(KeyCode::KeyS) {
            manipulation_type.set(FileManipulation::DialogSave);
        }
    }
}
/// What is happening with file
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum FileManipulation{
    #[default]
    None,
    DialogLoad,
    Load,
    DialogSave,
    Save,
}

fn create_save(
    mut manipulation_type:ResMut<NextState<FileManipulation>>,
    mut commands: Commands,
    cells_q: Query<(&CellState,&CellCoordinates)>,
    path_q: Query<(Entity,&SelectedPath)>
) {
    for (entity,path) in &path_q{
        match &path.0 {
            Some(path) => {
                // create empty file contents
                let mut contents= String::new();
                // write a record for each living cell
                for (state,pos) in &cells_q{
                    if let CellState::Alive = state {
                        contents.push_str(format!("{} {}\n",pos.0.x,pos.0.y).as_str());
                    }
                }
                let _ = fs::write(path, contents);

            },
            None => info!("no file selected"),
        }
        // remove file path entity
        commands.entity(entity).despawn();
        manipulation_type.set(FileManipulation::None);
    }
}

/// Clears cells and loads save
fn load_save(
    mut manipulation_type:ResMut<NextState<FileManipulation>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    path_q: Query<(Entity,&SelectedPath)>,
    cells_q: Query<Entity,With<CellState>>,
) {
    for (entity,path) in &path_q{
        match &path.0 {
            Some(path) => {
                // clear game from all cells
                for cell in &cells_q{
                    commands.entity(cell).despawn();
                }

                // read file and spawn cells using data from it
                for line in fs::read_to_string(path).unwrap_or("a".to_string()).lines(){
                    let coords:Vec<&str> = line.split_terminator(" ").collect();
                    let coords = Vec2 { x:coords[0].parse().unwrap_or(0.), y:coords[1].parse().unwrap_or(0.) };
                    commands.spawn(CellBundle::from_coords(coords, &asset_server));
                }
            },
            None => info!("no file selected"),
        }
        // remove file path entity
        commands.entity(entity).despawn();
        manipulation_type.set(FileManipulation::None);
    }

    
}


/// Selected file path used for loading/saving
#[derive(Component)]
struct SelectedFile(Task<Option<PathBuf>>);

/// Selected file path used for polling
#[derive(Component)]
struct SelectedPath(Option<PathBuf>);

/// Spawns dialog window to select/create file
fn dialog(mut commands: Commands,mut manipulation_type_next:ResMut<NextState<FileManipulation>>,manipulation_type:ResMut<State<FileManipulation>>) {
    let thread_pool = AsyncComputeTaskPool::get();
    let task = match manipulation_type.get() {
        FileManipulation::DialogLoad => {
            manipulation_type_next.set(FileManipulation::Load);
            thread_pool.spawn(async move {
                FileDialog::new().pick_file()
            })
        },
        FileManipulation::DialogSave => {
            manipulation_type_next.set(FileManipulation::Save);
            thread_pool.spawn(async move {
                FileDialog::new().save_file()
            })
        },
        _ => return,
    };
    commands.spawn(SelectedFile(task));
}

/// Polls for selected file
fn poll(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut SelectedFile)>,
) {
    for (entity, mut selected_file) in tasks.iter_mut() {
        if let Some(result) = future::block_on(
            future::poll_once(&mut selected_file.0)
        ) {
            commands.entity(entity).remove::<SelectedFile>();
            commands.spawn(SelectedPath(result));
        }
    }
}