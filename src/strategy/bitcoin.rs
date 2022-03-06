///
///
#[derive(Debug)]
pub struct BuildOption<'flag_option> {
    flag: &'flag_option str,
    enabled: bool,
}

impl<'flag_option> BuildOption<'flag_option> {
    pub fn new(flag: &'flag_option str, enabled: bool) -> Self {
        Self { flag, enabled }
    }

    pub fn flag(&self) -> &'flag_option str {
        &self.flag
    }

    pub fn enabled(&self) -> &bool {
        &self.enabled
    }
}

///
///
#[derive(Debug)]
pub struct BuildStrategy<'flag_option> {
    pub wallet: BuildOption<'flag_option>,
    pub sqlite: BuildOption<'flag_option>,
    pub bdb: BuildOption<'flag_option>,
    pub ebpf: BuildOption<'flag_option>,
    pub miniupnc: BuildOption<'flag_option>,
    pub upnp_default: BuildOption<'flag_option>,
    pub natpmp: BuildOption<'flag_option>,
    pub natpmp_default: BuildOption<'flag_option>,
    pub tests: BuildOption<'flag_option>,
}

impl<'flag_option> BuildStrategy<'flag_option> {
    pub fn new() -> Self {
        let wallet = BuildOption::new("--disable-wallet", true);
        let sqlite = BuildOption::new("--with-sqlite", true);
        let bdb = BuildOption::new("--without-bdb", true);
        let ebpf = BuildOption::new("", true);
        let miniupnc = BuildOption::new("", true);
        let upnp_default = BuildOption::new("", true);
        let natpmp = BuildOption::new("", true);
        let natpmp_default = BuildOption::new("", true);
        let tests = BuildOption::new("", true);
        let _ = BuildOption::new("", true);

        Self {
            wallet,
            sqlite,
            bdb,
            ebpf,
            miniupnc,
            upnp_default,
            natpmp,
            natpmp_default,
            tests,
        }
    }
}
