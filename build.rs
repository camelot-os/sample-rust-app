// SPDX-License-Identifier: Apache-2.0
//
// SPDX-FileCopyrightText: 2024 Ledger SAS

use std::env;

fn main() {
    let metadata =
        camelot_metadata::cargo_package_introspect(env::var("CARGO_MANIFEST_PATH").ok().as_deref());
    let _ = camelot_metadata::gen_package_metadata(
        env::var("CARGO_PKG_NAME").unwrap().as_str(),
        metadata,
        env::var("config").ok().as_deref(),
        None,
    );
}
