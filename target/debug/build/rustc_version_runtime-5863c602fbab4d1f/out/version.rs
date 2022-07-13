
            /// Returns the `rustc` SemVer version and additional metadata
            /// like the git short hash and build date.
            pub fn version_meta() -> VersionMeta {
                VersionMeta {
                    semver: Version {
                        major: 1,
                        minor: 61,
                        patch: 0,
                        pre: vec![],
                        build: vec![],
                    },
                    host: "x86_64-apple-darwin".to_owned(),
                    short_version_string: "rustc 1.61.0 (fe5b13d68 2022-05-18)".to_owned(),
                    commit_hash: Some("fe5b13d681f25ee6474be29d748c65adcd91f69e".to_owned()),
                    commit_date: Some("2022-05-18".to_owned()),
                    build_date: None,
                    channel: Channel::Stable,
                }
            }
            