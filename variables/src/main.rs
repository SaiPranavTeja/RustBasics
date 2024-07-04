fn main(){
  //Variable Definition Section
  {
    let x =10;//imutable variable declaration
    println!("value of x is {}",x);
    /*
        x=20;
        println!("value of x is {}",x);
      /*
        Error: cannot assign twice to immutable variable
        Rust Variables ate immutable by default
      */
    */
    let mut y=30;
    println!("value of y is {}",y);
    y=20;
    println!("value of y is {}",y);
  }

   /*
    Integer variables
    integer types available are
    8-bit	i8	u8
    16-bit	i16	u16
    32-bit	i32	u32
    64-bit	i64	u64
    128-bit	i128	u128
    arch	isize	usize
  */
  {
    let _a: u8 =30; 
    let _z=30; //default chooses i32
    /*
    let b: u8 =256; //error: literal out of range for `u8`
    let mut c: u8=255;
    c=c+1; //error: this arithmetic operation will overflow
    */
  
  }
  // floating point (f32,f64)
  {
     let _fa:f32 =10.0000000;
     let _fz=30.0; //default chooses f64
  }
  // Arthermatic operation +,-, *,/,%
  {
    let mut v1:f32=10.0;
    let mut v2:f32=30.0;
    let  add =v1+v2;
    println!("Sum is {}",add);
    v1=60.0;
    v2=2.0;
    let  mul=v1*v2;
    println!("product is {}",mul);
    let div= mul/add;
    println!("Div is {}",div);
    let mod1 = mul%7.0;
    println!("mod is {}",mod1);
  }
}