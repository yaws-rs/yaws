#[cfg(all(not(yaws_flavor = "io_uring"), not(yaws_flavor = "lunatic")))]
compile_error!("Set cfg(yaws_flavor) either as io_uring or lunatic");

fn main() {
    yaws_run::run();
}
