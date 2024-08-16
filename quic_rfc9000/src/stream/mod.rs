mod state;

#[derive(Debug, PartialEq, Eq)]
pub enum StreamType {
    ClientInitiatetBidirectional,
    ServerInitiatetBidirectional,
    ClientInitiatetUnidirectional,
    ServerInitiatetUnidirectional
}

impl TryFrom<u8> for StreamType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(StreamType::ClientInitiatetBidirectional),
            0x01 => Ok(StreamType::ServerInitiatetBidirectional),
            0x02 => Ok(StreamType::ClientInitiatetUnidirectional),
            0x03 => Ok(StreamType::ServerInitiatetUnidirectional),
            _ => Err("ERR: invalid stream type")
        }
    }
} 

impl From<StreamType> for i64 {
    fn from(val: StreamType) -> Self {
        match val {
            StreamType::ClientInitiatetBidirectional => 0x00,
            StreamType::ServerInitiatetBidirectional => 0x01,
            StreamType::ClientInitiatetUnidirectional => 0x02,
            StreamType::ServerInitiatetUnidirectional => 0x03,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct StreamId {
    stream_type: StreamType,
    id: i64,
}

impl TryFrom<i64> for StreamId {
    type Error = &'static str;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let sid = Self{
            id: (value >> 2),
            stream_type: StreamType::try_from((value & 0x03) as u8)?,
        };
        Ok(sid)
    }
}

impl From<StreamId> for i64 {
    fn from(value: StreamId) -> Self {
        (value.id << 2) | i64::from(value.stream_type)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_first_stream_id_of_type_zero() {
        let subject = StreamId{
            stream_type: StreamType::ClientInitiatetBidirectional,
            id: 0x00,
        };
        let result = i64::from(subject);
        assert_eq!(result, 0x00)
    }

    #[test]
    fn convert_second_stream_id_of_type_zero() {
        let subject = StreamId{
            stream_type: StreamType::ClientInitiatetBidirectional,
            id: 0x01
        };
        let result = i64::from(subject);
        assert_eq!(result, 4);
    }

    #[test]
    fn convert_first_stream_id_of_type_other() {
        let subject = StreamId{
            stream_type: StreamType::ServerInitiatetBidirectional,
            id: 0x00,
        };
        let result: i64 = subject.into();
        assert_eq!(result, 0x01)
    }

    #[test]
    fn convert_second_stream_id_of_type_other() {
        let subject = StreamId{
            stream_type: StreamType::ServerInitiatetBidirectional,
            id: 0x01
        };
        let result: i64 = subject.into();
        assert_eq!(result, 0x05);
    }

    #[test]
    fn convert_i64_to_stream_id_valid_input() {
        let subject: i64 = 0x00;
        let result: StreamId = subject.try_into().expect("DEFINITION: input is valid");
        assert_eq!(result, StreamId{
            stream_type: StreamType::ClientInitiatetBidirectional,
            id: 0x00,
        });

        let subject: i64 = 0x04;
        let result: StreamId = subject.try_into().expect("DEFINITION: input is valid");
        assert_eq!(result, StreamId{
            stream_type: StreamType::ClientInitiatetBidirectional,
            id: 0x01,
        });
        
        let subject: i64 = 0x07;
        let result: StreamId = subject.try_into().expect("DEFINITION: input is valid");
        assert_eq!(result, StreamId{
            stream_type: StreamType::ServerInitiatetUnidirectional,
            id: 0x01,
        });
    }
    
    #[test]
    fn convert_i64_to_stream_id_invalid_type() {
        let subject: u8 = 0x04;
        let result: Result<StreamType, _> = subject.try_into();
        assert!(result.is_err())
    }
}
