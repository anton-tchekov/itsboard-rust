use crate::decoder::{Section, SectionBuffer, SectionContent};
use crate::decoder_onewire::onewire_error::OneWireError;
use crate::decoder_onewire::onwire_iter::OnewireIter;
use crate::decoder_onewire::timings::Timings;

pub struct OneWireOutput<'a> {
    output: &'a mut SectionBuffer
}

impl <'a>OneWireOutput<'a> {
    pub fn push(&mut self, section: Section) -> Option<()> {
        self.output.push(section).ok()
    }

    pub fn push_err(&mut self, iter: &mut OnewireIter, start_time: u32, err: OneWireError) -> Option<()> {
        iter.set_timing(Timings::standard());
        iter.forward_to_reset()?;

        self.push(Section { 
            start: start_time,
            end: iter.current_time(),
            content: SectionContent::Err(err.to_string())
        })
    }
}

impl <'a>From<&'a mut SectionBuffer> for OneWireOutput<'a> {
    fn from(value: &'a mut SectionBuffer) -> Self {
        Self { output: value }
    }
}