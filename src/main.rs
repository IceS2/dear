use clap::StructOpt;

fn main() {
    let cli = dear::Cli::parse();

    let mut repo = dear::create_repo();

    let _initial_note = dear::create_note::execute(
        &mut repo,
        &dear::create_request("Initial Test", Some("Desc")),
    );

    let _initial_note2 = dear::create_note::execute(
        &mut repo,
        &dear::create_request("Second Test Override", None),
    );

    match &cli.action {
        dear::Action::Save { title, description } => {
            let req = dear::create_request(title, description.as_deref());
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
