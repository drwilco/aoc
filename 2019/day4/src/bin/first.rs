fn test(password: i32) -> bool {
  let mut digits = Vec::new();
  let mut remain = password;
  for i in (0..6).rev() {
    digits.push(remain / 10_i32.pow(i));
    remain = remain % 10_i32.pow(i);
  }
  if digits[0] != digits[1]
    && digits[1] != digits[2]
    && digits[2] != digits[3]
    && digits[3] != digits[4]
    && digits[4] != digits[5] {
    // no two adjacent digits are the same
    return false;
  }
  if digits[0] > digits[1]
    || digits[1] > digits[2]
    || digits[2] > digits[3]
    || digits[3] > digits[4]
    || digits[4] > digits[5] {
    // going from left to right digits should not decrease
    return false;
  }
  true
}

fn main() {
  let mut count = 0;
  for password in 372037..905158 {
    if test(password) {
      count += 1;
      println!("{}", password);
    }
  }
  println!("{}", count);
}
