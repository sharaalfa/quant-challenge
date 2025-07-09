use std::fs::File;
use std::path::Path;
use crate::domain::OrderbookSnapshot;

pub struct Saver;

impl Saver {
    pub fn save_snapshots(snapshots: &[OrderbookSnapshot], path: &Path) -> anyhow::Result<()>{
        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, snapshots)?;
        Ok(())
    }
    
    pub fn save_deltas(deltas: &[f64], path: &Path) -> anyhow::Result<()> {
        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, deltas)?;
        Ok(())
    }
}