pub struct GapBuffer {
    pub buffer: Vec<u8>,
    pub gap_start: usize,
    pub gap_end: usize,
    pub gap_size: usize,
}

pub fn create_buffer() -> GapBuffer {
    GapBuffer {
        buffer: vec![0; 1024],
        gap_start: 0,
        gap_end: 1024,
        gap_size: 1024,
    }
}

pub fn move_cursor_left(buffer: &mut GapBuffer) {
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

pub fn move_cursor_right(buffer: &mut GapBuffer) {
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