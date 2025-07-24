pub mod crypto;



#[cfg(test)]
mod tests {
    use crate::crypto::file_crypto::{decrypt_file, encrypt_file};

    // 文件加密(图片)
    #[test]
    pub fn test_encode_img() {
        encrypt_file(
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test_crypto\test.jpg",
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test_crypto\test.txt",
        );
    }

    // 文件解密(图片)
    #[test]
    pub fn test_decode_img() {
        decrypt_file(
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test_crypto\test.txt",
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test_crypto\test_decode.jpg",
        );
    }

    // 视频编码
    #[test]
    pub fn test_decode_video() {
        // let video_vec = vec![
        //     r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test\test_dashinit.mp4",
        //     r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test\test_dash1.m4s",
        //     r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test\test_dash2.m4s",
        //     r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test\test_dash3.m4s",
        //     r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test\test_dash4.m4s",
        //     r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test\test_dash5.m4s",
        //     r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test\test_dash6.m4s",
        //     r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test\test_dash7.m4s",
        //     r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test\test_dash8.m4s",
        // ];

        // for input_path  in video_vec {
        //     let input_path = Path::new(input_path);
        //     let output_path = input_path.with_extension("txt");
        //     encode_file(
        //         input_path.to_str().unwrap(),
        //         output_path.to_str().unwrap(),
        //     );
        // }
    }
}
