use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;
use termcolor::WriteColor;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    command: Subcommand,
}

#[derive(Debug, StructOpt)]
#[structopt(no_version)]
enum Subcommand {
    State {
        #[structopt(name = "container-id")]
        id: String,
    },

    Create {
        #[structopt(name = "container-id")]
        id: String,

        #[structopt(name = "path-to-bundle", parse(from_os_str))]
        bundle_path: PathBuf,
    },

    Start {
        #[structopt(name = "container-id")]
        id: String,
    },

    Kill {
        #[structopt(name = "container-id")]
        id: String,

        #[structopt(name = "signal", default_value = "SIGTERM")]
        signal: String,
    },

    Delete {
        #[structopt(name = "container-id")]
        id: String,
    },
}

fn main() {
    let _ = Opt::from_args();

    write_error("not implemented");
    std::process::exit(1);
}

fn write_error(message: &str) {
    let mut stderr = termcolor::StandardStream::stderr(termcolor::ColorChoice::Auto);
    stderr
        .set_color(termcolor::ColorSpec::new().set_fg(Some(termcolor::Color::Red)))
        .unwrap();
    write!(&mut stderr, "error: ").unwrap();

    stderr.set_color(&termcolor::ColorSpec::new()).unwrap();

    eprintln!("{}", message);
}
