pub mod input {
    use std::io;
    pub fn int_i8() -> i8 {
        let mut x = String::new();
        io::stdin().read_line(&mut x);
        let x: i8 = x.trim().parse().unwrap();
        x
    }
    pub fn int_i32() -> i32 {
        let mut x = String::new();
        io::stdin().read_line(&mut x);
        let x: i32 = x.trim().parse().unwrap();
        x
    }
        pub fn int_i64() -> i64 {
        let mut x = String::new();
        io::stdin().read_line(&mut x);
        let x: i64 = x.trim().parse().unwrap();
        x
    }    
        pub fn int_f32() -> f32 {
        let mut x = String::new();
        io::stdin().read_line(&mut x);
        let x: f32 = x.trim().parse().unwrap();
        x
    }
        pub fn int_f64() -> f64 {
        let mut x = String::new();
        io::stdin().read_line(&mut x);
        let x: f64 = x.trim().parse().unwrap();
        x  
    }
        pub fn string() -> String {
        let mut x = String::new();
        io::stdin().read_line(&mut x);
        let x: String = x.trim().parse().unwrap();
        x  
    }
        pub fn int_usize() -> usize {
        let mut x = String::new();
        io::stdin().read_line(&mut x);
        let x: usize = x.trim().parse().unwrap();
        x  
    }    
}
