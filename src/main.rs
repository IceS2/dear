use clap::StructOpt;
use dear::config::{BackendConfig, FileBackendConfig};
use dear::repository;
use dear::repository::Repository;

fn main() {
    let cli = dear::Cli::parse();

    let config = dear::config::DearConfig::default();
    println!("{config:?}");

    println!("{:?}", cli.profile);
    let profile = config.profiles.get(&cli.profile).unwrap();
    println!("{profile:?}");

    match &profile.backend {
        BackendConfig::File(FileBackendConfig { path }) => {
            run_with_repo(cli, repository::FileRepository::new(path));
        }
        BackendConfig::InMemory => {
            run_with_repo(cli, repository::InMemoryRepository::default());
        }
    }
}

fn run_with_repo<R: Repository>(cli: dear::Cli, mut repo: R) {
    match &cli.action {
        dear::Action::Save {
            title,
            description,
            tags,
        } => {
            let req = dear::create_request(title, description.as_deref(), tags.as_ref());
            let res = dear::create_note::execute(&mut repo, &req).unwrap();
            println!("{res:?}");
        }
        dear::Action::List => {
            println!("List");
            println!("{repo:?}");
            let res = dear::list_notes::execute(&repo)
                .map(Vec::from_iter)
                .unwrap();

            for note in res {
                println!("{note}");
            }
        }
    }
}
