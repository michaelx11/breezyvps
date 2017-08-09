#[macro_use]
extern crate log;

pub mod command;
pub mod digitalocean;
pub mod configure;
pub mod chain;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
