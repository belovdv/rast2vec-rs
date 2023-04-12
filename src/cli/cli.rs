use rusteval::{Interactive, InteractiveRoot};

use super::{Config, Manager, Status};

#[derive(Debug, derive_new::new)]
pub struct CLI {
    config: Config,
}

impl CLI {
    pub fn start(&self) {
        let mut manager = Manager::new(self.config.clone());
        dbg!(manager.get_all_field_names());

        for line in std::io::stdin().lines() {
            let line = line.expect("failed to read line");
            let eval = manager.eval_to_string(&line);
            println!("> {eval}");

            match manager.status() {
                Status::Normal => {}
                Status::Exit => {
                    eprintln!("exit");
                    return;
                }
            }
        }
        eprintln!("EOS");
    }
}


