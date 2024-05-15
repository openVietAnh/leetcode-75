pub fn gcd(a: usize, b: usize) -> usize {
    let (mut n, mut m) = (a, b);
    while n != 0 {
        let val = m % n;
        m = n;
        n = val;
    }
    m
}

pub fn gcd_of_strings(str1: String, str2: String) -> String {
    let lcm_length = gcd(str1.len(), str2.len());
    let (d1, d2) = (str1.len() / lcm_length, str2.len() / lcm_length);
    if str1[0..lcm_length] == str2[0..lcm_length] {
        if str1[0..lcm_length].repeat(d1) == str1 && str2[0..lcm_length].repeat(d2) == str2 {
            str1[0..lcm_length].to_string()
        } else {
            String::from("")
        }
    } else {
        String::from("")
    }
}
