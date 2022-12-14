use std::fs;
use hex::FromHex;
use bit_vec::BitVec;

fn main() {
    task01();
    task02();
}

fn task02(){
    let stream = get_hex_stream();
    let packets = decoder(&stream);
    let result = operate_packets(&packets[0]);
    println!("{}", result);
}

fn task01(){
    let stream = get_hex_stream();
    let packets = decoder(&stream);
    println!("{}", sum_versions(&packets));
}

fn decoder(stream: &String) -> Vec<Packet>{
    let bytes = <Vec<u8>>::from_hex(stream).unwrap();
    let mut bits = BitVec::from_bytes(&bytes);
    let mut packets:Vec<Packet> = Vec::new();
    while bits.len() > 8{
        let packet = get_packet(&mut bits);
        packets.push(packet);
    }
    return packets;
}

fn operate_packets(packet: &Packet) -> i64{
    let mut values: Vec<i64> = Vec::new();
    for sub_packet in &packet.sub_packets {
        match &sub_packet.type_id {
            TypeID::Literal => {values.push(sub_packet.value as i64);}
            _=> {values.push(operate_packets(sub_packet));}
        }
    }
    return match packet.type_id {
        TypeID::Literal => { packet.value as i64 }
        TypeID::Sum => { values.iter().sum() }
        TypeID::Product => { values.iter().product() }
        TypeID::Min => { *values.iter().min().unwrap() }
        TypeID::Max => { *values.iter().max().unwrap() }
        TypeID::Greater => { if values[0] > values[1] { 1 } else { 0 } }
        TypeID::Lesser => { if values[0] < values[1] { 1 } else { 0 } }
        TypeID::Equals => { if values[0] == values[1] { 1 } else { 0 } }
    }
}

fn sum_versions(packets: &Vec<Packet>) -> u32{
    let mut sum_version = 0;
    for packet in packets{
        if packet.sub_packets.len() > 0{
            sum_version +=  sum_versions(&packet.sub_packets) as u32;
        }
            sum_version += packet.version as u32;
    }
    return sum_version;
}

fn get_packet(bits: &mut BitVec) -> Packet{
    let version = read_version_id(bits);
    let type_id = read_version_id(bits);
    if type_id == 4 {
        return read_literal(bits, (version, type_id));
    }
    else{
        return read_operation(bits, (version, type_id));
    }
}

fn read_operation(bits: &mut BitVec, header: (u8, u8)) -> Packet{
    let len_flag = bits[0];
    let len_type_id: usize;
    *bits = bits.iter().skip(1).collect();
    if len_flag {len_type_id = 11} else {len_type_id = 15}
    let mut length_bits = BitVec::new();
    length_bits.extend(bits.iter().take(len_type_id));
    *bits = bits.iter().skip(len_type_id).collect();

    length_bits = length_bits.iter().rev().collect();
    let remaining = 8 - (length_bits.len() % 8);
    for _ in 0..remaining {
        length_bits.push(false);
    }
    length_bits = length_bits.iter().rev().collect();

    let buf = length_bits.to_bytes();
    let length = ((buf[0] as u16) << 8) | buf[1] as u16;
    let mut total = 0;
    let mut sub_packets: Vec<Packet> = Vec::new();
    if !len_flag {
        while total < length {
            let packet = get_packet(bits);
            total += packet.length;
            sub_packets.push(packet);
        }
    }
    else {
        for _ in 0..length {
            let packet = get_packet(bits);
            total += packet.length;
            sub_packets.push(packet);
        }
    }
    let total_len = 6 + 1 + len_type_id as u16 + total;
    let packet = Packet{version: header.0, type_id: TypeID::from_u8(header.1), length: total_len, value: 0, sub_packets};
    return packet;
}

fn read_version_id(bits: &mut BitVec) -> u8{
    let result: BitVec = bits.iter().take(3).collect();
    *bits = bits.iter().skip(3).collect();
    return result.to_bytes()[0] >> 5;
}

fn read_literal(bits: &mut BitVec, header: (u8, u8)) -> Packet{
    let mut collected = BitVec::new();
    let mut read = 0;
    loop{
        let flag = bits[0];
        *bits = bits.iter().skip(1).collect();
        collected.extend(bits.iter().take(4));
        *bits = bits.iter().skip(4).collect();
        read += 5;
        if !flag {
            break;
        }
    }
    collected = collected.iter().rev().collect();
    let remaining = 8 - (collected.len() % 8);
    for _ in 0..remaining {
        collected.push(false);
    }
    collected = collected.iter().rev().collect();
    let buf = collected.to_bytes();
    let mut value:u64 = 0;
    for idx in 0..buf.len() {
        value |= (buf[idx] as u64) << 8 * (buf.len()-(idx+1));
    }
    let packet = Packet{version: header.0, type_id: TypeID::Literal, length: read + 6, value, sub_packets:Vec::new()};
    packet
}

fn get_hex_stream() -> String{
    let filename = "./data/day16.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file").replace("\r", "");
    return contents.to_lowercase();
}

struct Packet{
    version: u8,
    type_id: TypeID,
    length: u16,
    value: u64,
    sub_packets: Vec<Packet>
}

enum TypeID{
    Literal,
    Sum,
    Product,
    Min,
    Max,
    Greater,
    Lesser,
    Equals
}

impl TypeID{
    fn from_u8(number: u8) -> TypeID{
        if number == 0 {
            TypeID::Sum
        } else if number == 1 {
            TypeID::Product
        } else if number == 2 {
            TypeID::Min
        } else if number == 3 {
            TypeID::Max
        } else if number == 5 {
            TypeID::Greater
        } else if number == 6 {
            TypeID::Lesser
        } else if number == 7 {
            TypeID::Equals
        } else {
            TypeID::Literal
        }
    }
}