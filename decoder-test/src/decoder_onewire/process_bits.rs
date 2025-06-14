fn process_bits(
    iter: Iterator<Item = OneWirePrimitive>,
    output: &mut SectionBuffer,
    reader: BitReader,
    on_success: H,
    on_error: F,
) -> Option<G>
where
    F: FnOnce(DecoderOneWireError) -> Result<Option<T>, DecoderOneWireError>,
    G: FnOnce(u64) -> SectionContent,
    H: FnOnce(u64) -> Result<Option<T>, DecoderOneWireError>,
{
    let start = iter.current_time();
    let mut end = iter.current_time();
    let mut result = Ok(None);

    while let Some(primitive) = iter.peek() {
        result = match primitive {
            Ok(bit) => {
                let push = output.push(Section{
                    start: end,
                    end: iter.current_time(),
                    content: SectionContent::Bit(bit)
                });

                if push.is_none() {
                    Ok(None)
                }

                if reader.read_bit(bit) {
                    on_success()
                }
                Ok(None)
            },
            Err(err) => on_error(err),
        };

        match result {
            Ok(None) => {
                end = iter.current_time()
            },
            _ => break,
        }
    }

    if let Some(value) = readet.get_value() {
        output.push(Section {
            start: start,
            end: end,
            content: value_to_content(value)
        })
    }

    match result {
        Err(msg) => { 
            ouput.push(Section {
                start: end,
                end: iter.current_time(),
            });

            Some(DecoderOneWireState::Reset)
        }

        Ok(Some(state)) => state,
        Ok(None) => None
    }
}