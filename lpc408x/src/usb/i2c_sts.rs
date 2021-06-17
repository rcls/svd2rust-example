#[doc = "Register `I2C_STS` reader"]
pub struct R(crate::R<I2C_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Transaction Done Interrupt. This flag is set if a transaction completes successfully. It is cleared by writing a one to bit 0 of the status register. It is unaffected by slave transactions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDI_A {
    #[doc = "0: Transaction has not completed."]
    TRANSACTION_HAS_NOT_ = 0,
    #[doc = "1: Transaction completed."]
    TRANSACTION_COMPLETE = 1,
}
impl From<TDI_A> for bool {
    #[inline(always)]
    fn from(variant: TDI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDI` reader - Transaction Done Interrupt. This flag is set if a transaction completes successfully. It is cleared by writing a one to bit 0 of the status register. It is unaffected by slave transactions."]
pub struct TDI_R(crate::FieldReader<bool, TDI_A>);
impl TDI_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDI_A {
        match self.bits {
            false => TDI_A::TRANSACTION_HAS_NOT_,
            true => TDI_A::TRANSACTION_COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `TRANSACTION_HAS_NOT_`"]
    #[inline(always)]
    pub fn is_transaction_has_not_(&self) -> bool {
        **self == TDI_A::TRANSACTION_HAS_NOT_
    }
    #[doc = "Checks if the value of the field is `TRANSACTION_COMPLETE`"]
    #[inline(always)]
    pub fn is_transaction_complete(&self) -> bool {
        **self == TDI_A::TRANSACTION_COMPLETE
    }
}
impl core::ops::Deref for TDI_R {
    type Target = crate::FieldReader<bool, TDI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Arbitration Failure Interrupt. When transmitting, if the SDA is low when SDAOUT is high, then this I2C has lost the arbitration to another device on the bus. The Arbitration Failure bit is set when this happens. It is cleared by writing a one to bit 1 of the status register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFI_A {
    #[doc = "0: No arbitration failure on last transmission."]
    NO_ARBITRATION_FAILU = 0,
    #[doc = "1: Arbitration failure occurred on last transmission."]
    ARBITRATION_FAILURE_ = 1,
}
impl From<AFI_A> for bool {
    #[inline(always)]
    fn from(variant: AFI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AFI` reader - Arbitration Failure Interrupt. When transmitting, if the SDA is low when SDAOUT is high, then this I2C has lost the arbitration to another device on the bus. The Arbitration Failure bit is set when this happens. It is cleared by writing a one to bit 1 of the status register."]
pub struct AFI_R(crate::FieldReader<bool, AFI_A>);
impl AFI_R {
    pub(crate) fn new(bits: bool) -> Self {
        AFI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AFI_A {
        match self.bits {
            false => AFI_A::NO_ARBITRATION_FAILU,
            true => AFI_A::ARBITRATION_FAILURE_,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ARBITRATION_FAILU`"]
    #[inline(always)]
    pub fn is_no_arbitration_failu(&self) -> bool {
        **self == AFI_A::NO_ARBITRATION_FAILU
    }
    #[doc = "Checks if the value of the field is `ARBITRATION_FAILURE_`"]
    #[inline(always)]
    pub fn is_arbitration_failure_(&self) -> bool {
        **self == AFI_A::ARBITRATION_FAILURE_
    }
}
impl core::ops::Deref for AFI_R {
    type Target = crate::FieldReader<bool, AFI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "No Acknowledge Interrupt. After every byte of data is sent, the transmitter expects an acknowledge from the receiver. This bit is set if the acknowledge is not received. It is cleared when a byte is written to the master TX FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NAI_A {
    #[doc = "0: Last transmission received an acknowledge."]
    LAST_TRANSMISSION_RE = 0,
    #[doc = "1: Last transmission did not receive an acknowledge."]
    LAST_TRANSMISSION_DI = 1,
}
impl From<NAI_A> for bool {
    #[inline(always)]
    fn from(variant: NAI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NAI` reader - No Acknowledge Interrupt. After every byte of data is sent, the transmitter expects an acknowledge from the receiver. This bit is set if the acknowledge is not received. It is cleared when a byte is written to the master TX FIFO."]
pub struct NAI_R(crate::FieldReader<bool, NAI_A>);
impl NAI_R {
    pub(crate) fn new(bits: bool) -> Self {
        NAI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NAI_A {
        match self.bits {
            false => NAI_A::LAST_TRANSMISSION_RE,
            true => NAI_A::LAST_TRANSMISSION_DI,
        }
    }
    #[doc = "Checks if the value of the field is `LAST_TRANSMISSION_RE`"]
    #[inline(always)]
    pub fn is_last_transmission_re(&self) -> bool {
        **self == NAI_A::LAST_TRANSMISSION_RE
    }
    #[doc = "Checks if the value of the field is `LAST_TRANSMISSION_DI`"]
    #[inline(always)]
    pub fn is_last_transmission_di(&self) -> bool {
        **self == NAI_A::LAST_TRANSMISSION_DI
    }
}
impl core::ops::Deref for NAI_R {
    type Target = crate::FieldReader<bool, NAI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Master Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a stop condition or it will hold SCL low until more data is available. The Master Data Request bit is set when the master transmitter is data-starved. If the master TX FIFO is empty and the last byte did not have a STOP condition flag, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the master TX FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRMI_A {
    #[doc = "0: Master transmitter does not need data."]
    MASTER_TRANSMITTER_D = 0,
    #[doc = "1: Master transmitter needs data."]
    MASTER_TRANSMITTER_N = 1,
}
impl From<DRMI_A> for bool {
    #[inline(always)]
    fn from(variant: DRMI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRMI` reader - Master Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a stop condition or it will hold SCL low until more data is available. The Master Data Request bit is set when the master transmitter is data-starved. If the master TX FIFO is empty and the last byte did not have a STOP condition flag, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the master TX FIFO."]
pub struct DRMI_R(crate::FieldReader<bool, DRMI_A>);
impl DRMI_R {
    pub(crate) fn new(bits: bool) -> Self {
        DRMI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRMI_A {
        match self.bits {
            false => DRMI_A::MASTER_TRANSMITTER_D,
            true => DRMI_A::MASTER_TRANSMITTER_N,
        }
    }
    #[doc = "Checks if the value of the field is `MASTER_TRANSMITTER_D`"]
    #[inline(always)]
    pub fn is_master_transmitter_d(&self) -> bool {
        **self == DRMI_A::MASTER_TRANSMITTER_D
    }
    #[doc = "Checks if the value of the field is `MASTER_TRANSMITTER_N`"]
    #[inline(always)]
    pub fn is_master_transmitter_n(&self) -> bool {
        **self == DRMI_A::MASTER_TRANSMITTER_N
    }
}
impl core::ops::Deref for DRMI_R {
    type Target = crate::FieldReader<bool, DRMI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Slave Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a STOP condition or it will hold SCL low until more data is available. The Slave Data Request bit is set when the slave transmitter is data-starved. If the slave TX FIFO is empty and the last byte transmitted was acknowledged, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the slave Tx FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRSI_A {
    #[doc = "0: Slave transmitter does not need data."]
    SLAVE_TRANSMITTER_DO = 0,
    #[doc = "1: Slave transmitter needs data."]
    SLAVE_TRANSMITTER_NE = 1,
}
impl From<DRSI_A> for bool {
    #[inline(always)]
    fn from(variant: DRSI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRSI` reader - Slave Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a STOP condition or it will hold SCL low until more data is available. The Slave Data Request bit is set when the slave transmitter is data-starved. If the slave TX FIFO is empty and the last byte transmitted was acknowledged, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the slave Tx FIFO."]
pub struct DRSI_R(crate::FieldReader<bool, DRSI_A>);
impl DRSI_R {
    pub(crate) fn new(bits: bool) -> Self {
        DRSI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRSI_A {
        match self.bits {
            false => DRSI_A::SLAVE_TRANSMITTER_DO,
            true => DRSI_A::SLAVE_TRANSMITTER_NE,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE_TRANSMITTER_DO`"]
    #[inline(always)]
    pub fn is_slave_transmitter_do(&self) -> bool {
        **self == DRSI_A::SLAVE_TRANSMITTER_DO
    }
    #[doc = "Checks if the value of the field is `SLAVE_TRANSMITTER_NE`"]
    #[inline(always)]
    pub fn is_slave_transmitter_ne(&self) -> bool {
        **self == DRSI_A::SLAVE_TRANSMITTER_NE
    }
}
impl core::ops::Deref for DRSI_R {
    type Target = crate::FieldReader<bool, DRSI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Active` reader - Indicates whether the bus is busy. This bit is set when a START condition has been seen. It is cleared when a STOP condition is seen.."]
pub struct ACTIVE_R(crate::FieldReader<bool, bool>);
impl ACTIVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCL` reader - The current value of the SCL signal."]
pub struct SCL_R(crate::FieldReader<bool, bool>);
impl SCL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDA` reader - The current value of the SDA signal."]
pub struct SDA_R(crate::FieldReader<bool, bool>);
impl SDA_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Receive FIFO Full (RFF). This bit is set when the RX FIFO is full and cannot accept any more data. It is cleared when the RX FIFO is not full. If a byte arrives when the Receive FIFO is full, the SCL is held low until the CPU reads the RX FIFO and makes room for it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFF_A {
    #[doc = "0: RX FIFO is not full"]
    RX_FIFO_IS_NOT_FULL = 0,
    #[doc = "1: RX FIFO is full"]
    RX_FIFO_IS_FULL = 1,
}
impl From<RFF_A> for bool {
    #[inline(always)]
    fn from(variant: RFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFF` reader - Receive FIFO Full (RFF). This bit is set when the RX FIFO is full and cannot accept any more data. It is cleared when the RX FIFO is not full. If a byte arrives when the Receive FIFO is full, the SCL is held low until the CPU reads the RX FIFO and makes room for it."]
pub struct RFF_R(crate::FieldReader<bool, RFF_A>);
impl RFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFF_A {
        match self.bits {
            false => RFF_A::RX_FIFO_IS_NOT_FULL,
            true => RFF_A::RX_FIFO_IS_FULL,
        }
    }
    #[doc = "Checks if the value of the field is `RX_FIFO_IS_NOT_FULL`"]
    #[inline(always)]
    pub fn is_rx_fifo_is_not_full(&self) -> bool {
        **self == RFF_A::RX_FIFO_IS_NOT_FULL
    }
    #[doc = "Checks if the value of the field is `RX_FIFO_IS_FULL`"]
    #[inline(always)]
    pub fn is_rx_fifo_is_full(&self) -> bool {
        **self == RFF_A::RX_FIFO_IS_FULL
    }
}
impl core::ops::Deref for RFF_R {
    type Target = crate::FieldReader<bool, RFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Receive FIFO Empty. RFE is set when the RX FIFO is empty and is cleared when the RX FIFO contains valid data.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFE_A {
    #[doc = "0: RX FIFO contains data."]
    RX_FIFO_CONTAINS_DAT = 0,
    #[doc = "1: RX FIFO is empty"]
    RX_FIFO_IS_EMPTY = 1,
}
impl From<RFE_A> for bool {
    #[inline(always)]
    fn from(variant: RFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFE` reader - Receive FIFO Empty. RFE is set when the RX FIFO is empty and is cleared when the RX FIFO contains valid data."]
pub struct RFE_R(crate::FieldReader<bool, RFE_A>);
impl RFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFE_A {
        match self.bits {
            false => RFE_A::RX_FIFO_CONTAINS_DAT,
            true => RFE_A::RX_FIFO_IS_EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `RX_FIFO_CONTAINS_DAT`"]
    #[inline(always)]
    pub fn is_rx_fifo_contains_dat(&self) -> bool {
        **self == RFE_A::RX_FIFO_CONTAINS_DAT
    }
    #[doc = "Checks if the value of the field is `RX_FIFO_IS_EMPTY`"]
    #[inline(always)]
    pub fn is_rx_fifo_is_empty(&self) -> bool {
        **self == RFE_A::RX_FIFO_IS_EMPTY
    }
}
impl core::ops::Deref for RFE_R {
    type Target = crate::FieldReader<bool, RFE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit FIFO Full. TFF is set when the TX FIFO is full and is cleared when the TX FIFO is not full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFF_A {
    #[doc = "0: TX FIFO is not full."]
    TX_FIFO_IS_NOT_FULL_ = 0,
    #[doc = "1: TX FIFO is full"]
    TX_FIFO_IS_FULL = 1,
}
impl From<TFF_A> for bool {
    #[inline(always)]
    fn from(variant: TFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFF` reader - Transmit FIFO Full. TFF is set when the TX FIFO is full and is cleared when the TX FIFO is not full."]
pub struct TFF_R(crate::FieldReader<bool, TFF_A>);
impl TFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFF_A {
        match self.bits {
            false => TFF_A::TX_FIFO_IS_NOT_FULL_,
            true => TFF_A::TX_FIFO_IS_FULL,
        }
    }
    #[doc = "Checks if the value of the field is `TX_FIFO_IS_NOT_FULL_`"]
    #[inline(always)]
    pub fn is_tx_fifo_is_not_full_(&self) -> bool {
        **self == TFF_A::TX_FIFO_IS_NOT_FULL_
    }
    #[doc = "Checks if the value of the field is `TX_FIFO_IS_FULL`"]
    #[inline(always)]
    pub fn is_tx_fifo_is_full(&self) -> bool {
        **self == TFF_A::TX_FIFO_IS_FULL
    }
}
impl core::ops::Deref for TFF_R {
    type Target = crate::FieldReader<bool, TFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit FIFO Empty. TFE is set when the TX FIFO is empty and is cleared when the TX FIFO contains valid data.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFE_A {
    #[doc = "0: TX FIFO contains valid data."]
    TX_FIFO_CONTAINS_VAL = 0,
    #[doc = "1: TX FIFO is empty"]
    TX_FIFO_IS_EMPTY = 1,
}
impl From<TFE_A> for bool {
    #[inline(always)]
    fn from(variant: TFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFE` reader - Transmit FIFO Empty. TFE is set when the TX FIFO is empty and is cleared when the TX FIFO contains valid data."]
pub struct TFE_R(crate::FieldReader<bool, TFE_A>);
impl TFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFE_A {
        match self.bits {
            false => TFE_A::TX_FIFO_CONTAINS_VAL,
            true => TFE_A::TX_FIFO_IS_EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `TX_FIFO_CONTAINS_VAL`"]
    #[inline(always)]
    pub fn is_tx_fifo_contains_val(&self) -> bool {
        **self == TFE_A::TX_FIFO_CONTAINS_VAL
    }
    #[doc = "Checks if the value of the field is `TX_FIFO_IS_EMPTY`"]
    #[inline(always)]
    pub fn is_tx_fifo_is_empty(&self) -> bool {
        **self == TFE_A::TX_FIFO_IS_EMPTY
    }
}
impl core::ops::Deref for TFE_R {
    type Target = crate::FieldReader<bool, TFE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Transaction Done Interrupt. This flag is set if a transaction completes successfully. It is cleared by writing a one to bit 0 of the status register. It is unaffected by slave transactions."]
    #[inline(always)]
    pub fn tdi(&self) -> TDI_R {
        TDI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Arbitration Failure Interrupt. When transmitting, if the SDA is low when SDAOUT is high, then this I2C has lost the arbitration to another device on the bus. The Arbitration Failure bit is set when this happens. It is cleared by writing a one to bit 1 of the status register."]
    #[inline(always)]
    pub fn afi(&self) -> AFI_R {
        AFI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - No Acknowledge Interrupt. After every byte of data is sent, the transmitter expects an acknowledge from the receiver. This bit is set if the acknowledge is not received. It is cleared when a byte is written to the master TX FIFO."]
    #[inline(always)]
    pub fn nai(&self) -> NAI_R {
        NAI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Master Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a stop condition or it will hold SCL low until more data is available. The Master Data Request bit is set when the master transmitter is data-starved. If the master TX FIFO is empty and the last byte did not have a STOP condition flag, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the master TX FIFO."]
    #[inline(always)]
    pub fn drmi(&self) -> DRMI_R {
        DRMI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Slave Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a STOP condition or it will hold SCL low until more data is available. The Slave Data Request bit is set when the slave transmitter is data-starved. If the slave TX FIFO is empty and the last byte transmitted was acknowledged, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the slave Tx FIFO."]
    #[inline(always)]
    pub fn drsi(&self) -> DRSI_R {
        DRSI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Indicates whether the bus is busy. This bit is set when a START condition has been seen. It is cleared when a STOP condition is seen.."]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The current value of the SCL signal."]
    #[inline(always)]
    pub fn scl(&self) -> SCL_R {
        SCL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The current value of the SDA signal."]
    #[inline(always)]
    pub fn sda(&self) -> SDA_R {
        SDA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receive FIFO Full (RFF). This bit is set when the RX FIFO is full and cannot accept any more data. It is cleared when the RX FIFO is not full. If a byte arrives when the Receive FIFO is full, the SCL is held low until the CPU reads the RX FIFO and makes room for it."]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Receive FIFO Empty. RFE is set when the RX FIFO is empty and is cleared when the RX FIFO contains valid data."]
    #[inline(always)]
    pub fn rfe(&self) -> RFE_R {
        RFE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmit FIFO Full. TFF is set when the TX FIFO is full and is cleared when the TX FIFO is not full."]
    #[inline(always)]
    pub fn tff(&self) -> TFF_R {
        TFF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Transmit FIFO Empty. TFE is set when the TX FIFO is empty and is cleared when the TX FIFO contains valid data."]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
#[doc = "I2C Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_sts](index.html) module"]
pub struct I2C_STS_SPEC;
impl crate::RegisterSpec for I2C_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_sts::R](R) reader structure"]
impl crate::Readable for I2C_STS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets I2C_STS to value 0x0a00"]
impl crate::Resettable for I2C_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0a00
    }
}
