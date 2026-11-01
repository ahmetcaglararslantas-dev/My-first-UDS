/// PyCAN-Diag - UDS Diagnostic Request Builder & Simulator Module
/// Author: Ahmet Çağlar Arslantaş

#[derive(Debug, PartialEq,Clone,Copy)]
pub enum UDSService {
    DiagnosticSessionControl = 0x10,
    ReadDataByIdentifier = 0x22,
    ReadDTCInformation = 0x19,
    ClearDiagnosticInformation = 0x14,
}

pub struct UDSRequest {
    service_id: UDSService,
    sub_function: Option<u8>,
    data: Vec<u8>,
}

impl UDSRequest {
    pub fn new(service_id: UDSService) -> Self {
        Self {
            service_id,
            sub_function: None,
            data: Vec::new(),
        }
    }

    pub fn with_sub_function(mut self, sub_fn: u8) -> Self {
        self.sub_function = Some(sub_fn);
        self
    }

    pub fn with_data(mut self, data: &[u8]) -> Self {
        self.data = data.to_vec();
        self
    }

    pub fn build_frame(&self) -> Vec<u8> {
        let mut frame = Vec::new();
        frame.push(self.service_id as u8);
        
        if let Some(sub_fn) = self.sub_function {
            frame.push(sub_fn);
        }
        
        frame.extend(&self.data);
        frame
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uds_request_building() {
        let req = UDSRequest::new(UDSService::ReadDataByIdentifier)
                    .with_data(&[0xF1, 0x90]);
        
        let frame = req.build_frame();
        assert_eq!(frame, vec![0x22, 0xF1, 0x90]);
    }
}