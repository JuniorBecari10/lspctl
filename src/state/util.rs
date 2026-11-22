use std::{
    iter::{self},
    path::PathBuf,
};

use crate::state::Executables;

// this iters over Executables' two fields
impl Executables {
    pub fn iter(&self) -> impl Iterator<Item = &PathBuf> {
        iter::once(&self.0).chain(self.1.iter())
    }
}
