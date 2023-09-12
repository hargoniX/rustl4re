use l4::sys::{l4_utcb, l4_msgtag};
use std::{thread, time};

pub fn main() {
    // retrieve IPC gate from Ned (created in the *.cfg script file
    let server = l4re::sys::l4re_env_get_cap("channel").unwrap();
    // test whether valid cap received
    if l4::sys::l4_is_invalid_cap(server) {
        panic!("No IPC Gate found.");
    }

    let mut counter = 1;
    loop {
        // dump value in UTCB
        unsafe {
            (*l4::sys::l4_utcb_mr()).mr[0] = counter;
        }
        println!("value written to register");
        // the message tag contains instructions to the kernel and to the other party what is being
        // transfered: no protocol, 1 message register, no flex page and no flags; flex pages are
        // not relevant here
        unsafe {
            let send_tag = l4_msgtag(0, 1, 0, 0);
            let tag = l4::sys::l4_ipc_call(server, l4_utcb(), send_tag,
                    l4::sys::l4_timeout_t { raw: 0 });
            println!("data sent");
            // check for IPC error, if yes, print out the IPC error code, if not, print the received
            // result.
            match l4::sys::l4_ipc_error(tag, l4_utcb()) {
                0 => // success
                    println!("Received: {}\n", (*l4::sys::l4_utcb_mr()).mr[0]),
                ipc_error => println!("client: IPC error: {}\n",  ipc_error),
            };
        }

        thread::sleep(time::Duration::from_millis(1000));
        counter += 1;
    }
}


