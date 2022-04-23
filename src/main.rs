use clap::StructOpt;

fn main() {
    let cli = dear::Cli::parse();

    let mut repo = dear::create_repo();

    match &cli.action {
        dear::Action::Save {
            title,
            description,
            tags,
        } => {
            println!("{tags:?}");
            let req = dear::create_request(title, description.as_deref(), tags.as_ref());
            let res = dear::create_note::execute(&mut repo, &req).unwrap();
            println!("{res:?}");
        }
        dear::Action::List => {
            let res = dear::list_notes::execute(&mut repo);
            println!("{res:?}");
        }
    }
    // dear::test();
}
