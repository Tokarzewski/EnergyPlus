//! Runs the Rust SimulateYear on the EPW that main.cpp reads and prints the same checksum line,
//! plus the checksum's bits and the time. `SIM_TIMING=1` also prints each phase's time.
#[allow(dead_code)]
// SimulateYear is the fast-anything entry point; this binary calls simulate_year
mod port;

fn main() {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "../../weather/USA_IL_Chicago-OHare.Intl.AP.725300_TMY3.epw".into());
    let text = match std::fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("cannot read {path}: {e}");
            std::process::exit(1);
        }
    };
    let (mut tdb, mut tdp, mut pb, mut wspd, mut wdir) = (vec![], vec![], vec![], vec![], vec![]);
    for line in text.lines().skip(8) {
        let f: Vec<&str> = line.split(',').collect();
        if f.len() < 22 {
            continue;
        }
        let p = |s: &str| s.trim().parse::<f64>().unwrap_or(0.0); // atof gives 0 on garbage too
        tdb.push(p(f[6]));
        tdp.push(p(f[7]));
        pb.push(p(f[9]));
        wspd.push(p(f[21]));
        wdir.push(p(f[20]));
    }
    let t = std::time::Instant::now();
    let cs = port::simulate_year(&tdb, &tdp, &pb, &wspd, &wdir);
    let el = t.elapsed();
    println!(
        "{} hours, checksum {cs:.9} (bits {:016x}, {el:?})",
        tdb.len(),
        cs.to_bits()
    );
}
