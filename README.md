minimal example to reproduce a compile issue i'm having
I'm trying to build a project in rust, compiled for the web with wasm32, emscripten  , enabling pthread support
Every time I try this, the linker is complaining:

"wasm-ld: error: --shared-memory is disallowed by std-e42ff3047517e183.std.70dd5b92-cgu.0.rcgu.o because it was not compiled with 'atomics' or 'bulk-memory' features."

I think some of the stdlibs for this target would have to be compiled differently to enable this support? 
wasm32-unknown-emscripten works fine for me without pthreads; see .cargo/config.toml, comment out this line and it should work:

	"-C", "link-arg=-s", "-C", "link-args=USE_PTHREADS", # IT JUST WONT COMILE WITH THIS :(


