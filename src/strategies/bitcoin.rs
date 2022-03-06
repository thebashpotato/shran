//! Defualt build strategy structures for Bitcoin

/// Container for each bitcoin build option, this allows the user
/// to have full control over the kind of bitcoin node they want, this results
/// in compiling in only the functionality they need, giving faster build times in ci/cd pipelines,
/// a smaller binary foot print etc.
///
/// For example if a company needs a bitcoin node just for rpc call purposes,
/// they would want to disable the wallet, sqlite and bdb flags, as that is unecessary.
///
#[derive(Debug)]
pub struct BuildOption<'f> {
    flag: &'f str,
    enabled: bool,
    desc: &'f str,
}

impl<'f> BuildOption<'f> {
    pub fn new(flag: &'f str, enabled: bool, desc: &'f str) -> Self {
        Self { flag, enabled, desc }
    }

    pub fn flag(&self) -> &'f str {
        &self.flag
    }

    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    pub fn description(&self) -> &'f str {
        &self.desc
    }
}

/// A build strategy is a composition of all the possible build options.
/// When creating a BuildStrategy object, it returns a pre-configured strategy with
/// sane defaults. This will only happen if a user does not give shran a build strategy
/// yaml file as an argument
///
#[derive(Debug)]
pub struct BuildStrategy<'f> {
    pub wallet: BuildOption<'f>,
    pub sqlite: BuildOption<'f>,
    pub bdb: BuildOption<'f>,
    pub ebpf: BuildOption<'f>,
    pub miniupnc: BuildOption<'f>,
    pub upnp_default: BuildOption<'f>,
    pub natpmp: BuildOption<'f>,
    pub natpmp_default: BuildOption<'f>,
    pub tests: BuildOption<'f>,
}

impl<'f> BuildStrategy<'f> {
    pub fn new() -> Self {
        let wallet = BuildOption::new("--disable-wallet", true, "");
        let sqlite = BuildOption::new("--with-sqlite", true, "");
        let bdb = BuildOption::new("--without-bdb", true, "");
        let ebpf = BuildOption::new("", true, "");
        let miniupnc = BuildOption::new("", true, "");
        let upnp_default = BuildOption::new("", true, "");
        let natpmp = BuildOption::new("", true, "");
        let natpmp_default = BuildOption::new("", true, "");
        let tests = BuildOption::new("", true, "");
        let _ = BuildOption::new("", true, "");
        let _ = BuildOption::new("", true, "");

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
