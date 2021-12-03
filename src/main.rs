use std::os::raw::c_int;

extern "C"  {
    fn hello_world();
}

fn main(){
    unsafe{
        hello_world();
    }
}

