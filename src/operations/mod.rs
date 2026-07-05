pub mod model;

pub fn install(args: model::InstallArgs) {
    for pkg in args.pkgs {
        println!("{pkg}");
    }
}

pub fn remove(args: model::RemoveArgs) {
    for pkg in args.pkgs {
        println!("{pkg}");
    }
}
