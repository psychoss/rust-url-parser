
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
    let s= "0123456789ABCDEF".to_string().into_bytes();

    for v in vv {
    //println!("{} {} {:?}",v,v >> 4,t.len());
        if *v == 32 && mode == &EncodeMode::QueryComponent {
            t[j] = 43;
            j += 1;
        }
        if should_escape(*v, mode) {

            t[j] = 37;
            t[j + 1] = s[(v >> 4) as usize];
            t[j + 2] =s[(v & 15) as usize];
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
    if 97 <= cc && cc <= 122 {
        return true;
    }
    if 65 <= cc && cc <= 90 {
        return true;
    }
    if 48 <= cc && cc <= 57 {
        return true;
    }
    false
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

// fn main(){
//
// println!("{}",escape("我方".to_string(),&EncodeMode::Path));
// }
