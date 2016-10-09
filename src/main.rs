#![feature(lang_items)]

// We won't use the usual `main` function. We are going to use a different "entry point".
#![no_main]

// We won't use the standard library because it requires OS abstractions like threads and files and
// those are not available in this platform.
#![no_std]

#![feature(asm)]

// Conceptually, this is our program "entry point". It's the first thing the microcontroller will
// execute when it (re)boots. (As far as the linker is concerned the entry point must be named
// `start` (by default; it can have a different name). That's why this function is `pub`lic, named
// `start` and is marked as `#[no_mangle]`.)
//
// Returning from this function is undefined because there is nothing to return to! To statically
// forbid returning from this function, we mark it as divergent, hence the `fn() -> !` signature.
#[no_mangle]
pub fn start() -> ! {
    // Our first program initializes some variables on the stack and does nothing more. Yay!
    let x = 42;
    let y = x;

//    unsafe {
//        let ram_boundary = *(0x0000_0000 as *const u32);
//        let _crash = *(ram_boundary as *const u32);
//    }

    // We can't return from this function so we just spin endlessly here.
    loop {}
}

// Ignore this part for now :-). It will covered in a later section.
mod vector_table {
    #[link_section = ".reset"]
    static RESET: fn() -> ! = ::start;

    #[link_section = ".exceptions"]
    static EXCEPTIONS: [Option<fn() -> !>; 14] = [Some(::exception::handler),  // NMI
                                                  Some(::exception::handler),  // Hard fault
                                                  Some(::exception::handler),  // Memory management fault
                                                  Some(::exception::handler),  // Bus fault
                                                  Some(::exception::handler),  // Usage fault
                                                  None, // Reserved
                                                  None, // Reserved
                                                  None, // Reserved
                                                  None, // Reserved
                                                  Some(::exception::handler),  // SVCall
                                                  None, // Reserved for Debug
                                                  None, // Reserved
                                                  Some(::exception::handler),  // PendSV
                                                  Some(::exception::handler)]; // Systick
}

mod exception {
    pub fn handler() -> ! {
        unsafe {
            asm!("bkpt");
        }

        loop {}
    }
}


// Finally, we need to define the panic_fmt and eh_personality "lang item"s, which are just
// functions. This specifies what the program should do when a `panic!` occurs. Our program won't
// panic, so we can leave the function bodies empty for now.
mod lang_items {
    #[lang = "panic_fmt"]
    extern fn panic_fmt() {}
}
