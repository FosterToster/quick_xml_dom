use std::str::FromStr;

use quick_xml_dom::{Element};

fn main(){
    let mut a = Element::from_str("<Data count=\"3\"><Ivan lastname=\"Petrov\">Hello, DOM navigation</Ivan></Data>").unwrap();

    println!("{}: count = {}", a.tag(), a.attributes().get("count").unwrap().value);
    let volodya = a.pop_child().unwrap();
    println!("{} {} says \"{}\"", volodya.tag(), volodya.attributes().get("lastname").unwrap().value, volodya.text().unwrap());

}