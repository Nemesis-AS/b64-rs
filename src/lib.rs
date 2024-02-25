use std::collections::HashMap;

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

fn decode_quadruplet(data: &str) -> Vec<u8> {
    let map: HashMap<char, u8> = HashMap::from([
        ('A', 0),
        ('B', 1),
        ('C', 2),
        ('D', 3),
        ('E', 4),
        ('F', 5),
        ('G', 6),
        ('H', 7),
        ('I', 8),
        ('J', 9),
        ('K', 10),
        ('L', 11),
        ('M', 12),
        ('N', 13),
        ('O', 14),
        ('P', 15),
        ('Q', 16),
        ('R', 17),
        ('S', 18),
        ('T', 19),
        ('U', 20),
        ('V', 21),
        ('W', 22),
        ('X', 23),
        ('Y', 24),
        ('Z', 25),
        ('a', 26),
        ('b', 27),
        ('c', 28),
        ('d', 29),
        ('e', 30),
        ('f', 31),
        ('g', 32),
        ('h', 33),
        ('i', 34),
        ('j', 35),
        ('k', 36),
        ('l', 37),
        ('m', 38),
        ('n', 39),
        ('o', 40),
        ('p', 41),
        ('q', 42),
        ('r', 43),
        ('s', 44),
        ('t', 45),
        ('u', 46),
        ('v', 47),
        ('w', 48),
        ('x', 49),
        ('y', 50),
        ('z', 51),
        ('0', 52),
        ('1', 53),
        ('2', 54),
        ('3', 55),
        ('4', 56),
        ('5', 57),
        ('6', 58),
        ('7', 59),
        ('8', 60),
        ('9', 61),
        ('+', 62),
        ('/', 63)
    ]);

    let mut mdata: Vec<u8> = Vec::new();
    let fallback: u8 = 255;

    for ch in data.chars() {

        let val = map.get(&ch).unwrap_or(&fallback).clone();
        if val == 255 {
            break;
        }
        mdata.push(val);
    }

    let mut v: Vec<u8> = Vec::new();
    match mdata.len() {
        2 => {
            v.push(((mdata[0] & 0b0011_1111) << 2) + ((mdata[1] & 0b0011_0000) >> 4));
        },
        3 => {
            v.push(((mdata[0] & 0b0011_1111) << 2) + ((mdata[1] & 0b0011_0000) >> 4));
            v.push(((mdata[1] & 0b0000_1111) << 4) + ((mdata[2] & 0b0011_1100) >> 2));
        },
        4 => {
            v.push(((mdata[0] & 0b0011_1111) << 2) + ((mdata[1] & 0b0011_0000) >> 4));
            v.push(((mdata[1] & 0b0000_1111) << 4) + ((mdata[2] & 0b0011_1100) >> 2));
            v.push(((mdata[2] & 0b0000_0011) << 6) + mdata[3]);
        },
        _ => ()
    }

    v
}

pub fn decode_data(data: String) -> Vec<u8> {
    if data.len() % 4 != 0 {
        panic!("Invalid Data!");
    }

    let mut datac: String = data.clone();
    let mut decoded_data: Vec<u8> = Vec::new();

    while datac.len() > 0 {
        let quad = datac.drain(..4);
        let mut res: Vec<u8> = decode_quadruplet(quad.as_str());
        decoded_data.append(&mut res);
    }

    decoded_data
}
