// Register the type with the OCaml runtime
ocaml::custom!(GapBuffer);

pub struct GapBuffer {
    pub buffer: Vec<u8>,
    pub gap_start: usize,
    pub gap_end: usize,
    pub gap_size: usize,
}

// You need a way to instantiate the buffer from OCaml
#[ocaml::func]
pub fn create_buffer() -> ocaml::Pointer<GapBuffer> {
    let buf = GapBuffer {
        buffer: vec![0; 1024], // Example size
        gap_start: 0,
        gap_end: 1024,
        gap_size: 1024,
    };
    ocaml::Pointer::alloc_custom(buf)
}

#[ocaml::func]
pub fn move_cursor_left(mut pointer: ocaml::Pointer<GapBuffer>) {
    let buffer = pointer.as_mut(); // Get a mutable reference to the Rust struct

    if buffer.gap_start == 0 {
        return;
    }

    let mut i = buffer.gap_start - 1;
    while i > 0 && (buffer.buffer[i] & 0b1100_0000) == 0b1000_0000 {
        i -= 1;
    }

    let char_start = i;
    let char_len = buffer.gap_start - char_start;

    for j in 0..char_len {
        buffer.buffer[buffer.gap_end - char_len + j] = buffer.buffer[char_start + j];
    }

    buffer.gap_start -= char_len;
    buffer.gap_end -= char_len;
}

#[ocaml::func]
pub fn move_cursor_right(mut pointer: ocaml::Pointer<GapBuffer>) {
    let buffer = pointer.as_mut();
    if buffer.gap_end == buffer.gap_size {
        return;
    }

    let first_byte = buffer.buffer[buffer.gap_end];
    let char_len = if first_byte & 0b1000_0000 == 0 {
        1
    } else if first_byte & 0b1110_0000 == 0b1100_0000 {
        2
    } else if first_byte & 0b1111_0000 == 0b1110_0000 {
        3
    } else {
        4
    };

    for i in 0..char_len {
        buffer.buffer[buffer.gap_start + i] = buffer.buffer[buffer.gap_end + i];
    }

    buffer.gap_start += char_len;
    buffer.gap_end += char_len;
    
}

#[ocaml::func]
pub fn debug_print(pointer: ocaml::Pointer<GapBuffer>) {
    let buffer = pointer.as_ref();
    println!("Gap Start: {} | Gap End: {}", buffer.gap_start, buffer.gap_end);
}

#[ocaml::func]
pub fn insert_string(mut pointer: ocaml::Pointer<GapBuffer>, text: String) {
    let buffer = pointer.as_mut();
    let bytes = text.as_bytes();
    let len = bytes.len();

    // Copy bytes into the start of the gap, then shrink the gap
    if buffer.gap_start + len <= buffer.gap_end {
        buffer.buffer[buffer.gap_start..buffer.gap_start + len].copy_from_slice(bytes);
        buffer.gap_start += len;
    }
}