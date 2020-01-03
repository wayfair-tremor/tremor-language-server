// Copyright 2018-2020, Wayfair GmbH
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod backend;
mod language;

use backend::Backend;
use clap::{App, Arg};
use tower_lsp::{LspService, Server};

fn main() {
    backend::file_dbg("main", "main");

    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::with_name("language")
                .help("Tremor language to support")
                .short("l")
                .long("language")
                .takes_value(true)
                .possible_values(language::NAMES)
                .default_value(language::DEFAULT_NAME),
        )
        .get_matches();

    let language_name = matches
        .value_of("language")
        // this is safe because we provide a default value for this arg above
        .unwrap_or_else(|| unreachable!());

    match language::lookup(language_name) {
        Some(language) => {
            let stdin = tokio::io::stdin();
            let stdout = tokio::io::stdout();

            let (service, messages) = LspService::new(Backend::new(language));
            let handle = service.close_handle();
            let server = Server::new(stdin, stdout)
                .interleave(messages)
                .serve(service);

            tokio::run(handle.run_until_exit(server));
        }
        None => {
            eprintln!("Error: unknown tremor language {}", language_name);
            std::process::exit(1)
        }
    }
}
