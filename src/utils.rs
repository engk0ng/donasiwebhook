use std::mem::drop;

pub fn convert_format_money(money: String) -> String {
    if money == "0" {
        let res = String::from("Rp 0");
        res
    }
    else {
        let mut len = money.len();
        let ss = (len as f64/3.0).round();
        let mut tt = len;
        let mut v = Vec::<String>::new();
        for _ in 0..ss as i32 {
            let res;
            if tt >= 3 {
                tt -= 3;
                res = String::from(&money[tt..len]);
                len -= 3;
            }
            else {
                res = String::from(&money[0..tt]);
            }
            v.push(res);
        }
        v.reverse();
        let mut result = String::from("");
        for i in 0..v.len() {
            result.push_str(v[i].as_str());
            if i < v.len() - 1 {
                result.push_str(".");
            }
        }
        println!("{:?}", v);
        std::mem::drop(v);
        String::from(result)
    }
}