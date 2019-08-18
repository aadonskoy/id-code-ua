const MULT_OUT: [u8; 7] = [1, 2, 3, 4, 5, 6, 7];
const MULT_BETWEEN: [u8; 7] = [7, 1, 2, 3, 4, 5, 6];
const LOW_EDGE: u32 = 30_000_000;
const HIGH_EDGE: u32 = 60_000_000;
const ID_LENGTH: usize = 8;
pub const ID_LOWEST: u32 = 10_000_000;

pub fn is_valid(id: u32) -> bool {
  let digits = num_to_vec(id);
  let ctrl_sum: u8 = control_sum(&digits, &multiplicator(id));

  let mut value_to_check: u8 = ctrl_sum % 11;
  if value_to_check > 10 {
    value_to_check = control_sum(&num_to_vec(id), &increased_multiplicator(id)) % 11
  }

  value_to_check == digits[(ID_LENGTH - 1) as usize]
}

fn multiplicator(id: u32) -> Vec<u8> {
  if id > LOW_EDGE && id < HIGH_EDGE {
    MULT_BETWEEN.to_vec()
  } else {
    MULT_OUT.to_vec()
  }
}

fn increased_multiplicator(id: u32) -> Vec<u8> {
  multiplicator(id).iter().map(|digit| digit + 2).collect()
}

fn num_to_vec(id: u32) -> Vec<u8> {
  id.to_string()
    .chars()
    .map(|ch| ch.to_digit(10).unwrap() as u8)
    .collect()
}

fn _num_to_vec(id: u32) -> Vec<u8> {
  let mut remain: u32 = id;
  let mut digits: Vec<u8> = Vec::new();
  let mut divider: u32 = ID_LOWEST;

  while divider != 0 {
    let digit = remain / divider;
    digits.push(digit as u8);

    remain = remain - divider * digit;
    divider = divider / 10;
  }

  digits
}

fn control_sum(digits: &Vec<u8>, mult: &Vec<u8>) -> u8 {
  let eval_digits = digits[0..((ID_LENGTH - 1) as usize)].to_vec();
  let mut res: u8 = 0;
  for (i, dig) in eval_digits.iter().enumerate() {
    res += dig * mult[i];
  }

  res
}
