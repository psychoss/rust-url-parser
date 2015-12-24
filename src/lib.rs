use std::fmt;
#[derive(PartialEq)]
pub enum EncodeMode {
    Path,
    Host,
    Zone,
    UserPassword,
    QueryComponent,
    Fragment,
}


pub struct UrlParser;


impl UrlParser {
    pub fn query_escape(url: String) -> String {
        escape(url, &EncodeMode::QueryComponent)
    }
}

#[allow(dead_code)]
fn unescape(url: &str, mode: &EncodeMode) -> Result<String, String> {
    let mut n = 0;
    let mut m = 0;
    let mut has_plus = false;
    // let mut skip = true;
    let vv = &url.as_bytes();

    let l = vv.len();
    loop {
        if m + 1 >= l {
            break;
        }
        match vv[m] {
            37 => {
                n += 1;

                if m + 2 >= l || !is_hex(vv[m + 1]) || !is_hex(vv[m + 2]) {
                    println!("{}{}", vv[m + 1],vv[m + 2]);
                    return Err("".to_string());
                }
                m+=1;
            }
            _ => {
                m += 1;
            }
        }
    }
    // for i in 0..l - 1 {
    //     if skip {
    //         skip = false;
    //         continue;
    //     }
    //     if vv[i] == 37 {
    //         n += 1;
    //         if i + 2 >= l || !is_hex(vv[i + 1]) || !is_hex(vv[i + 2]) {
    //             return Err("".to_string());
    //         }
    //     }
    //     if vv[i] == 43 {
    //         has_plus = mode == &EncodeMode::QueryComponent;
    //         skip = true;
    //     } else {
    // if (mode == &EncodeMode::Host || mode == &EncodeMode::Zone) &&
    // vv[i]< 0x80 &&
    //            should_escape(vv[i], mode) {
    //             return Err("".to_string());
    //         }
    //         skip = true;
    //     }
    // }
    if n == 0 && !has_plus {
        return Ok(url.to_string());
    }
    println!("{:?}", n);
    let ll = l - 2 * n;

    let mut t: Vec<u8> = vec![0;ll as usize];
    let mut j = 0;

    m = 0;
    loop {

        if m + 1 >= l {
            break;
        }

        match vv[m] {
            37 => {
                t[j] = un_hex(vv[m + 1]) << 4 | un_hex(vv[m + 2]);
                j += 1;
                m += 3;
            }
            43 => {
                if mode == &EncodeMode::QueryComponent {
                    t[j] = 32;
                } else {
                    t[j] = 43;
                }
                j += 1;
                m += 1;
            }
            _ => {
                t[j] = vv[m];
                j += 1;
                m += 1;
            }
        }

    }

    Ok(String::from_utf8(t).unwrap())

}
#[allow(dead_code)]
fn is_hex(cc: u8) -> bool {
    match cc {
        48...57 | 65...70 | 97...102 => {
            true
        },
        _ => {
            false
        }
    }
}
/// a-97,z-122
/// A-65,Z-90
/// 0-48,Z-57
#[allow(dead_code)]
fn un_hex(cc: u8) -> u8 {
    match cc {
        48...57 => {
            cc - 48
        }
        65...90 => {
            cc - 55
        }
        97...122 => {
            cc - 87
        }
        _ => {
            0
        }
    }
}

fn escape(url: String, mode: &EncodeMode) -> String {
    let mut space_count: u16 = 0;
    let mut hex_count: u16 = 0;
    let url_clone = url.clone();
    let vv = &url_clone.into_bytes();
    for v in vv {
        if should_escape(*v, mode) {
            if *v == 32 && mode == &EncodeMode::QueryComponent {
                space_count += 1;
            } else {
                hex_count += 1;
            }
        }
    }
    if space_count == 0 && hex_count == 0 {
        return url;
    }
    let l: u16 = vv.len() as u16 + 2 * hex_count;

    let mut t: Vec<u8> = vec![0;l as usize];
    let mut j = 0;
    let s = "0123456789ABCDEF".to_string().into_bytes();

    for v in vv {
        // println!("{} {} {:?}",v,v >> 4,t.len());
        if *v == 32 && mode == &EncodeMode::QueryComponent {
            t[j] = 43;
            j += 1;
        }
        if should_escape(*v, mode) {

            t[j] = 37;
            t[j + 1] = s[(v >> 4) as usize];
            t[j + 2] = s[(v & 15) as usize];
            j += 3;
        } else {
            t[j] = *v;
            j += 1;
        }
    }
    return String::from_utf8(t).unwrap();

}
/// Check the argument is either a letter or number
/// a-97,z-122
/// A-65,Z-90
/// 0-48,Z-57
#[allow(dead_code)]
fn is_alpha_numeric(cc: u8) -> bool {
    match cc {
        97...122 | 65...90 | 48...57 => {
            true
        }
        _ => {
            false
        }
    }
}


///[33,34,36,38,39,40,41,42,43,44,58,59,60,61,62,91,93]
#[allow(dead_code)]
fn should_escape(c: u8, mode: &EncodeMode) -> bool {
    if is_alpha_numeric(c) {
        return false;
    }
    match mode {
        &EncodeMode::Host | &EncodeMode::Zone => {
            let v: [u8; 17] = [33, 34, 36, 38, 39, 40, 41, 42, 43, 44, 58, 59, 60, 61, 62, 91, 93];
            if v.contains(&c) {
                false
            } else {
                true
            }
        }
        _ => {
            let v: [u8; 4] = [45, 46, 95, 126];
            if v.contains(&c) {
                return false;
            }
            let reserved: [u8; 10] = [36, 38, 43, 44, 47, 58, 59, 61, 63, 64];
            if reserved.contains(&c) {
                let x = match mode {
                    &EncodeMode::Path => {
                        c == 63;
                    }
                    &EncodeMode::UserPassword => {
                        &[47, 58, 63, 64].contains(&c);
                    }
                    &EncodeMode::QueryComponent => {
                        true;
                    }
                    _ => {
                        false;
                    }
                };
                x

            }
            true
        }
    }
}

fn main() {
    println!("{:?}", "我方".to_string().into_bytes());
    println!("{}",
             unescape(&"%E6%88%91%E6%96%B9".to_string(), &EncodeMode::Path).unwrap());
    println!("{}", escape("我方".to_string(), &EncodeMode::Path));
}
