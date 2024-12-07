
use structopt::StructOpt;

/// Configuration for the web crawler, parsed from command-line arguments.
#[derive(StructOpt, Debug)]
pub struct Config {
    /// The starting URL for the web crawler.
    #[structopt(short, long)]
    pub start_url: String,

    /// The maximum depth to crawl. Defaults to 3 if not specified.
    #[structopt(short, long, default_value = "3")]
    pub depth_limit: usize,

    /// The maximum number of pages to crawl. Optional.
    #[structopt(short, long)]
    pub max_pages: Option<usize>,

    /// Flag to restrict the crawl to the starting domain only.
    #[structopt(long)]
    pub same_domain: bool,
}

impl Config {
    /// Parses command-line arguments and returns a `Config` instance.
    pub fn from_args() -> Self {
        StructOpt::from_args()
    }
}
