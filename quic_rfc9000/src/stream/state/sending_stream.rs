pub enum SendingStreamStates {
    Ready,
    Send,
    DataSent,
    DataRecieved,
    ResetSend,
    ResetRecieved,
}

pub enum SendingStreamEvents {
    CreateStream,
    PeerCreatesBidirectionalStream,
    ResetStream,
    StreamDataBlocked,
    StreamFinished,
    RecievedAllAcks,
    RecievedResetAck,
}
