const INPUT: &str = include_str!("..\\..\\inputs\\day16.txt");

struct PacketDataStream<'a> {
    nibbles: &'a [u8],
    index: usize,
    bit_offset: u8,
    cur_nibble: u8,
}

impl<'a> PacketDataStream<'a> {
    fn new(packet: &[u8]) -> PacketDataStream {
        let first_nibble = PacketDataStream::parse_nibble(packet[0]);

        PacketDataStream {
            nibbles: packet,
            index: 0,
            bit_offset: 0,
            cur_nibble: first_nibble,
        }
    }

    fn position(&self) -> usize {
        (4 * self.index) + self.bit_offset as usize
    }

    fn parse_nibble(c: u8) -> u8 {
        if (b'0'..=b'9').contains(&c) {
            c - b'0'
        } else if (b'A'..=b'F').contains(&c) {
            c - b'A' + 10
        } else {
            0
        }
    }

    fn read_bits(&mut self, count: u8) -> u16 {
        let mut result: u16 = 0;
        for _ in 0..count {
            result <<= 1;
            result |= self.next_bit()
        }

        result
    }

    #[inline(always)]
    fn next_bit(&mut self) -> u16 {
        let mask = 1 << (3 - self.bit_offset);
        let bit = (self.cur_nibble & mask) >> (3 - self.bit_offset);

        self.bit_offset += 1;
        if self.bit_offset == 4 {
            self.bit_offset = 0;
            self.index += 1;
            if self.index < self.nibbles.len() {
                self.cur_nibble = PacketDataStream::parse_nibble(self.nibbles[self.index]);
            }
        }

        bit as u16
    }

    fn has_data(&self) -> bool {
        self.index < self.nibbles.len()
    }
}

pub fn solve_part_1() {
    let mut stream = PacketDataStream::new(INPUT.as_bytes());

    let packet = read_packet(&mut stream);

    let mut to_process: Vec<BITSPacket> = Vec::new();
    to_process.push(packet);

    let mut ans: usize = 0;

    while to_process.len() > 0 {
        let p = to_process.pop().unwrap();
        ans += p.version as usize;

        for c in p.sub_packets {
            to_process.push(c);
        }
    }

    println!("Day #16 Part 1: {}", ans);
}

#[derive(Debug)]
struct BITSPacket {
    version: u8,
    packet_type: u8,
    literal: u64,
    length_type_id: u8,
    sub_packets: Vec<BITSPacket>,
    size_in_bits: usize,
}

impl BITSPacket {
    pub fn new() -> BITSPacket {
        BITSPacket {
            version: 0,
            packet_type: 0,
            literal: 0,
            length_type_id: 0,
            sub_packets: Vec::new(),
            size_in_bits: 0,
        }
    }
}

fn read_packet(stream: &mut PacketDataStream) -> BITSPacket {
    let mut packet = BITSPacket::new();
    let stream_start_pos = stream.position();
    packet.version = stream.read_bits(3) as u8;
    packet.packet_type = stream.read_bits(3) as u8;

    if packet.packet_type == 4 {
        let mut last_group = stream.read_bits(1) == 0;
        while !last_group {
            let literal_chunk = stream.read_bits(4);
            packet.literal = (packet.literal << 4) | literal_chunk as u64;
            last_group = stream.read_bits(1) == 0;
        }
        /* read in the last literal chunk */
        let final_chunk = stream.read_bits(4);
        packet.literal = (packet.literal << 4) | final_chunk as u64;
    } else {
        /* this is an operator */
        packet.length_type_id = stream.read_bits(1) as u8;
        if packet.length_type_id == 1 {
            /* next 11 bits contain how many packets we have */
            let sub_count = stream.read_bits(11);

            for _ in 0..sub_count {
                packet.sub_packets.push(read_packet(stream));
            }
        }

        if packet.length_type_id == 0 {
            let total_sub_bits: usize = stream.read_bits(15) as usize;
            let mut read_bits: usize = 0;

            while read_bits < total_sub_bits {
                let sub_packet = read_packet(stream);
                read_bits += sub_packet.size_in_bits;
                packet.sub_packets.push(sub_packet);
            }
        }

        /* Process operators */
        packet.literal = match packet.packet_type {
            0 => packet.sub_packets.iter().map(|p| p.literal).sum(),

            1 => packet.sub_packets.iter().map(|p| p.literal).product(),

            2 => packet.sub_packets.iter().map(|p| p.literal).min().unwrap(),

            3 => packet.sub_packets.iter().map(|p| p.literal).max().unwrap(),

            5 => {
                if packet.sub_packets[0].literal > packet.sub_packets[1].literal {
                    1
                } else {
                    0
                }
            }

            6 => {
                if packet.sub_packets[0].literal < packet.sub_packets[1].literal {
                    1
                } else {
                    0
                }
            }

            7 => {
                if packet.sub_packets[0].literal == packet.sub_packets[1].literal {
                    1
                } else {
                    0
                }
            }

            _ => unreachable!(),
        }
    }

    let stream_stop_pos = stream.position();
    packet.size_in_bits = stream_stop_pos - stream_start_pos;
    packet
}

pub fn solve_part_2() {
    let mut stream = PacketDataStream::new(INPUT.as_bytes());

    let packet = read_packet(&mut stream);

    println!("Day #16 Part 2: {}", packet.literal);
}
