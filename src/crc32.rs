use std::cmp::min;

use crc32fast::Hasher;

use crate::file::File;

pub fn find(input: &String, hash_str: &String) {
    let hash = u32::from_str_radix(&hash_str, 16).unwrap();
    let mut file = File::new(&input);

    let size = file.get_size();
    let mut pos = 0;
    let mut length = 1;

    loop {
        if pos + length > size {
            println!("No results found.");
            break;
        }

        file.seek(&pos);
        let mut hasher = Hasher::new();
        let mut buf = vec![0; min(length as usize, 4096)];

        let mut remaining = length;
        while remaining > 0 {
            let to_read = min(4096, remaining) as usize;
            let read = file.read(&mut buf[..to_read]);
            hasher.update(read);
            remaining -= to_read as u64;
        }

        if hasher.finalize() == hash {
            println!("Found at position {} with length {}.", pos, length);
            break;
        }

        length += 1;
        if pos + length > size {
            pos += 1;
            length = 1;
        }
    }
}
