use crate::decoder::{Section, SectionBuffer, SectionContent};
use crate::decoder_onewire::onewire_error::OneWireError;
use crate::decoder_onewire::onwire_iter::OnewireIter;
use crate::decoder_onewire::timings::Timings;

pub struct OneWireOutput<'a> {
    output: &'a mut SectionBuffer
}

impl <'a>OneWireOutput<'a> {
    pub fn push(&self, section: Section) -> Option<()> {
        self.output.push(section).ok()
    }

    pub fn push_err(&self, iter: &mut OnewireIter, start_time: u32, err: OneWireError) -> Option<()> {
        iter.set_timing(Timings::standard());
        iter.forward_to_reset()?;

        self.push(Section { 
            start: start_time,
            end: iter.current_time(),
            content: SectionContent::Err(err.to_string())
        })
    }
}