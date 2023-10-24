use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

pub fn day06() {
    let filename = "dat/2022/day06.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Since P1 is smaller than P2 and doing a similar thing with history, we could just use the
    // larger array and have P1 track inside of it, but it isn't many bytes so we'll keep it simple
    const P1_BUFFER_LEN: usize = 4;
    let mut p1_buffer: [char; P1_BUFFER_LEN] = [ '\0'; P1_BUFFER_LEN ];
    let mut p1_buffer_pos = 0;

    const P2_BUFFER_LEN: usize = 14;
    let mut p2_buffer: [char; P2_BUFFER_LEN] = [ '\0'; P2_BUFFER_LEN ];
    let mut p2_buffer_pos = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let line_chars = line.chars().collect::<Vec<char>>();

        let mut found_start_of_packet: bool = false;
        let mut found_start_of_message: bool = false;

        for i in 0..line_chars.len() {
            p1_buffer[p1_buffer_pos] = line_chars[i];
            p1_buffer_pos = (p1_buffer_pos + 1) % P1_BUFFER_LEN;

            p2_buffer[p2_buffer_pos] = line_chars[i];
            p2_buffer_pos = (p2_buffer_pos + 1) % P2_BUFFER_LEN;

            if i >= 3 {
                if !found_start_of_packet {
                    let mut sorted = p1_buffer.clone();
                    sorted.sort();
                    let mut found_dupes = false;
                    for i in 0..P1_BUFFER_LEN-1 {
                       if sorted[i] == sorted[i + 1] {
                           found_dupes = true;
                           break;
                       }
                    }

                    if !found_dupes {
                        // [Part 1]
                        // Expected: 1140
                        println!("[Part 1] Start-of-packet marker after character {}", i + 1);
                        found_start_of_packet = true;
                    }
                }
            }

            if i >= 13 {
                if !found_start_of_message {
                    let mut sorted = p2_buffer.clone();
                    sorted.sort();
                    let mut found_dupes = false;
                    for i in 0..P2_BUFFER_LEN-1 {
                        if sorted[i] == sorted[i + 1] {
                            found_dupes = true;
                            break;
                        }
                    }

                    if !found_dupes {
                        // [Part 2]
                        // Expected: 3495
                        println!("[Part 2] Start-of-message marker after character {}", i + 1);
                        found_start_of_message = true;
                    }
                }
            }

            if found_start_of_packet && found_start_of_message {
                break;
            }
        }

        // Clean out the buffers between lines
        for i in 0..P1_BUFFER_LEN {
            p1_buffer[i] = '\0';
        }

        for i in 0..P2_BUFFER_LEN {
            p2_buffer[i] = '\0';
        }
    }
}
