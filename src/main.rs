
fn main(){
    let string1 = String::from("Jelly");
    let string2= String::from("bean!");

   let concatenated_string = concatenate_strings(&string1, &string2);

   println!("{}", concatenated_string);
}

// conacatenate 2 string slices
fn concatenate_strings(s:&str, t:&str)-> String{
    let mut result= String::from(s);
    result.push_str(t);
    result
}