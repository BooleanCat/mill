use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;
use termcolor::WriteColor;

mod config;

#[derive(Debug, StructOpt)]
#[structopt(about = "An OCI-compliant container runtime")]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    config: Option<PathBuf>,

    #[structopt(subcommand)]
    command: Subcommand,
}

#[derive(Debug, StructOpt)]
#[structopt(no_version)]
enum Subcommand {
    #[structopt(about = "Query container state")]
    State {
        #[structopt(name = "container-id")]
        id: String,
    },

    #[structopt(about = "Create container from bundle")]
    Create {
        #[structopt(name = "container-id")]
        id: String,

        #[structopt(name = "path-to-bundle", parse(from_os_str))]
        bundle_path: PathBuf,
    },

    #[structopt(about = "Start created container")]
    Start {
        #[structopt(name = "container-id")]
        id: String,
    },

    #[structopt(about = "Kill container with signal")]
    Kill {
        #[structopt(name = "container-id")]
        id: String,

        #[structopt(name = "signal", default_value = "SIGTERM")]
        signal: String,
    },

    #[structopt(about = "Delete container")]
    Delete {
        #[structopt(name = "container-id")]
        id: String,
    },
}

fn main() {
    let opt = Opt::from_args();

    let _ = match opt.config {
        None => config::Config::load(),
        Some(path) => config::Config::from_file(path),
    }
    .unwrap_or_else(|error| {
        write_error(&format!("failed to load mill config: {:?}", error));
        std::process::exit(1);
    });

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
