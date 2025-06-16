use crate::bit_reader::{BitOrder, BitReader};
use crate::decoder::{Section, SectionContent};
use crate::decoder_onewire::onewire_error::OneWireError;
use crate::decoder_onewire::onewire_output::OneWireOutput;
use crate::decoder_onewire::OneWireState;
use crate::decoder_onewire::onwire_iter::OnewireIter;

fn process_bits<F, G>(
    iter: &mut OnewireIter,
    output: &mut OneWireOutput,
    amount: u8,
    success_state: OneWireState,
    on_error: F,
    value_to_content: G,
) -> Option<OneWireState>
where
    F: FnOnce(OneWireError) -> Result<Option<OneWireState>, OneWireError>,
    G: FnOnce(u64) -> Result<OneWireState, OneWireError>,
{
    let reader = BitReader::new(amount, BitOrder::LSB);

    let start = iter.current_time();
    let mut end = iter.current_time();
    let mut result = Ok(None);

    while let Some(primitive) = iter.next_bit() {
        result = match primitive.1 {
            Ok(bit) => {
                let push = output.push(Section { 
                    start: end,
                    end: primitive.0,
                    content: SectionContent::Bit(bit)
                });
                if push.is_none() { break;}
                if reader.read_bit(bit) { success_state }

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

    if let Some(value) = reader.get_value() {
        let content = value_to_content(value);
        result = push_partial(output, content, start, end, result)?;
    };

    match result {
        Err(err) => { 
            output.push_err(iter, end, err)?;
            Some(OneWireState::Reset)
        }
        Ok(value) => value
    }
}

fn push_partial(
    output: &mut OneWireOutput, 
    content: Result<SectionContent, OneWireError>,
    start: u32,
    end: u32, 
    default: Result<Option<OneWireState>, OneWireError>
) -> Option<Result<Option<OneWireState>, OneWireError>> 
{
    let section_content = match (content, default) {
        (Err(code), Err(_)) => SectionContent::Err(code.to_string()),
        (Err(code), Ok(_)) => return Some(Err(code)),
        Ok(content) => content,
    };

    output.push(Section {start, end, content})?;
    return Some(default);
}