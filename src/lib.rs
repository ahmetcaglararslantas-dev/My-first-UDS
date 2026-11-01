#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UDSService {
    DiagnosticSessionControl = 0x10,
    ReadDataByIdentifier = 0x22,
    ReadDTCInformation = 0x19,
    ClearDiagnosticInformation = 0x14,
}

pub struct UDSRequest {
    pub service_id: UDSService,
    pub sub_function: Option<u8>,
    pub data: Vec<u8>,
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

#[derive(Debug, PartialEq)]
pub enum UDSResponse {
    Positive {
        service_id: UDSService,
        data: Vec<u8>,
    },
    Negative {
        service_id: UDSService,
        nrc: u8,
    },
}

impl UDSResponse {
    pub fn parse(request_service: UDSService, raw_data: &[u8]) -> Result<Self, &'static str> {
        if raw_data.is_empty() {
            return Err("Boş yanıt alındı");
        }

        if raw_data[0] == 0x7F {
            if raw_data.len() >= 3 {
                return Ok(UDSResponse::Negative {
                    service_id: request_service,
                    nrc: raw_data[2],
                });
            }
            return Err("Geçersiz negatif yanıt formatı");
        }

        let expected_positive_sid = (request_service as u8) | 0x40;
        if raw_data[0] == expected_positive_sid {
            return Ok(UDSResponse::Positive {
                service_id: request_service,
                data: raw_data[1..].to_vec(),
            });
        }

        Err("Eşleşmeyen servis yanıt ID'si")
    }
}

pub struct CanFrame {
    pub id: u32,       // CAN ID (Örn: 0x7DF - Functional, 0x7E0 - Physical Request)
    pub data: Vec<u8>, // En fazla 8 bayt olmalı
}

impl CanFrame {
    // UDS payload'unu ISO-TP Single Frame kurallarına göre CAN frame'ine dönüştürür
    pub fn from_uds_request(can_id: u32, request: &UDSRequest) -> Result<Self, &'static str> {
        let uds_data = request.build_frame();
        let payload_len = uds_data.len();

        // ISO-TP Single Frame için veri boyutu en fazla 7 bayt olmalıdır (1 bayt PCI için)
        if payload_len > 7 {
            return Err("Mesaj çok uzun! ISO-TP Consecutive Frame (Çoklu çerçeve) gerekiyor.");
        }

        let mut frame_data = Vec::with_capacity(8);
        // ISO-TP Single Frame PCI (Protocol Control Information): Üst nibble 0x0, alt nibble veri uzunluğu
        frame_data.push(payload_len as u8);
        frame_data.extend(uds_data);

        // Kalan boşlukları 0x00 ile doldurabiliriz (Padding)
        while frame_data.len() < 8 {
            frame_data.push(0x00);
        }

        Ok(Self {
            id: can_id,
            data: frame_data,
        })
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
  #[test]
    fn test_isotp_single_frame_creation() {
        let req = UDSRequest::new(UDSService::DiagnosticSessionControl)
                    .with_sub_function(0x01); // 0x10 0x01
        
        let can_frame = CanFrame::from_uds_request(0x7E0, &req).unwrap();
        
        assert_eq!(can_frame.id, 0x7E0);
        // İlk bayt ISO-TP PCI (uzunluk = 2), sonra 0x10 ve 0x01, kalanı padding (0x00)
        assert_eq!(can_frame.data[0], 0x02); 
        assert_eq!(can_frame.data[1], 0x10);
        assert_eq!(can_frame.data[2], 0x01);
        assert_eq!(can_frame.data.len(), 8);
    }

    #[test]
    fn test_uds_positive_response_parsing() {
        let raw_response = vec![0x62, 0xAB, 0xCD];
        let parsed = UDSResponse::parse(UDSService::ReadDataByIdentifier, &raw_response);

        assert!(parsed.is_ok());
        if let Ok(UDSResponse::Positive { service_id, data }) = parsed {
            assert_eq!(service_id, UDSService::ReadDataByIdentifier);
            assert_eq!(data, vec![0xAB, 0xCD]);
        } else {
            panic!("Pozitif yanıt bekleniyordu");
        }
    }
}