pub mod crypto;
pub mod dict;

#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::crypto::file_decode::decode_file;
    use crate::crypto::file_encode::encode_file;

    // 图片编码
    #[test]
    pub fn test_encode_img() {
        encode_file(
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\12179.jpg",
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\12179.txt",
        );
    }

    // 图片解码
    #[test]
    pub fn test_decode_img() {
        decode_file(
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\12179.txt",
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\12179_decode.jpg",
        );
    }

    // 视频编码
    #[test]
    pub fn test_decode_video() {
        let video_vec = vec![
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test\test_dashinit.mp4",
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test\test_dash1.m4s",
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test\test_dash2.m4s",
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test\test_dash3.m4s",
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test\test_dash4.m4s",
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test\test_dash5.m4s",
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test\test_dash6.m4s",
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test\test_dash7.m4s",
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test\test_dash8.m4s",
        ];

        for input_path  in video_vec {
            let input_path = Path::new(input_path);
            let output_path = input_path.with_extension("txt");
            encode_file(
                input_path.to_str().unwrap(),
                output_path.to_str().unwrap(),
            );
        }
        
      
    }
}
