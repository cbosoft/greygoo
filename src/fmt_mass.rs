pub fn fmt_mass(mass: f64) -> String {
    let p = mass.log10();
    let ip = (p as i32/3)*3;
    let m = mass / 10f64.powf(ip as f64);

    let mut unit = "";

    match ip {
        -24 => { unit = "yg" },
        -21 => { unit = "zg" },
        -18 => { unit = "ag" },
        -15 => { unit = "fg" },
        -12 => { unit = "pg" },
        -9 => { unit = "ng" },
        -6 => { unit = "μg" },
        -3 => { unit = "mg" },
        0 => { unit = "g" },
        3 => { unit = "kg" },
        6 => { unit = "t" },
        9 => { unit = "kt" },
        12 => { unit = "Mt" },
        15 => { unit = "Pg" },
        18 => { unit = "Eg" },
        21 => { unit = "Zg" },
        24 => { unit = "Yg" },
        _ => ()
    }

    format!("{:.2}{}", m, unit)
}