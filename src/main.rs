use std::{fmt, result};

use slog::*;


pub struct PrintlnSerializer;

impl Serializer for PrintlnSerializer {
    fn emit_arguments(&mut self, key: Key, val: &fmt::Arguments<'_>) -> Result {
        print!(", {}={}", key, val);
        Ok(())
    }
}

pub struct PrintlnDrain;

impl Drain for PrintlnDrain {
    type Ok = ();
    type Err = ();

    fn log(
        &self,
        record: &Record<'_>,
        values: &OwnedKVList,
    ) -> result::Result<Self::Ok, Self::Err> {
        print!("{}", record.msg());

        record
            .kv()
            .serialize(record, &mut PrintlnSerializer)
            .unwrap();
        values.serialize(record, &mut PrintlnSerializer).unwrap();

        println!();
        Ok(())
    }
}

fn main() {
    let log = Logger::root(Fuse(PrintlnDrain), o!());

    let output = vec![1, 2, 3];
    //trace_macros!(true);
    info!(log, "test"; "output" => slog::Serde(output));
}
