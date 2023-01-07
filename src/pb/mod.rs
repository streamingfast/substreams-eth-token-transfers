#[path = "eth.token.transfers.v1.rs"]
#[allow(dead_code)]
mod transfers_priv;

pub mod transfers {
    use csv::Terminator;

    pub use super::transfers_priv::*;

    impl Transfer {
        pub fn to_csv(&self) -> String {
            let mut writer = csv::WriterBuilder::new()
                .has_headers(false)
                .terminator(Terminator::Any('\n' as u8))
                .from_writer(Vec::with_capacity(128));

            writer.serialize(self).unwrap_or_else(|err| {
                panic!(
                    "should have been able to serialize {:?} to CSV: {}",
                    self, err
                )
            });

            // Takes the bytes written and remove the terminator which is always a new line
            let bytes = writer.into_inner().unwrap();
            if let Some((_terminator, line)) = bytes.split_last() {
                return std::str::from_utf8(line).unwrap().to_string();
            }

            // There was no bytes written, so let's return the empty string
            "".to_string()
        }
    }
}

#[path = "substreams.sink.files.v1.rs"]
#[allow(dead_code)]
pub mod sinkfiles;
