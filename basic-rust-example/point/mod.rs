// mod.rs is the entry point for the 'test' module, 
// and it declares the submodules that are part of the 'test'
mod core;
pub use core::Point; // re-export Point so that it can be accessed as test::Point


// If you want to add a line "class", you could make a file called line.rs 
// and then add the line "mod line;" to this file, 
// then you can use it as test::line::Line.