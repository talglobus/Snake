extern crate clap;

lazy_static! {
	pub static ref MATCHES: clap::ArgMatches<'static> = {		// TODO: De-`pub`ify this
        clap::App::new("snake")
		.version("1.0")
		.author("Tal Globus <talglobus@gmail.com>")
		.about(
		"Learning to Rust while making a classic game worthy of your childhood Nokia brick phone")
		.arg(clap::Arg::with_name("text")
			.conflicts_with("gui")			// Text mode and GUI mode are mutually exclusive
			.short("t")
			.long("text")
			.help("Enables text mode rather than GUI mode")
			.takes_value(false))
		.arg(clap::Arg::with_name("gui")
			.conflicts_with("visible")		// GUI mode *always* has visibility
			.short("g")
			.long("gui")
			.help("Enables GUI mode rather than text mode (default)")
			.takes_value(false))
		.arg(clap::Arg::with_name("visible")
			.short("v")
			.long("visible")
			.help("Enables visibility in text mode")
			.takes_value(false))
		.arg(clap::Arg::with_name("debug")
			.conflicts_with("text")			// Debug data interferes with text state output
			.short("d")
			.long("debug")
			.visible_alias("verbose")
			.help("Print verbose debug information")
			.takes_value(false))
		.get_matches()
	};
}

lazy_static! {
    pub static ref IS_GUI: bool = {
		!MATCHES.is_present("text")
    };
}

lazy_static! {
    pub static ref IS_VISIBLE: bool = {
		MATCHES.is_present("visible")
    };
}

lazy_static! {
    pub static ref IS_VERBOSE: bool = {
		MATCHES.is_present("debug")
    };
}


