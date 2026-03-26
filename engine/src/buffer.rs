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

pub fn load_content(buffer: &mut GapBuffer, content: &[u8]) {
    let gap = buffer.gap_size;
    let total = gap + content.len();
    let mut new_buf = vec![0u8; total];
    new_buf[gap..].copy_from_slice(content);
    buffer.buffer = new_buf;
    buffer.gap_start = 0;
    buffer.gap_end = gap;
    buffer.gap_size = total;
}

pub fn get_content(buffer: &GapBuffer) -> Vec<u8> {
    let gap_len = buffer.gap_end - buffer.gap_start;
    let content_len = buffer.gap_size - gap_len;
    let mut out = Vec::with_capacity(content_len);
    out.extend_from_slice(&buffer.buffer[..buffer.gap_start]);
    out.extend_from_slice(&buffer.buffer[buffer.gap_end..buffer.gap_size]);
    out
}

pub fn cursor_position(buffer: &GapBuffer) -> (usize, usize) {
    let pre = &buffer.buffer[..buffer.gap_start];
    let row = pre.iter().filter(|&&b| b == b'\n').count();
    let col = pre.iter().rev().take_while(|&&b| b != b'\n').count();
    (row, col)
}

pub fn move_cursor_to(buffer: &mut GapBuffer, offset: usize) {
    while buffer.gap_start < offset {
        move_cursor_right(buffer);
    }
    while buffer.gap_start > offset {
        move_cursor_left(buffer);
    }
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