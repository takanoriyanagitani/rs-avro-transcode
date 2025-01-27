use std::io;

use std::io::BufReader;
use std::io::Read;

use std::io::BufWriter;
use std::io::Write;

use apache_avro::types::Value;
use apache_avro::Writer;

use apache_avro::Codec;
use apache_avro::Reader;
use apache_avro::Schema;

pub fn val2wtr<I, W>(values: I, mut wtr: Writer<W>) -> Result<(), io::Error>
where
    I: Iterator<Item = Result<Value, io::Error>>,
    W: Write,
{
    for rval in values {
        let val: Value = rval?;
        wtr.append(val).map_err(io::Error::other)?;
    }
    wtr.flush().map_err(io::Error::other)?;
    Ok(())
}

pub fn rdr2wtr<R, W>(r: R, c: Codec, mut w: W) -> Result<(), io::Error>
where
    R: Read,
    W: Write,
{
    let reader: Reader<R> = Reader::new(r).map_err(io::Error::other)?;
    let s: Schema = reader.writer_schema().clone();
    let wtr: Writer<_> = Writer::with_codec(&s, &mut w, c);
    let values = reader.map(|rslt| rslt.map_err(io::Error::other));
    val2wtr(values, wtr)?;
    w.flush()?;
    Ok(())
}

pub fn stdin2stdout(codec_string: &str) -> Result<(), io::Error> {
    let c: Codec = str::parse(codec_string)
        .map_err(|_| io::Error::other(format!("wrong codec: {codec_string}")))?;

    let i = io::stdin();
    let il = i.lock();
    let br = BufReader::new(il);

    let o = io::stdout();
    let mut ol = o.lock();
    let bw = BufWriter::new(&mut ol);
    rdr2wtr(br, c, bw)?;
    ol.flush()
}
