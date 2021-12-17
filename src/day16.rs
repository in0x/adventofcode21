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

    /// Reads num_bits from the stream currents pos into the returned value.
    /// NOTE: Reading does not advance the current stream position!
    pub fn read(&self, num_bits: usize) -> u64 {
        assert!((self.pos + 1) >= num_bits);    
    
        let mut result = 0;

        for i in 0..num_bits {
            let byte_idx = self.pos - i;
            let this_byte = {
                let aligned_idx = byte_idx / 8;
                self.bits[aligned_idx]
            };
            let bit_idx = byte_idx % 8;
            let ith_bit = {
                let shift_by = 7 - bit_idx;
                (this_byte & (1 << shift_by)) >> shift_by
            } as u64; 

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

    // for b in &stream.bits {
    //     print!("{:08b}'", b);
    // }
    // println!("");

    fn align_to_n(val: usize, align_to: usize) -> usize {
        (val + (align_to - 1)) & (!(align_to - 1))
    }

    fn align_to_end_of_packet(stream: &mut BitStream) -> Result<(), ()> {
        let next_aligned_pos = align_to_n(stream.pos, 4);
        let result = stream.try_seek(next_aligned_pos); // Try to skip past padding to next package
        result
    }

    fn parse_header(stream: &mut BitStream) -> (u64, u64) {
        stream.advance(2); // Move to the inital version bits
        let version = stream.read(3);

        stream.advance(3); // Move to end of type_id bits
        let type_id = stream.read(3);

        (version, type_id)
    }

    fn parse_literal_packet(stream: &mut BitStream) -> u64 {
        let mut digits = Vec::new();
        digits.reserve(8);
        loop {
            stream.advance(1); // Move to "keep_going" bit
            let keep_going = stream.read(1);
            
            stream.advance(4); // Move to next 4 bits of literal
            digits.push(stream.read(4));

            if keep_going == 0 {
                break;
            }
        }

        let mut result = 0;
        let mut count = 0;
        for digit in digits.iter().rev() {
            result |= digit << (4 * count);
            count += 1;
        } 

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
                *args.iter().min().unwrap()
            },
            OP_KIND_MAX => {
                *args.iter().max().unwrap()
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

    #[allow(unused)]
    fn op_to_str(kind: u64) -> &'static str {
        match kind {
            OP_KIND_ADD => "add",
            OP_KIND_MUL => "mul", 
            OP_KIND_MIN => "min",
            OP_KIND_MAX => "max",
            OP_KIND_GT  => "gt", 
            OP_KIND_LT  => "lt", 
            OP_KIND_EQ  => "eq", 
            _ => panic!(),
        }
    }

    fn parse_packet(stream: &mut BitStream, is_subpacket: bool) -> u64 {
        if is_subpacket { 
            stream.advance(1)
        }

        let (_, kind) = parse_header(stream);

        let mut result = 0;

        if kind == PACKET_KIND_LITERAL {
            result = parse_literal_packet(stream);
        } 
        else { // PACKET_KIND_OPERATOR
            stream.advance(1);
            let len_kind = stream.read(1);
  
            if len_kind == LEN_KIND_TOTAL_BITS {
                let num_bits = parse_operator_num_bits(stream);

                let end_pos = stream.pos + num_bits as usize;

                let mut args: Vec<u64> = Vec::new();

                while stream.pos < end_pos {
                    let val = parse_packet(stream, true);
                    args.push(val);
                }

                result = apply_op(kind, &args);
                // println!("Len Op {} ({:?}) -> {}", op_to_str(kind), args, result);
            } else { // LEN_KIND_SUBPACKETS
                let num_packets = parse_operator_num_packets(stream);
                let mut args: Vec<u64> = Vec::new();

                for _ in 0..num_packets {
                    let val = parse_packet(stream, true);
                    args.push(val);
                }

                result = apply_op(kind, &args);
                // println!("Pak Op {} ({:?}) -> {}", op_to_str(kind), args, result);
            }
        }

        if !is_subpacket {
            let _ = align_to_end_of_packet(stream);
        }

        result
    }

    let mut result = 0;

    stream.seek(0);
    result =  parse_packet(&mut stream, false); 

    println!("Result {}", result);

    // let mut test_stream = BitStream::new();

    // fn write_bit(stream: &mut BitStream, bit: u8) {
    //     let byte_idx = stream.pos;
    //     if stream.len <= byte_idx {
    //         stream.push(0);
    //     }

    //     let bit_idx = byte_idx % 8;
    //     let byte_ref = &mut stream.bits[byte_idx / 8];
    //     // *byte_ref |= bit << bit_idx;
            
    //     let shift_by = 7 - bit_idx;
    //     *byte_ref |= bit << shift_by;

    //     stream.pos += 1;
    // }

    // fn write_header(stream: &mut BitStream, kind: u64) {
    //     write_bit(stream, 0);
    //     write_bit(stream, 0);
    //     write_bit(stream, 0);

    //     let kind8 = kind as u8;
    //     write_bit(stream, (kind8 >> 2) & 1);
    //     write_bit(stream, (kind8 >> 1) & 1);
    //     write_bit(stream, kind8 & 1);
    // }

    // fn write_op_len(stream: &mut BitStream, bit_len: u32) {
    //     write_bit(stream, 0);
    //     write_bit(stream, ((bit_len >> 14) & 1) as u8);
    //     write_bit(stream, ((bit_len >> 13) & 1) as u8);
    //     write_bit(stream, ((bit_len >> 12) & 1) as u8);
    //     write_bit(stream, ((bit_len >> 11) & 1) as u8);
    //     write_bit(stream, ((bit_len >> 10) & 1) as u8);
    //     write_bit(stream, ((bit_len >> 9) & 1) as u8);
    //     write_bit(stream, ((bit_len >> 8) & 1) as u8);
    //     write_bit(stream, ((bit_len >> 7) & 1) as u8);
    //     write_bit(stream, ((bit_len >> 6) & 1) as u8);
    //     write_bit(stream, ((bit_len >> 5) & 1) as u8);
    //     write_bit(stream, ((bit_len >> 4) & 1) as u8);
    //     write_bit(stream, ((bit_len >> 3) & 1) as u8);
    //     write_bit(stream, ((bit_len >> 2) & 1) as u8);
    //     write_bit(stream, ((bit_len >> 1) & 1) as u8);
    //     write_bit(stream,  (bit_len       & 1) as u8);
    // }

    // fn write_op_packets(stream: &mut BitStream, num_packets: u32) {
    //     write_bit(stream, 1);
    //     write_bit(stream, ((num_packets >> 10) & 1) as u8);
    //     write_bit(stream, ((num_packets >> 9) & 1) as u8);
    //     write_bit(stream, ((num_packets >> 8) & 1) as u8);
    //     write_bit(stream, ((num_packets >> 7) & 1) as u8);
    //     write_bit(stream, ((num_packets >> 6) & 1) as u8);
    //     write_bit(stream, ((num_packets >> 5) & 1) as u8);
    //     write_bit(stream, ((num_packets >> 4) & 1) as u8);
    //     write_bit(stream, ((num_packets >> 3) & 1) as u8);
    //     write_bit(stream, ((num_packets >> 2) & 1) as u8);
    //     write_bit(stream, ((num_packets >> 1) & 1) as u8);
    //     write_bit(stream, (       num_packets & 1) as u8);
    // }

    // fn write_literal(stream: &mut BitStream, value: u8) {
    //     write_bit(stream, 0);

    //     write_bit(stream, (value >> 3) & 1);
    //     write_bit(stream, (value >> 2) & 1);
    //     write_bit(stream, (value >> 1) & 1);
    //     write_bit(stream, value & 1);
    // }

    // write_header(&mut test_stream, OP_KIND_ADD);     // do an add of
    // write_op_packets(&mut test_stream, 3);
    // write_header(&mut test_stream, _OP_KIND_LITERAL); // 7
    // write_literal(&mut test_stream, 7);
    // write_header(&mut test_stream, _OP_KIND_LITERAL); // 15
    // write_literal(&mut test_stream, 15);

    // write_header(&mut test_stream, OP_KIND_MAX); // and a max with
    // write_op_packets(&mut test_stream, 2);

    // write_header(&mut test_stream, OP_KIND_GT); // a greate of
    // write_op_packets(&mut test_stream, 2);
    // write_header(&mut test_stream, _OP_KIND_LITERAL); // 5
    // write_literal(&mut test_stream, 5);
    // write_header(&mut test_stream, _OP_KIND_LITERAL); // and 3
    // write_literal(&mut test_stream, 3);

    // write_header(&mut test_stream, OP_KIND_MUL);  // and a mul of 
    // write_op_packets(&mut test_stream, 3);
    // write_header(&mut test_stream, _OP_KIND_LITERAL); // 1 
    // write_literal(&mut test_stream, 1);
    // write_header(&mut test_stream, _OP_KIND_LITERAL); // 2
    // write_literal(&mut test_stream, 2);
    
    // write_header(&mut test_stream, OP_KIND_ADD); // and a add of 
    // write_op_packets(&mut test_stream, 3);
    // write_header(&mut test_stream, _OP_KIND_LITERAL); // 13
    // write_literal(&mut test_stream, 13);
    
    // write_header(&mut test_stream, OP_KIND_MAX); // and a max of 
    // write_op_packets(&mut test_stream, 2);

    // write_header(&mut test_stream, OP_KIND_ADD); // and an add of
    // write_op_packets(&mut test_stream, 4);
    // write_header(&mut test_stream, _OP_KIND_LITERAL); // 7
    // write_literal(&mut test_stream, 7);
    // write_header(&mut test_stream, _OP_KIND_LITERAL); // 6
    // write_literal(&mut test_stream, 6);
    // write_header(&mut test_stream, _OP_KIND_LITERAL); // 8
    // write_literal(&mut test_stream, 8);
    // write_header(&mut test_stream, _OP_KIND_LITERAL); // 9
    // write_literal(&mut test_stream, 9);

    // write_header(&mut test_stream, OP_KIND_MIN); // and a min of
    // write_op_packets(&mut test_stream, 2);
    // write_header(&mut test_stream, _OP_KIND_LITERAL); // 0
    // write_literal(&mut test_stream, 0);
    // write_header(&mut test_stream, _OP_KIND_LITERAL); // and 1
    // write_literal(&mut test_stream, 1);

    // write_header(&mut test_stream, _OP_KIND_LITERAL); // and 1
    // write_literal(&mut test_stream, 4);

    // for b in &test_stream.bits {
    //     print!("{:08b}'", b);
    // }
    // println!("");

    // test_stream.seek(0);
    // println!("test result {}", parse_packet(&mut test_stream, false)); 
}