fn main() {
    let s1:String = String::from("Hello world");
    let s2:&str = &s1;
    let s3 = s2.to_string();

    println!("{}",s3);

    let it = Iter {
        current: 0,
        max:10,
    };
    for num in it{
        println!("{}",num);
    }

    let a = Color{r:255,g:255,b:255};

    println!("{} {} {}",a.r,a.g,a.b);
}

struct Iter {
    current:usize,
    max:usize,
}

impl Iterator for Iter {
    type Item = usize;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.current += 2;
        if self.current -1 < self.max{
            Some(self.current)
        }else {
            None
        }
    }
}


struct Color{
    r:i32,
    g:i32,
    b:i32,
}
