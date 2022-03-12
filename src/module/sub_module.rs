pub mod matfhs {
	pub fn public(){
		println!("level4: ");
		println!("this is public function");
		private();
	}
	fn private(){
		println!("2+2=5... HEY this is private !\n");
	}
}