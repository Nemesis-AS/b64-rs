pub fn encode_data(data: &[u8]) -> String {
    let mut vec: Vec<u8> = Vec::new();
    let mut res: String = String::new();

    vec.extend_from_slice(data);
   
    while vec.len() > 2 {
        let sl: Vec<u8> = vec.drain(0..3).collect();
        let st: String = encode_triplet(sl.as_slice());
        res.push_str(st.as_str());
    }

    if vec.len() > 0 {
        let st: String = encode_triplet(vec.as_slice());
        res.push_str(st.as_str());
    }

    res
}

fn encode_triplet(chars: &[u8]) -> String {
    const CHARACTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";

    let mut idxs: Vec<usize> = Vec::new();

    match chars.len() {
        1 => {
            idxs.push(((chars[0] & 0b1111_1111) >> 2) as usize);
            idxs.push(((chars[0] & 0b0000_0011) << 4) as usize);
            idxs.push(64);
            idxs.push(64);
        },
        2 => {
            idxs.push(((chars[0] & 0b1111_1111) >> 2) as usize);
            idxs.push((((chars[0] & 0b0000_0011) << 4) + ((chars[1] & 0b1111_0000) >> 4)) as usize);
            idxs.push(((chars[1] & 0b0000_1111) << 2) as usize);
            idxs.push(64);
        },
        3 => {
            idxs.push(((chars[0] & 0b1111_1111) >> 2) as usize);
            idxs.push((((chars[0] & 0b0000_0011) << 4) + ((chars[1] & 0b1111_0000) >> 4)) as usize);
            idxs.push((((chars[1] & 0b0000_1111) << 2) + ((chars[2] & 0b1100_0000) >> 6)) as usize);
            idxs.push((chars[2] & 0b0011_1111) as usize);
        },
        _ => ()
    }
    
    let mut res: String = String::new();
    for idx in idxs {
        res.push(CHARACTERS.chars().nth(idx).unwrap());
    }

    res
}
