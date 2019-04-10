fn main() -> Result<()> {
  let mut matches = app_from_crate!()
  .arg(Arg::with_name("interval")
    .short("i")
    .long("interval")
    .value_name("INTERVAL")
    .help("Interval between knocks in seconds")
    .default_value("1")
    .takes_value(true))
  .arg(Arg::with_name("host")
    .value_name("HOST")
    .help("Host to knock at")
    .index(1))
  .arg(Arg::with_name("port")
    .value_name("PORT")
    .help("Port to knock")
    .index(2)
    .multiple(true))
  .get_matches();
  .get_matches();
  
  let mut interval = value_t!(matches, "interval", u32).unwrap_or(1);
  let mut ports = 
}