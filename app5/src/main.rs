pub fn valid_utf8(data: Vec<i32>) -> bool {
    let code = data;
    let mut i = 0;
    let mut result = false;
    while true {
        if code[i] <= 0b01111111 {
            result = true;
            i = i + 1;
        } else if code[i] >= 0b11000000 && code[i] <= 0b11011111 {
            if i + 1 >= code.len() {
                result = false;
                break;
            }
            result = code[i + 1] >> 6 == 0b00000010;
            i = i + 2;
        } else if code[i] >= 0b11100000 && code[i] <= 0b11101111 {
            if i + 2 >= code.len() {
                result = false;
                break;
            }
            result = code[i + 1] >> 6 == 0b00000010 && code[i + 2] >> 6 == 0b00000010;
            i = i + 3;
        } else if code[i] >= 0b11110000 && code[i] <= 0b11110111 {
            if i + 3 >= code.len() {
                result = false;
                break;
            }
            result = code[i + 1] >> 6 == 0b00000010
                && code[i + 2] >> 6 == 0b00000010
                && code[i + 3] >> 6 == 0b00000010;
            i = i + 4;
        } else {
            result = false;
            break;
        }
        if i >= code.len() {
            break;
        }
        if !result {
            break;
        }
    }
    return result;
}
fn main() {
    let data = vec![225];
    if valid_utf8(data) {
        println!("成功");
    } else {
        println!("失败");
    }
}
#[test]
fn test() {
    let t = |v| valid_utf8(v);
    assert_eq!(true, t(vec![197, 130, 1]));
    assert_eq!(false, t(vec![235, 140, 4]));
    assert_eq!(false, t(vec![0b1111_1111, 130, 1]));
    assert_eq!(false, t(vec![0b1100_0000, 0b0000_1111]));
    assert_eq!(false, t(vec![250, 145, 145, 145, 145]));
    assert_eq!(false, t(vec![248, 130, 130, 130]));
    assert_eq!(false, t(vec![0b1011_0000, 130, 130]));
}
