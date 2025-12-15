mod bindings {
    use wit_bindgen::generate;
    generate!({
        path: ["../wit"],
        world: "encoder-decoder-service",
        async: true,
        generate_all,
    });

    use super::CaesarCipherComponent;
    export!(CaesarCipherComponent);
}

use bindings::saritnike::cipher::audit_log;

struct CaesarCipherComponent;

impl bindings::Guest for CaesarCipherComponent {

    async fn encr(input: String,) -> String {
        // provided by the host:
        audit_log::auditrecord("encrypt".to_string(), format!("Length: {}", input.len())).await;
        let shift = get_shift();
        shift_string(&input, shift)
    }

    async fn decr(input: String,) -> String {
        // provided by the host:
        audit_log::auditrecord("decrypt".to_string(), format!("Length: {}", input.len())).await;
        let shift = get_shift();
        shift_string(&input, 26-(shift % 26))
    }

}


fn get_shift() -> u8 {
    match std::env::var("SHIFT_AMOUNT") {
        Ok(val) => val.parse().unwrap_or(3),
        Err(_) => 3,
    }
}

fn shift_string(input: &str, shift: u8) -> String {
    input.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let offset = c as u8 - base;
            let new_offset = (offset + shift) % 26;
            (base + new_offset) as char
        } else {
            c
        }
    }).collect()
}