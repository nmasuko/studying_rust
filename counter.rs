fn counter() -> (Box<FnMut() -> i64>){
    let mut x : i64 = 0;
    let cls = move ||{
        x += 1;
        x
    };
    return Box::new(cls);
}

fn main(){
    let mut c = counter();
    println!("{}", c());
    println!("{}", c());
    let mut c2 = counter();
    println!("{}", c2());

}