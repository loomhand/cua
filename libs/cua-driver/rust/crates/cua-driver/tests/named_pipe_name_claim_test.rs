#![cfg(target_os = "windows")]

//! A Windows daemon must not join a pipe name another server already holds:
//! the two servers would split the clients between them. The holder here
//! keeps its only instance connected, so the daemon's `WaitNamedPipeW`
//! already-running probe finds no listening instance and the daemon reaches
//! its own first `CreateNamedPipeW`.

use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use cua_driver_testkit::IsolatedStateRoot;
use tokio::net::windows::named_pipe::{ClientOptions, ServerOptions};

const EXIT_WINDOW: Duration = Duration::from_secs(30);

#[tokio::test]
async fn serve_refuses_a_pipe_name_another_server_holds() {
    let socket = format!(r"\\.\pipe\cua-driver-test-claim-{}", std::process::id());
    let holder = ServerOptions::new()
        .first_pipe_instance(true)
        .create(&socket)
        .expect("create the holder's pipe instance");
    let _client = ClientOptions::new()
        .open(&socket)
        .expect("connect to the holder");
    holder.connect().await.expect("holder accepts the client");

    let state = IsolatedStateRoot::new().expect("isolated driver state root");
    let mut child = Command::new(env!("CARGO_BIN_EXE_cua-driver"))
        .args([
            "serve",
            "--socket",
            &socket,
            "--no-permissions-gate",
            "--no-overlay",
        ])
        .envs(state.env())
        .env("CUA_DRIVER_RS_TELEMETRY_ENABLED", "false")
        .env("CUA_DRIVER_RS_UPDATE_CHECK", "false")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn cua-driver serve");

    let deadline = Instant::now() + EXIT_WINDOW;
    let status = loop {
        if let Some(status) = child.try_wait().expect("poll the daemon") {
            break status;
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            panic!("the daemon joined {socket}, which another server holds");
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    };
    let stderr = std::io::read_to_string(child.stderr.take().expect("daemon stderr"))
        .expect("read the daemon's stderr");
    assert!(!status.success(), "the daemon exited cleanly: {stderr}");
    assert!(
        stderr.contains(&format!("create named pipe {socket}")),
        "the daemon failed for another reason: {stderr}"
    );
}
