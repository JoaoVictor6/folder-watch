mod git;
use chrono::prelude::*;
use notify::{
    event::{CreateKind, ModifyKind, RemoveKind, RenameMode},
    Event, RecursiveMode, Watcher,
};
use std::{path::Path, sync::mpsc::channel, time::Duration};

struct CommitMessages {
    last_message: String
}

impl CommitMessages {
    fn set_last_message(&mut self, commit_message: String) {
        self.last_message = commit_message;
    }
}

fn get_file_or_folder_name(path: &str) -> &str {
    path.split('/')
        .filter(|s| !s.is_empty())
        .last()
        .unwrap_or("")
}

fn create_commit_date() -> String {
    let local: DateTime<Local> = Local::now();
    let day = local.day();
    let month = local.month();
    let year = local.year();

    let hour = local.hour();
    let minutes = local.minute();

    return format!("[{}/{}/{} {}:{}] - ", month, day, year, hour, minutes);
}

fn commit_modify_event(event_kind: ModifyKind, event: Event) {
    let commit_prefix = create_commit_date();
    let current_path = event.paths[0].to_str().unwrap();

    if let ModifyKind::Data(_) = event_kind {
        let commit_message = format!("edit {}", get_file_or_folder_name(current_path));
        git::commit_and_push(
            format!("{}{}", commit_prefix, commit_message).as_str(),
            current_path,
        );
    }
    if ModifyKind::Name(RenameMode::Both) == event_kind {
        let new_path = event.paths[1].to_str().unwrap();
        let commit_message = format!(
            "rename {} to {}",
            get_file_or_folder_name(current_path),
            get_file_or_folder_name(new_path)
        );
        git::commit_and_push(
            format!("{}{}", commit_prefix, commit_message).as_str(),
            new_path,
        );
    }
}

fn commit_create_event(event_kind: CreateKind, file_source: &str) {
    let commit_prefix = create_commit_date();
    if event_kind == CreateKind::File {
        let commit_message = format!(
            "create new file called {}",
            get_file_or_folder_name(file_source)
        );
        git::commit_and_push(
            format!("{}{}", commit_prefix, commit_message).as_str(),
            file_source,
        );
    }
}

fn commit_remove_event(event_kind: RemoveKind, file_source: &str) {
    let commit_prefix = create_commit_date();
    let file_or_folder_name = get_file_or_folder_name(file_source);
    if event_kind == RemoveKind::File {
        let commit_message = format!("delete {} file", file_or_folder_name);
        git::commit_and_push(
            format!("{}{}", commit_prefix, commit_message).as_str(),
            file_source,
        );
    }
}

fn main() {
    let (tx, rx) = channel();
    let mut commit_messages = CommitMessages::new();
    let mut watcher = notify::recommended_watcher(move |res| {
        let _ = tx.send(res);
    })
    .unwrap();

    let _ = watcher.watch(Path::new(".."), RecursiveMode::Recursive);

    loop {
        match rx.recv_timeout(Duration::from_secs(30)) {
            Ok(event) => {
                let event = event.unwrap();
                let path = event.paths[0].to_str().unwrap();
                // ignore files/folder change on .git folder
                if path.contains("/.git") {
                    continue;
                }
                if let notify::EventKind::Create(ev) = event.kind {
                    commit_create_event(ev, path);
                }
                if let notify::EventKind::Modify(ev) = event.kind {
                    commit_modify_event(ev, event.clone())
                }
                if let notify::EventKind::Remove(ev) = event.kind {
                    commit_remove_event(ev, path)
                }
            }
            Err(_) => {}
        }
    }
}
