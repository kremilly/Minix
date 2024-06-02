use crate::minix::minify::Minify;

use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::Path,
};


pub struct Engine;

impl Engine {

    fn read(input: &str) -> Result<String, Box<dyn Error>> {
        let content = if Path::new(&input).is_file() {
            fs::read_to_string(&input)?
        } else {
            "".to_string()
        };

        Ok(content)
    }

    fn scan_path(input: &str, filter: &str, output: Option<&str>) -> Result<(), Box<dyn Error>> {
        let paths = fs::read_dir(input)?;

        if let Some(output) = output {
            let mut content = String::new();

            for path in paths {
                let path = path?.path();
                let path_str = path.to_str().unwrap();

                if path.is_file() && !path_str.contains(".min") && path_str.ends_with(filter) {
                    let content_file = Self::read(path_str)?;
                    content.push_str(&content_file);
                }
            }

            let output_file = output.to_string();
            Self::append_write(&content, input, &output_file, filter)?;

            return Ok(());
        } else {
            for path in paths {
                let path = path?.path();
                let path_str = path.to_str().unwrap();
    
                if path.is_file() && !path_str.contains(".min") && path_str.ends_with(filter) {
                    let output_file = path.to_string_lossy().replace(
                        filter, format!("min.{}", filter).as_str()
                    );
                    
                    Self::write(path_str, &output_file)?;
                }
            }
        }

        Ok(())
    }

    fn write(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
        let content = Self::read(input)?;
        let content_minified = if input.ends_with("js") {
            Minify::js(&content)
        } else {
            Minify::css(&content)
        };

        let mut file = File::create(output)?;
        file.write_all(content_minified.as_bytes())?;

        println!("File minified from {} to {} successfully!", input, output);
        Ok(())
    }
    
    fn append_write(content: &str, input: &str, output: &str, filter: &str) -> Result<(), Box<dyn Error>> {
        let content_minified = if filter == "js" {
            Minify::js(&content)
        } else {
            Minify::css(&content)
        };

        let mut file = File::create(output)?;
        file.write_all(content_minified.as_bytes())?;

        println!("File minified from {} to {} successfully!", input, output);
        Ok(())
    }

    pub fn run(input: &str, output: Option<&str>) -> Result<(), Box<dyn Error>> {
        if input.contains("*") {
            let filter: Vec<&str> = input.split("*.").collect();
            Self::scan_path(filter[0], filter[1], output)?;
        } else {
            Self::write(input, output.unwrap_or(""))?; // Dereference the output parameter
        }

        Ok(())
    }

}
