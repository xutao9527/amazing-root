use crate::crypto::unicode_crypto::{decode_unicode_to_file, encode_file_to_unicode};


mod crypto;

fn main()-> std::io::Result<()> {
    //generate_unicode_crypto_dict()?;
    let input = r"C:\Users\Administrator\Desktop\unicode\12179.jpg";
    let output = r"C:\Users\Administrator\Desktop\unicode\12179.txt";
    let decode = r"C:\Users\Administrator\Desktop\unicode\12179_decode.jpg";
    encode_file_to_unicode(input, output)?;
    decode_unicode_to_file(output,decode)?;
    Ok(())
}