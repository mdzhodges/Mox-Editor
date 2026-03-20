fn move_cursor_left(buffer: &mut Vec<u8>, gap_start: &mut usize, gap_end: &mut usize) {
    // If we're already at the beginning, we can't move left
    if *gap_start == 0 {
        return;
    }

    // Start one byte to the left of the gap
    let mut i = *gap_start - 1;

    // Walk backward over UTF-8 continuation bytes
    // Continuation bytes have the form: 10xxxxxx
    //
    // (byte & 0b1100_0000) isolates the top two bits
    // If those bits equal 0b1000_0000 → it's a continuation byte
    //
    // So: keep moving left while we're still inside a multi-byte character
    while i > 0 && (buffer[i] & 0b1100_0000) == 0b1000_0000 {
        i -= 1;
    }

    // Now `i` points to the FIRST byte of the previous character
    let char_start = i;

    // The number of bytes in this character
    // (could be 1–4 bytes depending on UTF-8 encoding)
    let char_len = *gap_start - char_start;

    //
    // We copy the bytes into the space just before gap_end
    for j in 0..char_len {
        buffer[*gap_end - char_len + j] = buffer[char_start + j];
    }

    // Slide the gap left by the character length
    //
    // Both ends move together to preserve gap size
    *gap_start -= char_len;
    *gap_end -= char_len;
}

fn move_cursor_right(buffer: &mut Vec<u8>, gap_start: &mut usize, gap_end: &mut usize) {
    // If gap_end is at the end of the buffer, nothing to move
    if *gap_end == buffer.len() {
        return;
    }

    // Look at the first byte after the gap
    let first_byte = buffer[*gap_end];

    // Determine the length of the UTF-8 character from the leading byte
    //
    // UTF-8 patterns:
    // 0xxxxxxx → 1 byte (ASCII)
    // 110xxxxx → 2 bytes
    // 1110xxxx → 3 bytes
    // 11110xxx → 4 bytes
    let char_len = if first_byte & 0b1000_0000 == 0 {
        // Top bit is 0 → ASCII → 1 byte
        1
    } else if first_byte & 0b1110_0000 == 0b1100_0000 {
        2
    } else if first_byte & 0b1111_0000 == 0b1110_0000 {
        3
    } else {
        4
    };

    for i in 0..char_len {
        buffer[*gap_start + i] = buffer[*gap_end + i];
    }

    // Slide the gap right by the character length
    *gap_start += char_len;
    *gap_end += char_len;
}


debug_assert!(std::str::from_utf8(&buffer[..*gap_start]).is_ok());
debug_assert!(std::str::from_utf8(&buffer[*gap_end..]).is_ok());