//! Defualt build strategy structures for Bitcoin

use crate::error::ShranError;
use std::collections::HashMap;
use std::error::Error;

/// Hardcoded build option names from bitcoins configure.ac file
/// https://github.com/bitcoin/bitcoin/blob/v22.0/configure.ac
///
/// These are used to easily index into the BuildOptions map to trigger options
/// on and off, basically to reduce programmer error, and enable the use of the match
/// statmement when dealing with the map.
#[derive(Debug)]
pub struct BuildOptionName;

impl<'f> BuildOptionName {
    pub const WALLET: &'f str = "wallet";
    pub const SQLITE: &'f str = "sqlite";
    pub const BDB: &'f str = "bdb";
    pub const EBPF: &'f str = "ebpf";
    pub const MINIUPNC: &'f str = "miniupnc";
    pub const UPNP_DEFAULT: &'f str = "upnp-default";
    pub const NATPMP: &'f str = "natpmp";
    pub const NATPMP_DEFAULT: &'f str = "natpmp-default";
    pub const TESTS: &'f str = "tests";
    pub const GUI_TESTS: &'f str = "gui-tests";
    pub const BENCH: &'f str = "bench";
    pub const EXTENDED_FUNCTIONAL_TESTS: &'f str = "extended-functional-tests";
    pub const FUZZ: &'f str = "fuzz";
    pub const FUZZ_BINARY: &'f str = "fuzz-binary";
    pub const QRENCODE: &'f str = "qrencode";
    pub const HARDENING: &'f str = "hardening";
    pub const REDUCE_EXPORTS: &'f str = "reduce-exports";
    pub const CCACHE: &'f str = "ccache";
    pub const SUPPRESS_EXTERNAL_WARNINGS: &'f str = "suppress-external-warnings";
    pub const LCOV: &'f str = "lcov";
    pub const LCOV_BRANCH_COVERAGE: &'f str = "lcov-branch-coverage";
    pub const GLIBC_BACK_COMPAT: &'f str = "glibc-back-compat";
    pub const THREADLOCAL: &'f str = "threadlocal";
    pub const ASM: &'f str = "asm";
    pub const SYSTEM_UNIVALUE: &'f str = "system-univalue";
    pub const ZMQ: &'f str = "zqm";
    pub const LIBMULTIPROCESS: &'f str = "libmultiprocess";
    pub const MPGEN: &'f str = "mpgen";
    pub const MULTIPROCESS: &'f str = "multiprocess";
    pub const MAN: &'f str = "man";
    pub const DEBUG: &'f str = "debug";
    pub const SANITIZERS: &'f str = "sanitizers";
    pub const GPROF: &'f str = "gprof";
    pub const WERROR: &'f str = "werror";
    pub const EXTERNAL_SIGNER: &'f str = "external-signer";
}

/// Bitcoin controls compile flags with these three values,
/// so I mimic them here with an Enum
///
/// # Example
/// * `Yes`  Use the associated build option
/// * `No`   Do not use the associated build option
/// * `Auto` bitcoins build system figures it out, best to leave an option marked with this alone.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OptionEnabled {
    Yes,
    No,
    Auto,
}

/// Container for each bitcoin build option, this allows the user
/// to have full control over the kind of bitcoin node they want, this results
/// in compiling in only the functionality they need, giving faster build times in ci/cd pipelines,
/// a smaller binary foot print etc.
///
/// # Example
/// If an entity needs a bitcoin node just for rpc call purposes,
/// they would want to disable the wallet, sqlite and bdb flags, as that is unecessary.
///
/// * `flag` the command line parameter to be passed to the auto tools configure script
/// * `enabled` the option is turned on or off, if None, the option is auto
/// * `desc` detailed description of the command line parameter
#[derive(Debug, Copy, Clone)]
pub struct BuildOption<'f> {
    flag: &'f str,
    enabled: OptionEnabled,
    desc: &'f str,
}

impl<'f> BuildOption<'f> {
    pub fn new(flag: &'f str, enabled: OptionEnabled, desc: &'f str) -> Self {
        Self {
            flag,
            enabled,
            desc,
        }
    }

    pub fn flag(&self) -> &'f str {
        &self.flag
    }

    pub fn enabled(&self) -> &OptionEnabled {
        &self.enabled
    }

    pub fn update_enabled(&mut self, option: OptionEnabled) {
        self.enabled = option;
    }

    pub fn description(&self) -> &'f str {
        &self.desc
    }
}

