use gpui_ghostty_terminal::{TerminalConfig, TerminalSession};

fn screen_after(bytes: &[u8]) -> Vec<String> {
    let mut terminal = TerminalSession::new(TerminalConfig {
        cols: 10,
        rows: 5,
        ..Default::default()
    })
    .expect("terminal initializes");
    terminal.feed(bytes).expect("terminal accepts output");
    (0..5)
        .map(|row| {
            terminal
                .dump_viewport_row(row)
                .expect("viewport row is readable")
                .trim_end()
                .to_string()
        })
        .collect()
}

#[test]
fn erase_characters_clears_stale_text_without_moving_cursor() {
    assert_eq!(
        screen_after(b"abcdefghij\x1b[1;4H\x1b[4X!"),
        ["abc!   hij", "", "", "", ""]
    );
}

#[test]
fn delete_line_moves_following_rows_up() {
    assert_eq!(
        screen_after(b"one\x1b[2;1Htwo\x1b[3;1Hthree\x1b[4;1Hfour\x1b[2;1H\x1b[M"),
        ["one", "three", "four", "", ""]
    );
}

#[test]
fn insert_line_moves_following_rows_down() {
    assert_eq!(
        screen_after(b"one\x1b[2;1Htwo\x1b[3;1Hthree\x1b[2;1H\x1b[L"),
        ["one", "", "two", "three", ""]
    );
}

#[test]
fn delete_line_respects_scrolling_region() {
    assert_eq!(
        screen_after(
            b"title\x1b[2;1Hone\x1b[3;1Htwo\x1b[4;1Hthree\x1b[5;1Hfooter\x1b[2;4r\x1b[2;1H\x1b[M"
        ),
        ["title", "two", "three", "", "footer"]
    );
}
