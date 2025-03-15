use crate::canvas::Mode;

#[derive(Debug)]
pub struct Args {
    pub width: usize,
    pub height: usize,
    pub mode: Mode,
    pub x_is_row: bool,
    pub log_x: bool,
    pub log_y: bool,
    pub cdf: bool,
}

impl Args {
    pub fn from_env() -> Result<Self, pico_args::Error> {
        let mut pargs = pico_args::Arguments::from_env();

        if pargs.contains(["-h", "--help"]) {
            println!(
                "\
Terminal Plotter

USAGE:
    plotter [OPTIONS]

OPTIONS:
    -d <WxH>             Set dimensions in columns x rows (default: 90x25)
    --dot                Use Dot mode instead of Count mode
    --no-x-is-row        Treat row number as the y-value
    --log-x              Apply log10 transform to X axis
    --log-y              Apply log10 transform to Y axis
    --cdf                Plot cumulative distribution function
    -h, --help           Show help message
"
            );
            std::process::exit(0);
        }

        // Parse dimensions from -d
        let dims: Option<String> = pargs.opt_value_from_str("-d")?;
        let (width, height) =
            parse_dimensions(dims.as_deref().unwrap_or("90x25")).unwrap_or_else(|| {
                eprintln!("Invalid dimensions format! Use <width>x<height>, e.g., 72x30");
                std::process::exit(1);
            });

        let x_is_row: bool = !pargs.contains("--no-x-is-row");

        let args = Self {
            width,
            height,
            mode: if pargs.contains("--dot") {
                Mode::Dot
            } else {
                Mode::Count
            },
            x_is_row,
            log_x: pargs.contains("--log-x"),
            log_y: pargs.contains("--log-y"),
            cdf: pargs.contains("--cdf"),
        };

        eprintln!("{:?}", args);
        Ok(args)
    }
}

fn parse_dimensions(s: &str) -> Option<(usize, usize)> {
    // Remove all whitespace first
    let s = s.replace(char::is_whitespace, "");

    let mut parts = s.split('x');
    let width = parts.next()?.parse().ok()?;
    let height = parts.next()?.parse().ok()?;

    Some((width, height))
}
