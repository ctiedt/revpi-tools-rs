use clap::Parser;
use picontrol::PiControl;

#[derive(Parser)]
enum Args {
    Read { var: String },
    Write { var: String, data: String },
}

fn main() {
    let args = Args::parse();
    let mut pc = PiControl::new().unwrap();
    match args {
        Args::Read { var } => {
            let var_data = pc.find_variable(&var);
            let res = pc.read(var_data.i16uAddress.into(), 1);
            println!("{}: {:?}", var, res);
        }
        Args::Write { var, data } => {
            let var_data = pc.find_variable(&var);
            pc.write(var_data.i16uAddress.into(), &[data.parse().unwrap()]);
        }
    }
}
