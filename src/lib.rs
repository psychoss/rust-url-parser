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
fn escape(url:String,mode:EncodeMode)->String{
    let mut spaceCount:u16=0;
    let mut hexCount:u16=0;
    let vv=url.into_bytes();
    for v in vv{
        if should_escape(v){
            if v==32 && mode==EncodeMode::QueryComponent{
                spaceCount +=1;
            }eles{
                hexCount+=1;
            }
        }
    }
    if spaceCount==0&& hexCount==0 {
        return url;
    }
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
fn should_escape(c: u8, mode: EncodeMode) -> bool {
    if is_alpha_numeric(c) {
        return false;
    }
    match mode {
        EncodeMode::Host | EncodeMode::Zone => {
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
                    EncodeMode::Path => {
                        c == 63;
                    },
                    EncodeMode::UserPassword=>{
                        &[47,58,63,64].contains(&c);
                    },
                    EncodeMode::QueryComponent => {
                        true;
                    },
                    _=> {
                        false;
                    }
                };
                x

            }
            true
        }
    }
}

#[test]
fn it_works() {
    assert_eq!(false, should_escape('a' as u8, EncodeMode::Path));
    assert_eq!(true, should_escape('-' as u8, EncodeMode::Host));
    assert_eq!(true, should_escape('<' as u8, EncodeMode::Path));
    assert_eq!(true, should_escape('?' as u8, EncodeMode::UserPassword));

}
