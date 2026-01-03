const DENSITY_RAMP: [char; 9] = [' ', '.', ',', ':', ';', 'o', 'x', '%', '#'];

pub fn parse_ascii_art(text: &str) -> Vec<String> {
    let mut lines: Vec<String> = text
        .lines()
        .map(|line| line.trim_end_matches('\r').to_string())
        .collect();

    while matches!(lines.last(), Some(last) if last.is_empty()) {
        lines.pop();
    }

    lines
}

pub fn dimensions(lines: &[String]) -> (usize, usize) {
    let width = lines
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);
    (width, lines.len())
}

pub fn fit_dimensions(
    (in_w, in_h): (usize, usize),
    (max_w, max_h): (usize, usize),
) -> (usize, usize) {
    let max_w = max_w.max(1);
    let max_h = max_h.max(1);

    if in_w == 0 || in_h == 0 {
        return (1, 1);
    }

    let mut out_w = in_w;
    let mut out_h = in_h;

    if out_w > max_w {
        out_w = max_w;
        out_h = scaled_dimension(in_h, out_w, in_w);
    }

    if out_h > max_h {
        out_h = max_h;
        out_w = scaled_dimension(in_w, out_h, in_h);
    }

    (out_w.max(1), out_h.max(1))
}

pub fn downsample(lines: &[String], out_w: usize, out_h: usize) -> Vec<String> {
    let out_w = out_w.max(1);
    let out_h = out_h.max(1);

    let (in_w, in_h) = dimensions(lines);
    if in_w == 0 || in_h == 0 {
        return vec![String::new(); out_h];
    }

    let mut matrix: Vec<Vec<char>> = Vec::with_capacity(in_h);
    for line in lines {
        let mut chars: Vec<char> = line.chars().collect();
        if chars.len() < in_w {
            chars.resize(in_w, ' ');
        }
        matrix.push(chars);
    }

    let mut out_lines = Vec::with_capacity(out_h);
    for y_out in 0..out_h {
        let y0 = y_out * in_h / out_h;
        let mut y1 = (y_out + 1) * in_h / out_h;
        if y1 <= y0 {
            y1 = (y0 + 1).min(in_h);
        }

        let mut row = String::with_capacity(out_w);
        for x_out in 0..out_w {
            let x0 = x_out * in_w / out_w;
            let mut x1 = (x_out + 1) * in_w / out_w;
            if x1 <= x0 {
                x1 = (x0 + 1).min(in_w);
            }

            let mut sum: u32 = 0;
            let mut count: u32 = 0;
            for row in matrix.iter().take(y1).skip(y0) {
                for ch in row.iter().take(x1).skip(x0) {
                    sum += u32::from(density_index(*ch));
                    count += 1;
                }
            }

            let avg = (sum + count / 2) / count;
            let idx = usize::try_from(avg).unwrap_or(0);
            let ch = DENSITY_RAMP[idx.min(DENSITY_RAMP.len() - 1)];
            row.push(ch);
        }
        out_lines.push(row);
    }

    out_lines
}

fn scaled_dimension(input: usize, output_primary: usize, input_primary: usize) -> usize {
    (input * output_primary + input_primary / 2) / input_primary
}

fn density_index(ch: char) -> u8 {
    match ch {
        ' ' => 0,
        '.' => 1,
        ',' => 2,
        ':' => 3,
        ';' => 4,
        'o' => 5,
        'x' => 6,
        '%' => 7,
        '#' => 8,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ascii_art_trims_trailing_empty_lines() {
        let lines = parse_ascii_art("a\nb\n\n\n");
        assert_eq!(lines, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn fit_dimensions_scales_down_preserving_aspect_ratio() {
        assert_eq!(fit_dimensions((100, 24), (60, 18)), (60, 14));
        assert_eq!(fit_dimensions((100, 24), (30, 7)), (30, 7));
    }

    #[test]
    fn downsample_emits_requested_dimensions() {
        let input = vec!["# ".to_string(), " .".to_string()];
        let out = downsample(&input, 3, 1);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].chars().count(), 3);
    }

    #[test]
    fn downsample_identity_preserves_known_ramp_chars() {
        let input = vec![".,:;".to_string(), "ox%#".to_string()];
        let out = downsample(&input, 4, 2);
        assert_eq!(out, input);
    }
}
