#[macro_use]
extern crate nom;
//use nom::IResult;

named!(get_greeting<&str,&str>,
   tag_s!("hi")
);

//named!(get_greeting<&str,&str>,
//    ws!(tag_s!("hi"))
//);

// named!(get_greeting<&str, &str>,
//     ws!(alt!(tag_s!("hi") | tag_s!("bye")))
// );

// named!(full_greeting<&str,(&str,&str)>,
//     pair!(
//         get_greeting,
//         nom::alpha
//     )
// );

// named!(full_greeting<&str,(&str,&str)>,
//     pair!(
//         get_greeting,
//         nom::alpha
//     )
// );

fn main() {
    let res = get_greeting("hi there");
    println!("{:?}", res);
    //println!("result {:?}", full_greeting(" hi Bob  "));
    //println!("result {:?}", full_greeting(" bye ?"));
    // result Ok(("hi", Some("Bob")))
    // result Ok(("bye", None))
}
// Done(" there", "hi")