/// Custom type that represents to represent a map of all possible build options
pub type BuildOptions<'f> = HashMap<&'f str, BuildOption<'f>>;

/// A build strategy is a composition of all the possible build options.
/// When creating a BuildStrategy object, it returns a pre-configured strategy with
/// sane defaults. This will only happen if a user does not give shran a build strategy
/// yaml file as an argument
#[derive(Debug)]
pub struct BuildStrategy<'f> {
    build_options: BuildOptions<'f>,
}

impl<'f, 'e> BuildStrategy<'f> {
    /// Builds a bitcoin node to default spec, a direct 1 to 1 translation from the bitcoin
    /// configure.ac file options
    ///
    /// ```no_run
    /// let strategy = strategies::bitcoin::BuildStrategy::new();
    /// ```
    pub fn new() -> Self {
        let mut build_options = BuildOptions::new();

        build_options.insert(
            BuildOptionName::WALLET,
            BuildOption::new(
                "--disable-wallet",
                OptionEnabled::Yes,
                "disable wallet (enabled by default)",
            ),
        );

        build_options.insert(
            BuildOptionName::SQLITE,
            BuildOption::new(
                "--with-sqlite",
                OptionEnabled::Auto,
                "enable sqlite wallet support (default: auto, i.e., enabled if wallet is enabled and sqlite is found)"
            ),
        );

        build_options.insert(
            BuildOptionName::BDB,
            BuildOption::new(
                "--without-bdb",
                OptionEnabled::Auto,
                "enable sqlite wallet support (default: auto, i.e., enabled if wallet is enabled and sqlite is found)"
            )
        );

        build_options.insert(
            BuildOptionName::EBPF,
            BuildOption::new(
                "--enable-epbf",
                OptionEnabled::Yes,
                "enable eBPF tracing (default is yes if sys/sdt.h is found)",
            ),
        );

        build_options.insert(
            BuildOptionName::MINIUPNC,
            BuildOption::new(
                "--with-miniupnpc",
                OptionEnabled::Auto,
                "enable UPNP (default is yes if libminiupnpc is found)",
            ),
        );

        build_options.insert(
            BuildOptionName::UPNP_DEFAULT,
            BuildOption::new(
                "--enable-upnp-default",
                OptionEnabled::No,
                "if UPNP is enabled, turn it on at startup (default is no)",
            ),
        );

        build_options.insert(
            BuildOptionName::NATPMP,
            BuildOption::new(
                "--with-natpmp",
                OptionEnabled::Auto,
                "enable NAT-PMP (default is yes if libnatpmp is found)",
            ),
        );

        build_options.insert(
            BuildOptionName::NATPMP_DEFAULT,
            BuildOption::new(
                "--enable-natpmp-default",
                OptionEnabled::No,
                "if NAT-PMP is enabled, turn it on at startup (default is no)",
            ),
        );

        build_options.insert(
            BuildOptionName::TESTS,
            BuildOption::new(
                "--disable-tests",
                OptionEnabled::Yes,
                "do not compile tests (default is yes)",
            ),
        );

        build_options.insert(
            BuildOptionName::GUI_TESTS,
            BuildOption::new(
                "--disable-gui-tests",
                OptionEnabled::No,
                "do not compile GUI tests (default is to compile if GUI and tests enabled)",
            ),
        );

        build_options.insert(
            BuildOptionName::BENCH,
            BuildOption::new(
                "--disable-bench",
                OptionEnabled::No,
                "do not compile benchmarks (default is to compile)",
            ),
        );

        build_options.insert(
            BuildOptionName::EXTENDED_FUNCTIONAL_TESTS,
            BuildOption::new(
                "--enable-extended-functional-tests",
                OptionEnabled::No,
                "enable expensive functional tests when using lcov (default no)",
            ),
        );

        build_options.insert(
            BuildOptionName::FUZZ,
            BuildOption::new(
                "--enable-fuzz",
                OptionEnabled::No,
                "build for fuzzing (default no). enabling this will disable all other targets and override --{enable,disable}-fuzz-binary"
            ),
        );

        build_options.insert(
            BuildOptionName::FUZZ_BINARY,
            BuildOption::new(
                "--enable-fuzz-binary",
                OptionEnabled::Yes,
                "enable building of fuzz binary (default yes).",
            ),
        );

        build_options.insert(
            BuildOptionName::QRENCODE,
            BuildOption::new(
                "--with-qrencode",
                OptionEnabled::Auto,
                "enable QR code support (default is yes if qt is enabled and libqrencode is found)",
            ),
        );

        build_options.insert(
            BuildOptionName::HARDENING,
            BuildOption::new(
                "--disable-hardening",
                OptionEnabled::Auto,
                "do not attempt to harden the resulting executables (default is to harden when possible)"
            )
        );

        build_options.insert(
            BuildOptionName::REDUCE_EXPORTS,
            BuildOption::new(
                "--enable-reduce-exports",
                OptionEnabled::No,
                "attempt to reduce exported symbols in the resulting executables (default is no)",
            ),
        );

        build_options.insert(
            BuildOptionName::CCACHE,
            BuildOption::new(
                "--disable-ccache",
                OptionEnabled::Auto,
                "do not use ccache for building (default is to use if found)",
            ),
        );

        build_options.insert(
            BuildOptionName::SUPPRESS_EXTERNAL_WARNINGS,
            BuildOption::new(
                "--enable-suppress-external-warnings",
                OptionEnabled::No,
                "Suppress warnings from external headers (default is no)",
            ),
        );

        build_options.insert(
            BuildOptionName::LCOV,
            BuildOption::new(
                "--enable-lcov",
                OptionEnabled::No,
                "enable lcov testing (default is no)",
            ),
        );

        build_options.insert(
            BuildOptionName::LCOV_BRANCH_COVERAGE,
            BuildOption::new(
                "--enable-lcov-branch-coverage",
                OptionEnabled::No,
                "enable lcov testing branch coverage (default is no)",
            ),
        );

        build_options.insert(
            BuildOptionName::GLIBC_BACK_COMPAT,
            BuildOption::new(
                "--enable-glibc-back-compat",
                OptionEnabled::No,
                "enable backwards compatibility with glibc (default is no)",
            ),
        );

        build_options.insert(
            BuildOptionName::THREADLOCAL,
            BuildOption::new(
                "--enable-threadlocal",
                OptionEnabled::Auto,
                "enable features that depend on the c++ thread_local keyword (currently just thread names in debug logs). (default is to enabled if there is platform support and glibc-back-compat is not enabled (default is auto)"
            )
        );

        build_options.insert(
            BuildOptionName::ASM,
            BuildOption::new(
                "--disable-asm",
                OptionEnabled::Yes,
                "disable assembly routines (default is yes)",
            ),
        );

        build_options.insert(
            BuildOptionName::SYSTEM_UNIVALUE,
            BuildOption::new(
                "--with-system-univalue",
                OptionEnabled::No,
                "Build with system UniValue (default is no)",
            ),
        );

        build_options.insert(
            BuildOptionName::ZMQ,
            BuildOption::new(
                "--disable-zmq",
                OptionEnabled::Yes,
                "disable ZMQ notifications (default is yes)",
            ),
        );

        build_options.insert(
            BuildOptionName::LIBMULTIPROCESS,
            BuildOption::new(
                "--with-libmultiprocess",
                OptionEnabled::Auto,
                "Build with libmultiprocess library. (default: auto, i.e. detect with pkg-config)",
            ),
        );

        build_options.insert(
            BuildOptionName::MPGEN,
            BuildOption::new(
                "--with-mpgen",
                OptionEnabled::Auto,
                "Build with libmultiprocess codegen tool. Useful to specify different libmultiprocess host system library and build system codegen tool prefixes when cross-compiling (default is auto e.g. host system libmultiprocess prefix)"
            )
        );

        build_options.insert(
            BuildOptionName::MULTIPROCESS,
            BuildOption::new(
                "--enable-multiprocess",
                OptionEnabled::No,
                "build multiprocess bitcoin-node, bitcoin-wallet, and bitcoin-gui executables in addition to monolithic bitcoind and bitcoin-qt executables. Requires libmultiprocess library. Experimental (default is no)"
            )
        );

        build_options.insert(
            BuildOptionName::MAN,
            BuildOption::new(
                "--disable-man",
                OptionEnabled::No,
                "do not install man pages (default is to install)",
            ),
        );

        build_options.insert(
            BuildOptionName::DEBUG,
            BuildOption::new(
                "--enable-debug",
                OptionEnabled::No,
                "use compiler flags and macros suited for debugging (default is no)",
            ),
        );

        build_options.insert(
            BuildOptionName::SANITIZERS,
            BuildOption::new(
                "--with-sanitizers",
                OptionEnabled::No,
                "comma separated list of extra sanitizers to build with (default is no)",
            ),
        );

        build_options.insert(
            BuildOptionName::GPROF,
            BuildOption::new(
                "--enable-gprof",
                OptionEnabled::No,
                "use gprof profiling compiler flags (default is no)",
            ),
        );

        build_options.insert(
            BuildOptionName::WERROR,
            BuildOption::new(
                "--enable-werror",
                OptionEnabled::No,
                "Treat certain compiler warnings as errors (default is no)",
            ),
        );

        build_options.insert(
            BuildOptionName::EXTERNAL_SIGNER,
            BuildOption::new(
                "--enable-external-signer",
                OptionEnabled::Yes,
                "compile external signer support (default is yes, requires Boost::Process)",
            ),
        );

        Self { build_options }
    }

    /// Getter for the BuildOptions hash map
    /// Mostly for testing purposes
    pub fn build_options(&self) -> &BuildOptions<'f> {
        &self.build_options
    }

    /// Update the default BuildStrategy
    ///
    /// * `build_option` Should use the BuildOptionName struct
    /// * `enable_option`
    ///
    /// # Example
    ///
    /// ```no_run
    /// if let Err(error) = build.update_build_option(BuildOptionName::WALLET, &OptionEnabled::No) {
    ///     eprintln!("{}", error);
    ///     std::process::exit(1);
    /// }
    /// ```
    pub fn update_build_option(
        &mut self,
        build_option_name: &str,
        enable_option: OptionEnabled,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(value) = self.build_options.get_mut(build_option_name) {
            value.update_enabled(enable_option);
            return Ok(());
        }
        Err(Box::new(ShranError::UnrecognizedBuildOptionNameError {
            msg: build_option_name.to_string(),
            file: file!(),
            line: line!(),
            column: column!(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::{BuildOptionName, BuildStrategy, OptionEnabled};

    #[test]
    fn test_verify_wallet_build_options() {
        let b = BuildStrategy::new();
        let build_opts = b.build_options();
        let build_options = build_opts.get(BuildOptionName::WALLET);

        assert_eq!(build_options.is_some(), true);
        let option = build_options.unwrap();
        assert_eq!(option.flag(), "--disable-wallet");
        assert_eq!(option.enabled(), &OptionEnabled::Yes);
        assert_eq!(option.description(), "disable wallet (enabled by default)");
    }

    #[test]
    fn test_updating_wallet_build_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::WALLET, OptionEnabled::No);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::WALLET);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::No);
    }

    #[test]
    fn test_updating_sqlite_build_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::SQLITE, OptionEnabled::No);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::SQLITE);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::No);
    }

    #[test]
    fn test_updating_bdb_build_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::BDB, OptionEnabled::No);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::BDB);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::No);
    }

    #[test]
    fn test_updating_ebpf_build_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::EBPF, OptionEnabled::Auto);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_opts = b.build_options();
        let opt = build_opts.get(BuildOptionName::EBPF);
        assert_eq!(opt.is_some(), true);
        let wallet = opt.unwrap();
        assert_eq!(wallet.enabled(), &OptionEnabled::Auto);
    }

    #[test]
    fn test_updating_miniupnc_build_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::MINIUPNC, OptionEnabled::Yes);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::MINIUPNC);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::Yes);
    }

    #[test]
    fn test_updating_upnp_default_build_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::UPNP_DEFAULT, OptionEnabled::Yes);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::UPNP_DEFAULT);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::Yes);
    }

    #[test]
    fn test_updating_natpmp_build_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::NATPMP, OptionEnabled::Yes);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::NATPMP);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::Yes);
    }

    #[test]
    fn test_updating_natpmp_default_build_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::NATPMP_DEFAULT, OptionEnabled::Yes);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::NATPMP_DEFAULT);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::Yes);
    }

    #[test]
    fn test_updating_tests_build_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::TESTS, OptionEnabled::No);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::TESTS);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::No);
    }

    #[test]
    fn test_updating_gui_tests_build_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::GUI_TESTS, OptionEnabled::Yes);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::GUI_TESTS);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::Yes);
    }

    #[test]
    fn test_updating_bench_build_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::BENCH, OptionEnabled::Yes);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::BENCH);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::Yes);
    }

    #[test]
    fn test_updating_extended_functional_tests_build_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(
            BuildOptionName::EXTENDED_FUNCTIONAL_TESTS,
            OptionEnabled::Yes,
        );
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::EXTENDED_FUNCTIONAL_TESTS);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::Yes);
    }

    #[test]
    fn test_updating_fuzz_build_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::FUZZ, OptionEnabled::Yes);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::FUZZ);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::Yes);
    }

    #[test]
    fn test_updating_fuzz_binary_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::FUZZ_BINARY, OptionEnabled::Auto);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::FUZZ_BINARY);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::Auto);
    }

    #[test]
    fn test_updating_qrencode_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::QRENCODE, OptionEnabled::No);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::QRENCODE);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::No);
    }

    #[test]
    fn test_updating_hardening_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::HARDENING, OptionEnabled::No);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::HARDENING);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::No);
    }

    #[test]
    fn test_updating_reduce_exports_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::REDUCE_EXPORTS, OptionEnabled::Yes);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::REDUCE_EXPORTS);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::Yes);
    }

    #[test]
    fn test_updating_ccache_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::CCACHE, OptionEnabled::Yes);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::CCACHE);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::Yes);
    }

    #[test]
    fn test_updating_suppress_external_warnings_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(
            BuildOptionName::SUPPRESS_EXTERNAL_WARNINGS,
            OptionEnabled::Yes,
        );
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::SUPPRESS_EXTERNAL_WARNINGS);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::Yes);
    }

    #[test]
    fn test_updating_lcov_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::LCOV, OptionEnabled::Yes);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::LCOV);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::Yes);
    }

    #[test]
    fn test_updating_lcov_branch_coverage_option() {
        let mut b = BuildStrategy::new();
        let result =
            b.update_build_option(BuildOptionName::LCOV_BRANCH_COVERAGE, OptionEnabled::Yes);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::LCOV_BRANCH_COVERAGE);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::Yes);
    }

    #[test]
    fn test_updating_glibc_back_compat_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::GLIBC_BACK_COMPAT, OptionEnabled::Yes);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::GLIBC_BACK_COMPAT);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::Yes);
    }

    #[test]
    fn test_updating_threadlocal_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::THREADLOCAL, OptionEnabled::Yes);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::THREADLOCAL);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::Yes);
    }

    #[test]
    fn test_updating_asm_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::ASM, OptionEnabled::No);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::ASM);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::No);
    }

    #[test]
    fn test_updating_system_univalue_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::SYSTEM_UNIVALUE, OptionEnabled::Yes);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::SYSTEM_UNIVALUE);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::Yes);
    }

    #[test]
    fn test_updating_zmq_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::ZMQ, OptionEnabled::No);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::ZMQ);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::No);
    }

    #[test]
    fn test_updating_libmultiprocess_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::LIBMULTIPROCESS, OptionEnabled::No);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::LIBMULTIPROCESS);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::No);
    }

    #[test]
    fn test_updating_mpgen_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::MPGEN, OptionEnabled::No);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::MPGEN);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::No);
    }

    #[test]
    fn test_updating_multiprocess_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::MULTIPROCESS, OptionEnabled::Yes);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::MULTIPROCESS);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::Yes);
    }

    #[test]
    fn test_updating_man_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::MAN, OptionEnabled::Yes);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::MAN);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::Yes);
    }

    #[test]
    fn test_updating_debug_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::DEBUG, OptionEnabled::Yes);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::DEBUG);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::Yes);
    }

    #[test]
    fn test_updating_sanitizers_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::SANITIZERS, OptionEnabled::Yes);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::SANITIZERS);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::Yes);
    }

    #[test]
    fn test_updating_gprof_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::GPROF, OptionEnabled::Yes);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::GPROF);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::Yes);
    }

    #[test]
    fn test_updating_werror_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::WERROR, OptionEnabled::Yes);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::WERROR);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::Yes);
    }

    #[test]
    fn test_updating_external_signer_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option(BuildOptionName::EXTERNAL_SIGNER, OptionEnabled::No);
        assert_eq!(result.is_ok(), true);

        // Verify our new enabled option is No , intstead of the default Yes
        let build_options = b.build_options();
        let wrapped_option = build_options.get(BuildOptionName::EXTERNAL_SIGNER);
        assert_eq!(wrapped_option.is_some(), true);
        let option = wrapped_option.unwrap();
        assert_eq!(option.enabled(), &OptionEnabled::No);
    }

    #[test]
    fn test_updating_non_existant_build_option() {
        let mut b = BuildStrategy::new();
        let result = b.update_build_option("does not exist", OptionEnabled::No);
        assert_eq!(result.is_err(), true);
    }
}
