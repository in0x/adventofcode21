use super::common;
use std::path::Path;

fn decode_byte(byte: u8) -> u8 {
    if byte <= 57 {
        byte - 48 // '0' -> 48
    } else {
        byte - 55 // 'A' -> 65
    }
}

struct BitStream {
    bits: Vec<u8>,
    len: usize,
    pos: usize
}

impl BitStream {
    pub fn new() -> BitStream {
        BitStream { bits: Vec::new(), len: 0, pos: 0 }
    }

    pub fn push(&mut self, byte: u8) {
        self.bits.push(byte);
        self.len += 8;
    }

    pub fn seek(&mut self, new_pos: usize) {
        assert!(new_pos < self.len);
        self.pos = new_pos;
    }

    pub fn try_seek(&mut self, new_pos: usize) -> Result<(), ()> {
        if new_pos < self.len {
            self.pos = new_pos;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn advance(&mut self, by: usize) {
        assert!(self.pos + by < self.len);
        self.pos += by;
    }

    pub fn try_advance(&mut self, by: usize) -> Result<(), ()> {
        if self.pos + by < self.len {
            self.pos += by;
            Ok(())
        } else {
            Err(())
        }
    }

    fn byte_at(bytes: &[u8], pos: usize) -> u8 {
        bytes[pos / 8]
    }

    fn bit_at(byte: u8, pos: usize) -> u8 {
        let shift_by = 7 - pos;
        (byte & (1 << shift_by)) >> shift_by
    }

    /// Reads num_bits from the stream currents pos into the returned value.
    /// NOTE: Reading does not advance the current stream position!
    pub fn read(&self, num_bits: usize) -> u64 {
        assert!((self.pos + 1) >= num_bits);    
    
        let mut result = 0;

        for i in 0..num_bits {
            let byte_idx = self.pos - i;
            let this_byte = BitStream::byte_at(&self.bits, byte_idx);
            let bit_idx = byte_idx % 8;
            let ith_bit = BitStream::bit_at(this_byte, bit_idx) as u64; 

            result |= ith_bit << i;
        }

        result
    }
}

pub fn run(root_dir: &Path) {
    let input_path = root_dir.join("day16_input.txt");
    let bytes = common::read_input_bytes(input_path.as_path());

    let mut stream = BitStream::new();
    for i in (0..(bytes.len() - 1)).step_by(2) {
        let b1 = decode_byte(bytes[i]);
        let b2 = decode_byte(bytes[i + 1]);
        let spliced = (b1 << 4) | b2;
        stream.push(spliced);
    }

    for b in &stream.bits {
        print!("{:08b}'", b);
    }
    println!("");

    fn align_to_n(val: usize, align_to: usize) -> usize {
        (val + (align_to - 1)) & (!(align_to - 1))
    }

    fn align_to_end_of_packet(stream: &mut BitStream) -> Result<(), ()> {
        println!("Before align {}", stream.pos);
        let next_aligned_pos = align_to_n(stream.pos, 4);
        let result = stream.try_seek(next_aligned_pos); // Try to skip past padding to next package
        println!("After align {}", stream.pos);
        result
    }

    fn parse_header(stream: &mut BitStream) -> (u64, u64) {
        stream.advance(2); // Move to the inital version bits

        let version = stream.read(3);
        println!("version {}", version);

        stream.advance(3); // Move to end of type_id bits
        let type_id = stream.read(3);
        println!("type id {}", type_id);

        (version, type_id)
    }

    fn parse_literal_packet(stream: &mut BitStream) -> u64 {
        let mut num_digits = 0;
        let mut result = 0;
        loop {
            stream.advance(1); // Move to "keep_going" bit
            let keep_going = stream.read(1);
            
            stream.advance(4); // Move to next 4 bits of literal
            let digit = stream.read(4);
            // println!("digit {}", );
            
            result |= digit << (4 * num_digits);

            println!("keep going {}", keep_going);
            if keep_going == 0 {
                break;
            }
            num_digits += 1;
        }
        println!("literal result {}", result);
        result
    }

    fn parse_operator_num_bits(stream: &mut BitStream) -> u64 {
        stream.advance(15);
        stream.read(15)
    }

    fn parse_operator_num_packets(stream: &mut BitStream) -> u64 {
        stream.advance(11);
        stream.read(11)
    }

        // Test values
    // 38006F45291200
    // D2FE28D2FE28
    // EE00D40C823060

    fn parse_packet(stream: &mut BitStream, is_subpacket: bool) -> u64 {
        println!("Packet pos {}", stream.pos);
        let (_, kind) = parse_header(stream);

        const PACKET_KIND_LITERAL: u64 = 4;
        const LEN_KIND_TOTAL_BITS: u64 = 0;
        const OP_KIND_ADD: u64 = 0;
        const OP_KIND_MUL: u64 = 1;
        const OP_KIND_MIN: u64 = 2;
        const OP_KIND_MAX: u64 = 3;
        const OP_KIND_GT: u64  = 5;
        const OP_KIND_LT: u64  = 6;
        const OP_KIND_EQ: u64  = 7;
        const _OP_KIND_LITERAL: u64 = 4;

        let mut result = 0;

        if kind == PACKET_KIND_LITERAL {
            result = parse_literal_packet(stream);
            
            let res = stream.try_advance(1);

            // if !is_subpacket {
                // stream.advance(1);
            // }
        } 
        else { // PACKET_KIND_OPERATOR
            stream.advance(1);
            let len_kind = stream.read(1);
            
            fn apply_op(kind: u64, args: &[u64]) -> u64 {
                assert!(!args.is_empty());
                match kind {
                    OP_KIND_ADD => {
                        args.iter().sum::<u64>()
                    },
                    OP_KIND_MUL => { 
                        args.iter().product::<u64>()
                    },
                    OP_KIND_MIN => {
                        let mut min = args[0];
                        for i in 1..args.len() {
                            min = u64::min(min, args[i]);
                        }
                        min
                    },
                    OP_KIND_MAX => {
                        let mut max = args[0];
                        for i in 1..args.len() {
                            max = u64::max(max, args[i]);
                        }
                        max
                    },
                    OP_KIND_GT  => { 
                        assert!(args.len() == 2); 
                        if args[0] > args[1] { 1 } else { 0 } 
                    },
                    OP_KIND_LT  => { 
                        assert!(args.len() == 2); 
                        if args[0] < args[1] { 1 } else { 0 } 
                    },
                    OP_KIND_EQ  => { 
                        assert!(args.len() == 2); 
                        if args[0] == args[1] { 1 } else { 0 } 
                    },
                    _ => panic!(),
                }
            }

            if len_kind == LEN_KIND_TOTAL_BITS {
                let num_bits = parse_operator_num_bits(stream);
                println!("Operator num bits {}", num_bits);

                let end_pos = stream.pos + num_bits as usize;

                let mut args: Vec<u64> = Vec::new();

                stream.advance(1);

                while stream.pos < end_pos {
                    // stream.advance(1);
                    let val = parse_packet(stream, true);
                    args.push(val);
                }

                result = apply_op(kind, &args);

            } else { // LEN_KIND_SUBPACKETS
                let num_packets = parse_operator_num_packets(stream);
                println!("Operator num packets {}", num_packets);

                let mut args: Vec<u64> = Vec::new();

                stream.advance(1);

                for _ in 0..num_packets {
                    // stream.advance(1);
                    let val = parse_packet(stream, true);
                    args.push(val);
                }

                result = apply_op(kind, &args);
            }
        }

        if !is_subpacket {
            let _ = align_to_end_of_packet(stream);
        }

        result
    }

    let mut result = 0;

    stream.seek(0);
    loop {
        if stream.pos >= stream.len || (stream.len - stream.pos) < 11 {
            // Our smallest possible packet (without padding) is 11 bits,
            // so yield if we dont have that much left to read.
            break;
        }
        result =  parse_packet(&mut stream, false); 
    }

    println!("Result {}", result);
}