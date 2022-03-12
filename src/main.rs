// level 0: generate this file
// use "cargo new <project-name>"
// note: apparently camel-case are
// 		 despised in this language ?

// level 1: hello world
fn hello(){
	println!("level1:");
	println!("Hello, world!\n");
}

// level 2: import
mod neighbor;

// level 3: import function
mod neighbor2;
use neighbor2::call_me_instead;

// level 4: import module
mod module;
use module::sub_module::matfhs as matfhs;

// level 5: crate ? (from what i've read its a module concept ?)
// nvm apparently crate is optional ? idk im new to this
mod lib;
use lib::test as test;

// level 6: variable (ALL... LIKE ALL THE VAR)
fn all_var(){
	let decimal = 2;
	let pi = 3.14;
	let name = "jaka";
	let arr_dec = [1,2,3];
	let arr_str = ["1","abc","jaka"];
	// let arr_com = [1,"abc","jaka"]; // cant combine int and string
	// let empty_arr = []; // no clue how to make this ???
	println!("ex. dec={}",decimal);
	println!("ex. pi={}",pi);
	println!("ex. name={}",name);
	println!("ex. arr={}", arr_dec[0]); //sadly cant print the whole array (need loop to do so)
	println!("ex. arr={}", arr_str[1]); //sadly cant print the whole array (need loop to do so) note: thank god you can print string like so
	println!("");
}

// level 7: use multiple module
mod multi_lib; // didnt add the inside of the lib but i created an example that doesnt return error

// execute levels
fn main() {
	hello();
	neighbor::well_hello_there();
	call_me_instead();
	matfhs::public();
	test();
	all_var();
}
