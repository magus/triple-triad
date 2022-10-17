use unicode_width::UnicodeWidthStr;

pub fn box_text(text: &str, padding_u8: u8) -> String {
    let mut parts: Vec<String> = vec![];

    let padding = padding_u8 as usize;
    let padding_total: usize = padding * 2;
    let text_len = unicode_width(text);
    let length = text_len + padding_total;

    let side: Vec<&str> = (0..length).map(|_| "─").collect();
    let pad: Vec<&str> = (0..padding).map(|_| " ").collect();

    parts.push(format!("┌{}┐", side.join("")));
    parts.push(format!("│{}{}{}│", pad.join(""), text, pad.join("")));
    parts.push(format!("└{}┘", side.join("")));

    return parts.join("\n");
}

pub fn box_lines(lines: Vec<String>, padding_u8: u8) -> String {
    let mut parts: Vec<String> = vec![];

    let max_text_len = lines.iter().map(|l| unicode_width(l)).max().unwrap();

    let padding = padding_u8 as usize;
    let padding_total: usize = padding * 2;
    let length = max_text_len + padding_total;

    let side: Vec<&str> = (0..length).map(|_| "─").collect();
    let pad: Vec<&str> = (0..padding).map(|_| " ").collect();

    parts.push(format!("┌{}┐", side.join("")));

    for line in lines {
        let line_pad_len = max_text_len - unicode_width(&line);
        let line_pad: Vec<&str> = (0..line_pad_len).map(|_| " ").collect();
        parts.push(format!(
            "│{}{}{}{}│",
            pad.join(""),
            line,
            line_pad.join(""),
            pad.join("")
        ));
    }

    parts.push(format!("└{}┘", side.join("")));

    return parts.join("\n");
}

fn unicode_width(text: &str) -> usize {
    return UnicodeWidthStr::width(text);
}
