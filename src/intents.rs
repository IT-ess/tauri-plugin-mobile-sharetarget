use crossbeam_queue::SegQueue;

static INTENTS: SegQueue<String> = SegQueue::new();

pub(crate) fn push_new_intent(raw_intent: String) {
    INTENTS.push(raw_intent);
}

pub(crate) fn pop_intent() -> Option<String> {
    INTENTS.pop()
}
