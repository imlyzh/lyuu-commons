use std::collections::HashMap;

use once_cell::sync::Lazy;




pub static CSR_MAP: Lazy<HashMap<usize, &str>> = Lazy::new(|| {
    let mut map = HashMap::new();

    /*
    let gpr_def = include_str!("./gpr_def");
    let gpr_def = gpr_def.trim().split("\n").map(|x| {
        let mut r = x.trim().split(" ");
        (
            r.next().unwrap(),
            (RegType::GPR, r.next().unwrap().parse::<usize>().unwrap()),
        )
    });
    map.extend(gpr_def);
     */

    let csr_def = include_str!("./csr_def");
    let csr_def = csr_def.trim().split('\n').map(|x| {
        let mut r = x.trim().split('\t');
        let t = r.next().unwrap();
        let z = r.next().unwrap();
        let z = dbg!(z.trim_start_matches("0x"));
        let z = usize::from_str_radix(z, 16).unwrap();
        (
            z,
            t,
        )
    });
    map.extend(csr_def);

    /*
    let fpr_def = include_str!("./reg_def/fpr_def");
    let fpr_def = fpr_def.trim().split("\n").map(
        |x| {
            let mut r = x.trim().split(" ");
            (r.next().unwrap(), (RegType::FPR, r.next().unwrap().parse::<usize>().unwrap()))
        }
    );
    map.extend(fpr_def);
    //  */

    map
});