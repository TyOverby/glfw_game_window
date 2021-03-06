#![crate_id = "glfw_game_window"]
#![deny(missing_doc)]

//! A GLFW window back-end for the Piston game engine.

extern crate piston;
extern crate glfw;
extern crate collections;
extern crate gl;

pub use game_window_glfw::GameWindowGLFW;

mod game_window_glfw;

