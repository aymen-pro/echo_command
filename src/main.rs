use std::env;

fn main() {
    let cli_args = get_cli_args();
}

fn get_cli_args() -> Vec<String> {
    let mut cli_args: Vec<_> = env::args_os().collect();
    let cli_args = cli_args.split_off(1);

    cli_args
        .iter()
        .map(|arg| {
            arg.to_str()
                .expect("arguments should contain valid unicode values")
                .to_string()
        })
        .collect()
}
