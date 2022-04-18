use clap::StructOpt;
use dear;

fn main() {
//     let cli = dear::Cli::parse();

//     let mut repo = dear::create_repo();

//     match &cli.action {
//         dear::Action::Save { title, description } => {
//             let req = dear::create_request(title, description.as_deref());
//             let res = dear::create_note::execute(&mut repo, &req).unwrap();
//             println!("{res:?}");
//         }
//     }
    dear::test();
}
