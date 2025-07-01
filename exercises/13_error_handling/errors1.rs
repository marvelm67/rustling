fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // Jika nama kosong, kembalikan Err dengan pesan kesalahan
        Err("`name` was empty; it must be nonempty.".to_string())
    } else {
        // Jika nama tidak kosong, kembalikan Ok dengan teks nametag
        Ok(format!("Hi! My name is {name}"))
    }
}

fn main() {
    // Anda bisa bereksperimen di sini jika diperlukan.
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()).as_deref(),
            Ok("Hi! My name is Beyoncé"),
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        // Modify this to expect an `Err` with a `String`
        assert_eq!(
            generate_nametag_text(String::new()).as_ref().map_err(|e| e.as_str()),
            Err("`name` was empty; it must be nonempty."),
        );
    }
}
