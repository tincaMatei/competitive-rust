//! Run the given solution and use a profiler to figure out which places are
//! not properly optimized.
//!
//! The profile can be opened by opening the `flamegraph.svg` file in a browser.
//!
//! Requires the [pprof] crate. Since in Competitive Programming, you cannot download
//! additional crates, don't forget to comment it out.

fn computation() {

}

fn main() {
    let guard = pprof::ProfilerGuardBuilder::default()
        .frequency(100)
        .blocklist(&["libc", "libgcc", "pthread", "vdso"])
        .build()
        .unwrap();

    computation();

    if let Ok(report) = guard.report().build() {
        let file = std::fs::File::create("flamegraph.svg").unwrap();
        report.flamegraph(file).unwrap();
    }
}


