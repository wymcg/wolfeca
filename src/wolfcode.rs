fn local_state_to_rule(left: bool, center: bool, right: bool) -> u8 {
    let mut out: u8 = 0b00000000;

    if left { out |= 0b00000100 }
    if center { out |= 0b00000010 }
    if right { out |= 0b00000001 }

    return out;
}

fn get_rule_from_code(code: u8, rule_number: u8) -> bool {

    let mask: u8 = 0b00000001 << rule_number;

    return (code & mask) != 0;

}

pub fn get_next_state_from_local_state(code: u8, left: bool, center: bool, right: bool) -> bool {
    return get_rule_from_code(code, local_state_to_rule(left, center, right));
}