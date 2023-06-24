/// Compare two version code.
/// if version 1 > version 2, return -1
/// if version 2 > version 1 ,return 1
/// if equal, return 0
pub fn compare_version(version1: String, version2: String) -> i32 {
    let mut v1 = version1.split('.');
    let mut v2 = version2.split('.');
    loop {
        match (v1.next(), v2.next()) {
            (Some(v1_), Some(v2_)) => {
                if v1_.parse::<i32>().unwrap() > v2_.parse::<i32>().unwrap() {
                    return 1;
                } else if v1_.parse::<i32>().unwrap() < v2_.parse::<i32>().unwrap() {
                    return -1;
                }
            }
            (Some(v1_), None) => {
                if v1_.parse::<i32>().unwrap() != 0 {
                    return 1;
                }
            }
            (None, Some(v2_)) => {
                if v2_.parse::<i32>().unwrap() != 0 {
                    return -1;
                }
            }
            (None, None) => {
                return 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(compare_version("1.01".to_string(), "1.001".to_string()), 0);
    }

        #[test]
    fn a() {
        assert_eq!(compare_version("1.0".to_string(), "1.0.0".to_string()), 0);
    }

        #[test]
    fn it_wobks() {
        assert_eq!(compare_version("0.1".to_string(), "1.1".to_string()), -1);
    }
}
