
// OCaml Buffer type
ocaml::custom!(GapBuffer);


// Struct for the buffer, handles the actual buffer
// as well as the managing the start and end indices
pub struct GapBuffer {
    pub buffer: Vec<u8>,
    pub gap_start: usize,
    pub gap_end: usize,
    pub gap_size: usize,
}


// Function to create a new buffer
#[ocaml::func]
pub fn create_buffer() -> ocaml::Pointer<GapBuffer> {
    let buf = GapBuffer {
        buffer: vec![0; 1024], // <- can increased
        gap_start: 0,
        gap_end: 1024,
        gap_size: 1024,
    };
    // send the buffer to OCaml
    ocaml::Pointer::alloc_custom(buf)
}


// Function to move the cursor left
#[ocaml::func]
pub fn move_cursor_left(mut pointer: ocaml::Pointer<GapBuffer>) {
    // get a mutable reference to the buffer
    let buffer = pointer.as_mut();

    
    // if the buffer is at 0, then cant move left
    if buffer.gap_start == 0 {
        return;
    }
    // find the start of the current character
    let mut i = buffer.gap_start - 1;
    while i > 0 && (buffer.buffer[i] & 0b1100_0000) == 0b1000_0000 {
        i -= 1;
    }

    // shift the current character to the gap
    let char_start = i;
    let char_len = buffer.gap_start - char_start;

    // shift the current character to the gap
    for j in 0..char_len {
        buffer.buffer[buffer.gap_end - char_len + j] = buffer.buffer[char_start + j];
    }

    // move the gap
    buffer.gap_start -= char_len;
    buffer.gap_end -= char_len;
}


// Function to move the cursor right
#[ocaml::func]
pub fn move_cursor_right(mut pointer: ocaml::Pointer<GapBuffer>) {
    // get a mutable reference to the buffer
    let buffer = pointer.as_mut();
    // if the gap is at the end, then cant move right
    if buffer.gap_end == buffer.gap_size {
        return;
    }
    // find the start of the current character
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

    // shift the current character to the gap
    for i in 0..char_len {
        buffer.buffer[buffer.gap_start + i] = buffer.buffer[buffer.gap_end + i];
    }

    // move the gap
    buffer.gap_start += char_len;
    buffer.gap_end += char_len;
    
}


