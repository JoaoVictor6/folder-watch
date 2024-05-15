pub mod git;
use notify::{event::CreateKind, RecursiveMode, Watcher};
use std::{path::Path, sync::mpsc::channel, time::Duration};
use chrono::prelude::*;

fn get_file_or_folder_name (path: &str) -> &str {
    path.split('/').filter(|s| !s.is_empty()).last().unwrap_or("")
}

fn create_commit_date () -> String {
    let local: DateTime<Local> = Local::now();
    let day = local.day();
    let month = local.month();
    let year = local.year();

    let hour = local.hour();
    let minutes = local.minute();

    return format!("[{}/{}/{} {}:{}] - ", month, day, year, hour, minutes);
}


fn commit_creation_event (event_kind: CreateKind, file_source: &str) {
    let commit_prefix = create_commit_date();
    let commit_message: String;
    match event_kind {
        CreateKind::Any => todo!(),
        CreateKind::File => commit_message = format!("create new file called {}", get_file_or_folder_name(file_source)),
        CreateKind::Folder => commit_message = format!("create new file called {}", get_file_or_folder_name(file_source)),
        CreateKind::Other => todo!(),
    }

    git::commit_and_push(format!("{}{}", commit_prefix, commit_message).as_str(), file_source);
    println!("Git command runned!!!");
}

fn main() {
    let (tx, rx) = channel();
    
    let mut watcher = notify::recommended_watcher(move |res| {
        let _ = tx.send(res);
    }).unwrap();

    let _ = watcher.watch(Path::new(".."), RecursiveMode::Recursive);
    
    loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(event) => {
                let event = event.unwrap();
                println!("Event attributes: {:?}", event.info());
                match event.kind {
                    notify::EventKind::Create(ev) => {
                        commit_creation_event(ev, event.paths[0].to_str().unwrap());
                    },
                    notify::EventKind::Any => {
                        println!("Event attributes: {:?}", event.info())
                    }
                    notify::EventKind::Access(_) => {},
                    notify::EventKind::Modify(ev) => println!("MODIFY EVENT: {:?}", ev),
                    notify::EventKind::Remove(ev) => println!("REMOVE EVENT: {:?}", ev),
                    notify::EventKind::Other => todo!(),
                }
            }
            Err(_) => {} 
        }
    }
}
