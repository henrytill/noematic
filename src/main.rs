use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use tempfile::NamedTempFile;

fn read_length(reader: &mut impl Read) -> io::Result<Option<u32>> {
    let mut bytes = [0; 4];
    match reader.read_exact(&mut bytes) {
        Ok(_) => Ok(Some(u32::from_ne_bytes(bytes))),
        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => Ok(None),
        Err(e) => Err(e),
    }
}

fn read_message(reader: &mut impl Read, length: u32) -> io::Result<Vec<u8>> {
    let length = length as usize;
    let mut message = vec![0; length];
    reader.read_exact(&mut message)?;
    Ok(message)
}

fn handle_message(writer: &mut impl Write, message: &[u8]) -> io::Result<()> {
    let length = (message.len() as u32).to_ne_bytes();
    writer.write_all(&length)?;
    writer.write_all(message)?;
    writer.flush()?;
    Ok(())
}

fn main() -> io::Result<()> {
    let temp_file = NamedTempFile::new()?;
    let mut reader = BufReader::new(io::stdin());
    let mut writer = BufWriter::new(File::create(temp_file.path())?);

    loop {
        let length = match read_length(&mut reader)? {
            Some(len) => len,
            None => break, // End of input
        };

        let message = read_message(&mut reader, length)?;
        handle_message(&mut writer, &message)?;
        handle_message(&mut io::stdout(), &message)?;
    }

    Ok(())
}
