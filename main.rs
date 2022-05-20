fn main() {
	println!("hello from rust");
}


type _Unwind_Reason_Code = i32;
type _Unwind_Action=usize;
struct _Unwind_Context();
struct _Unwind_Exception();

#[cfg(target_os="emscripten")]
#[no_mangle]
extern "C" fn __gxx_personality_v0   (version:isize, actions:_Unwind_Action, exceptionClass:u64,unwind_exception:*const _Unwind_Exception,context:*mut _Unwind_Context)->_Unwind_Reaso
n_Code
{
        return  0;
}
