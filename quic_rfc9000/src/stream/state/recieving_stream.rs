#[derive(Debug)]
pub struct Recieving;
#[derive(Debug)]
pub struct SizeKnown;
#[derive(Debug)]
pub struct DataRecieved;
#[derive(Debug)]
pub struct DataRead;
#[derive(Debug)]
pub struct ResetRecieved;
#[derive(Debug)]
pub struct ResetRead;

#[derive(Debug)]
pub struct RecievingState<S> {
    state: S,
}

#[derive(Debug)]
pub enum RecievingStateMachine {
    Recieving(RecievingState<Recieving>),
    SizeKnown(RecievingState<SizeKnown>),
    DataRecieved(RecievingState<DataRecieved>),
    DataRead(RecievingState<DataRead>),
    ResetRecieved(RecievingState<ResetRecieved>),
    ResetRead(RecievingState<ResetRead>),
    Corrupted(String),
}

impl RecievingStateMachine {
    pub fn new(event: RecievingStateMachine) -> Self {
        match event {
            _ => {
                tracing::error!("ILLEGAL: envent cannot start machine: {:#?}", event)
            },
        }
    }

    pub fn next(self: Self, event: RecievingStateMachine) -> Self {
        match (self, event) {
            (s, e) => {
                tracing::error!("ILLEGAL: state and event combination: {:#?} {:#?}", s, e);
                RecievingStateMachine::Corrupted("ERROR: unrecoverable".into())
            }
        }
    }
}

pub enum RecievingStreamEvents {
    StreamDataBlocked,
    ResetStream,
    CreateBidirectionalStream,
    MaxStreamData,
    StopSending,
    HigherNumberStreamCreated,
}
