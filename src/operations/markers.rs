use crate::operations::{model, util::PackageSelection};

pub trait Selection {
    fn pkgs(&self) -> &[String];
    fn all(&self) -> bool;

    fn to_package_selection(&self) -> Option<PackageSelection> {
        if !self.all() && self.pkgs().is_empty() {
            return None;
        }

        if self.all() {
            Some(PackageSelection::All)
        } else {
            Some(PackageSelection::Specific(self.pkgs().to_vec()))
        }
    }
}

impl Selection for model::RemoveArgs {
    fn pkgs(&self) -> &[String] {
        &self.pkgs
    }

    fn all(&self) -> bool {
        self.all
    }
}

impl Selection for model::UpdateRegistryArgs {
    fn pkgs(&self) -> &[String] {
        &self.pkgs
    }

    fn all(&self) -> bool {
        self.all
    }
}
