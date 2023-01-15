#[cfg(all(not(yaws_flavor = "io_uring"), not(yaws_flavor = "lunatic")))]
compile_error!("Set cfg(yaws_flavor) either as io_uring or lunatic");

#[cfg(yaws_flavor = "io_uring")]
fn main() {
    #[cfg(yaws_flavor = "io_uring")]
    yaws_run_uring::run();
}

#[cfg(yaws_flavor = "lunatic")]
#[lunatic::main]
fn main(mailbox: lunatic::Mailbox<()>) {
    yaws_run_lunatic::run();

    // This will block forever
    let _ = mailbox.receive();
}


