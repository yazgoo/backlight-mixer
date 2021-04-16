use std::fs;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::env;

struct Backlight {
    path: String, 
}

type Percent = u8;

const BACKLIGHT_DIRECTORY: &str = "/sys/class/backlight";

fn list_backlights() -> Result<Vec<Backlight>, Box<dyn Error>> {
    let paths = fs::read_dir(BACKLIGHT_DIRECTORY)?;
    Ok(paths.flat_map(|path| {
        path.map(|x| {
            let file_name = x.file_name();
            file_name.into_string().map ( |n| Backlight { path: format!("{}/{}", &BACKLIGHT_DIRECTORY, n) } )
        })}).flatten().collect())
}

impl Backlight {

    fn read_file(&self, file: &str) -> Result<u32, Box<dyn Error>> {
        let mut x = fs::read_to_string(format!("{}/{}", self.path, file))?;
        x.truncate(x.len() - 1);
        Ok(x.parse::<u32>()?)
    }

    fn get_maximum(&self) -> Result<u32, Box<dyn Error>> {
        self.read_file("max_brightness")
    }

    fn get_brightness(&self) -> Result<Percent, Box<dyn Error>> {
        let max = self.get_maximum()?;
        let current = self.read_file("brightness")?;
        Ok((current * 100 / max) as Percent)
    }

    fn set_brightness(&self, percent: Percent) -> Result<(), Box<dyn Error>> {
        let max = self.get_maximum()?;
        let value = percent as u32 * max / 100;
        let path = format!("{}/brightness", self.path);
        let mut ofile = File::create(&path)?;
        ofile.write_all(format!("{}\n", value).as_bytes())?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    for b in list_backlights()?.iter() {
        //println!("{}", b.path);
        if args.len() > 1 {
            b.set_brightness(args[1].parse::<Percent>()?)?;
        }
        println!("{}", b.get_brightness()?);
    }
    Ok(())
}
