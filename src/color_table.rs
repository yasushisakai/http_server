use image::Rgb;

pub struct ColorTable {
    colors: Vec<Color>
}

impl ColorTable {

    pub fn new() -> ColorTable {
        ColorTable{colors:Vec::new()}
    }

    pub fn from_file_header(bytes: Vec<u8>) -> ColorTable {
        let mut ct = ColorTable::new();

        for i in 0..bytes.len()/3 {
            let index = i * 3;
            let c = Color::new(bytes[index],bytes[index + 1],bytes[index + 2]);
            ct.push(&c);
        }
        ct
    }

    pub fn has(&self, new_color:&Color) -> Option<u8> {
        let mut cnt = 0;    
        for c in &self.colors {
            if new_color == c {
                return Some(cnt)
            }
            cnt += 1;
        }

        None
    }

    pub fn get(&self, nth:u8) -> Color {
        self.colors[nth as usize]
    }

    pub fn push(&mut self, new_color:Color)  {
        self.colors.push(new_color);
    }

    pub fn push_if_unique(&mut self, new_color:&Color) -> u8 {
        let is_unique = self.has(&new_color);

        match is_unique {
            Some(i) => i,
            None => {
                let len = self.colors.len();
                self.push(new_color);
                len as u8
            }
        }
    }

    pub fn output_file_header(&self) -> Vec<u8> {
        let mut color_vec:Vec<u8> = Vec::new();

        let colors = self.colors.clone();

        for c in colors {
            color_vec.extend_from_slice(&c.to_u8().as_ref());
        }
        color_vec
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Color (u8, u8, u8);

impl Color {
    pub fn new (r:u8, g:u8, b:u8) -> Color {
        Color(r, g, b);
    }

    pub fn to_u8(&self) -> [u8;3] {
        [self.0, self.1, self.2]
    }

    pub fn to_rgb(&self) -> Rgb {
        let byte_arry = self.to_u8();
        Rgb(byte_arry)
    }
}

#[derive(Debug, Deserialize)]
struct Data { 
    index: u8;
    value: u8;
}

impl Data {
    pub fn new (i: u8, v:u8) -> Data {
        Data{index:i, value:v}
    }
}