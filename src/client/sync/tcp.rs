// SPDX-FileCopyrightText: Copyright (c) 2017-2025 slowtec GmbH <post@slowtec.de>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! TCP client connections

use std::{io, net::SocketAddr, time::Duration};

use crate::{client::tcp::connect_slave as async_connect_slave, Slave};

use super::{block_on_with_timeout, Context};

/// Establish a direct connection to a _Modbus_ TCP coupler.
pub fn connect(socket_addr: SocketAddr) -> io::Result<Context> {
    connect_slave(socket_addr, Slave::tcp_device())
}

/// Establish a direct connection to a _Modbus_ TCP coupler with a timeout.
pub fn connect_with_timeout(
    socket_addr: SocketAddr,
    timeout: Option<Duration>,
) -> io::Result<Context> {
    connect_slave_with_timeout(socket_addr, Slave::tcp_device(), timeout)
}

/// Connect to any kind of _Modbus_ slave device, probably through a _Modbus_ TCP/RTU
/// gateway that is forwarding messages to/from the corresponding unit identified
/// by the slave parameter.
pub fn connect_slave(socket_addr: SocketAddr, slave: Slave) -> io::Result<Context> {
    connect_slave_with_timeout(socket_addr, slave, None)
}

/// Connect to any kind of _Modbus_ slave device, probably through a _Modbus_ TCP/RTU
/// gateway that is forwarding messages to/from the corresponding unit identified
/// by the slave parameter.
pub fn connect_slave_with_timeout(
    socket_addr: SocketAddr,
    slave: Slave,
    timeout: Option<Duration>,
) -> io::Result<Context> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()?;
    let async_ctx =
        block_on_with_timeout(&runtime, timeout, async_connect_slave(socket_addr, slave))?;
    let sync_ctx = Context {
        runtime,
        async_ctx,
        timeout,
    };
    Ok(sync_ctx)
}
