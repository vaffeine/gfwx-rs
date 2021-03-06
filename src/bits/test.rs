use super::*;

#[test]
fn test_bits_write() {
    let expected: Vec<u8> = vec![
        244, 23, 0, 0, 2, 0, 126, 0, 58, 0, 0, 160, 0, 160, 4, 0, 0, 230, 80, 13, 0, 32, 121, 14,
    ];
    let mut output = vec![];
    {
        let mut stream = BitsIOWriter::new(&mut output);
        stream.put_bits(185, 27).unwrap();
        stream.put_bits(61, 3).unwrap();
        stream.put_bits(63, 17).unwrap();
        stream.put_bits(42, 21).unwrap();
        stream.put_bits(29, 27).unwrap();
        stream.put_bits(37, 20).unwrap();
        stream.put_bits(213, 25).unwrap();
        stream.put_bits(230, 12).unwrap();
        stream.put_bits(115, 19).unwrap();
        stream.put_bits(201, 8).unwrap();
        stream.flush_write_word().unwrap();
    }
    assert_eq!(output, expected);
}

#[test]
fn test_bits_write_overflow_detection() {
    let mut output = [0u8; 7];
    let mut slice: &mut [u8] = &mut output;

    let mut stream = BitsIOWriter::new(&mut slice);
    stream.put_bits(185, 27).unwrap();
    stream.put_bits(61, 3).unwrap();
    stream.flush_write_word().unwrap();
    stream.put_bits(3, 6).unwrap();
    assert!(stream.flush_write_word().is_err());
}

#[test]
fn test_bits_read() {
    let input: [u8; 20] = [
        0, 0, 223, 1, 0, 37, 0, 93, 162, 128, 29, 0, 0, 122, 208, 1, 0, 0, 0, 39,
    ];
    let mut output: Vec<u32> = vec![];
    let expected: Vec<u32> = vec![239, 2, 186, 148, 59, 162, 0, 29, 244, 156];
    {
        let mut slice: &[u8] = &input;
        let mut stream = BitsIOReader::new(&mut slice);
        output.push(stream.get_bits(15).unwrap());
        output.push(stream.get_bits(2).unwrap());
        output.push(stream.get_bits(24).unwrap());
        output.push(stream.get_bits(17).unwrap());
        output.push(stream.get_bits(23).unwrap());
        output.push(stream.get_bits(15).unwrap());
        output.push(stream.get_bits(1).unwrap());
        output.push(stream.get_bits(11).unwrap());
        output.push(stream.get_bits(13).unwrap());
        output.push(stream.get_bits(17).unwrap());
    }
    assert_eq!(output, expected);
}

#[test]
fn test_bits_read_underflow_detection() {
    let output = [1, 2, 3, 4, 5, 6u8];
    let mut slice: &[u8] = &output;

    let mut stream = BitsIOReader::new(&mut slice);
    stream.get_bits(27).unwrap();
    stream.get_bits(3).unwrap();
    stream.flush_read_word();
    assert!(stream.get_bits(4).is_err());
}

#[test]
fn test_zeros() {
    // In Little endian: 0xb10001000, 0b00101100, 0b10000001, 0b11110001_u8
    let input = [0b11110001_u8, 0b10000001, 0b00101100, 0b10001000];
    let mut slice: &[u8] = &input;
    let mut stream = BitsIOReader::new(&mut slice);
    assert_eq!(stream.get_zeros(11).unwrap(), 0);
    assert_eq!(stream.get_zeros(1).unwrap(), 1);
    assert_eq!(stream.get_zeros(5).unwrap(), 2);
    assert_eq!(stream.get_zeros(6).unwrap(), 5);
    assert_eq!(stream.get_zeros(2).unwrap(), 1);
    assert_eq!(stream.get_zeros(2).unwrap(), 0);
    assert_eq!(stream.get_zeros(3).unwrap(), 2);
    assert_eq!(stream.get_zeros(10).unwrap(), 6);
    assert_eq!(stream.get_zeros(2).unwrap(), 0);
    assert_eq!(stream.get_zeros(2).unwrap(), 0);
    assert_eq!(stream.get_zeros(2).unwrap(), 0);
    assert_eq!(stream.get_zeros(2).unwrap(), 0);
    assert_eq!(stream.get_zeros(5).unwrap(), 3);
}
