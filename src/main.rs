use actix::prelude::*;

fn main() {
  let system = System::new();

  let execution = async {

  };

  let arbiter = Arbiter::new();
  arbiter.spawn(execution);
  system.run().expect("Could not run the actix-rt system");
}
