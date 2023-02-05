
            /// Returns the `rustc` SemVer version and additional metadata
            /// like the git short hash and build date.
            pub fn version_meta() -> VersionMeta {
                VersionMeta {
                    semver: Version {
                        major: 1,
                        minor: 66,
                        patch: 1,
                        pre: vec![],
                        build: vec![],
                    },
                    host: "x86_64-pc-windows-msvc".to_owned(),
                    short_version_string: "rustc 1.66.1 (90743e729 2023-01-10)".to_owned(),
                    commit_hash: Some("90743e7298aca107ddaa0c202a4d3604e29bfeb6".to_owned()),
                    commit_date: Some("2023-01-10".to_owned()),
                    build_date: None,
                    channel: Channel::Stable,
                }
            }
            