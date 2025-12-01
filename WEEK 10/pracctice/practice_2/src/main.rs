fn main() {
  

  let v = vec![10, 20, 39];

  let v2 = v.clone();

  display(v2.clone());

  println!("In main {:?}", v2);
}

fn display(v:Vec<i32>){
    println!("inside display {:?}", v);
}
