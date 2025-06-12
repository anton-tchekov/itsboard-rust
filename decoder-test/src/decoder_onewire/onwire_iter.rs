struct OnewireIter<'a> {
    iter: EdgewiseIterator<'a>,
    last_idx: usize,
}

impl <'a>OnewireIter<'a> {
    fn next_bit() {
    }

    fn next_reset() {
    }

    fn next_response() {
    }

    fn next_empty() {
    }

    fn consume_last() {
    }

    fn current_time() {
    }
}

impl <'a>From<EdgewiseIterator<'a>> for OnewireIter<'a> {
    fn from(iter: EdgewiseIterator<'a>) -> Self {
        OnewireIter {
            iter,
            last_idx: iter.idx,
        }
    }
}