use std::path::PathBuf;

use crate::state::Executables;

// this iters over Executables' two fields
impl Executables {
    pub fn iter(&self) -> impl Iterator<Item = &PathBuf> {
        std::iter::once(&self.bin).chain(self.links.iter())
    }
}
