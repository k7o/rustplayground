

enum Voorbeeld {
    U8Waarde(u8),
    U16Waarde(u16),
    U32Waarde(u32),
}

fn main() {
    let voorbeeld = Voorbeeld::U16Waarde(4004);
    let voorbeeld2 = Voorbeeld::U32Waarde(8008);

    
    match_voorbeeld(&voorbeeld);
    match_voorbeeld(&voorbeeld2);
    
}

fn match_voorbeeld(voorbeeld: &Voorbeeld) {
    match voorbeeld {
        Voorbeeld::U8Waarde(u8_waarde) => println!("u8 waarde: {}", u8_waarde),
        Voorbeeld::U16Waarde(u16_waarde) => println!("u16 waarde: {}", u16_waarde),
        Voorbeeld::U32Waarde(u32_waarde) => println!("u32 waarde: {}", u32_waarde),
    }
}
