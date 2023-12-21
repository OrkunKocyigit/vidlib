use std::path::{Path, PathBuf};

use serde::Serialize;
use slab_tree::{NodeId, Tree};
use tauri::{AppHandle, Manager};
use tokio::sync::mpsc::Receiver;

use crate::state::AppState;

#[derive(Clone, Serialize, Debug)]
pub struct Folder {
    path: String,
}

impl Folder {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

pub fn scan_folder<P: AsRef<Path>>(path: P) {}

#[derive(Clone, Serialize)]
pub struct FolderInfoEmitEvent {
    media_info: Folder,
}

impl FolderInfoEmitEvent {
    pub fn new(media_info: Folder) -> Self {
        Self { media_info }
    }
}

pub(crate) async fn process_folder_output_channels(
    app: &AppHandle,
    mut folder_output_rx: Receiver<PathBuf>,
) -> Result<(), anyhow::Error> {
    debug!("Folder output channel started");
    let state = app.state::<AppState>();
    while let Some(input) = folder_output_rx.recv().await {
        debug!("Folder output message received");
        let mut tree_mutex = state.folders.lock().unwrap();
        let tree = tree_mutex.as_mut().unwrap();
        let node_id = get_tree_node(tree, &input);
        debug!("Media info output message send for: {}", input.display());
    }

    Ok(())
}

fn get_tree_node<P: AsRef<Path>>(tree: &mut Tree<Folder>, path: P) -> NodeId {
    let mut root_id = tree.root_id().unwrap();
    for p in path.as_ref().display().to_string().split('\\') {
        let new_root_id = match tree
            .get(root_id)
            .unwrap()
            .children()
            .find(|n| n.data().path == p)
        {
            None => {
                let new_folder = Folder::new(p.into());
                tree.get_mut(root_id).unwrap().append(new_folder).node_id()
            }
            Some(node) => node.node_id(),
        };
        root_id = new_root_id;
    }
    root_id
}
