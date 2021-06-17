#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMDCRCFAIL` reader - Command response received (CRC check failed)."]
pub struct CMDCRCFAIL_R(crate::FieldReader<bool, bool>);
impl CMDCRCFAIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMDCRCFAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDCRCFAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATACRCFAIL` reader - Data block sent/received (CRC check failed)."]
pub struct DATACRCFAIL_R(crate::FieldReader<bool, bool>);
impl DATACRCFAIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATACRCFAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATACRCFAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDTIMEOUT` reader - Command response timeout."]
pub struct CMDTIMEOUT_R(crate::FieldReader<bool, bool>);
impl CMDTIMEOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMDTIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDTIMEOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATATIMEOUT` reader - Data timeout."]
pub struct DATATIMEOUT_R(crate::FieldReader<bool, bool>);
impl DATATIMEOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATATIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATATIMEOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUNDERRUN` reader - Transmit FIFO underrun error."]
pub struct TXUNDERRUN_R(crate::FieldReader<bool, bool>);
impl TXUNDERRUN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUNDERRUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUNDERRUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOVERRUN` reader - Receive FIFO overrun error."]
pub struct RXOVERRUN_R(crate::FieldReader<bool, bool>);
impl RXOVERRUN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOVERRUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOVERRUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDRESPEND` reader - Command response received (CRC check passed)."]
pub struct CMDRESPEND_R(crate::FieldReader<bool, bool>);
impl CMDRESPEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMDRESPEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDRESPEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDSENT` reader - Command sent (no response required)."]
pub struct CMDSENT_R(crate::FieldReader<bool, bool>);
impl CMDSENT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMDSENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDSENT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAEND` reader - Data end (data counter is zero)."]
pub struct DATAEND_R(crate::FieldReader<bool, bool>);
impl DATAEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATAEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STARTBITERR` reader - Start bit not detected on all data signals in wide bus mode."]
pub struct STARTBITERR_R(crate::FieldReader<bool, bool>);
impl STARTBITERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        STARTBITERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STARTBITERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATABLOCKEND` reader - Data block sent/received (CRC check passed)."]
pub struct DATABLOCKEND_R(crate::FieldReader<bool, bool>);
impl DATABLOCKEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATABLOCKEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATABLOCKEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDACTIVE` reader - Command transfer in progress."]
pub struct CMDACTIVE_R(crate::FieldReader<bool, bool>);
impl CMDACTIVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMDACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXACTIVE` reader - Data transmit in progress."]
pub struct TXACTIVE_R(crate::FieldReader<bool, bool>);
impl TXACTIVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXACTIVE` reader - Data receive in progress."]
pub struct RXACTIVE_R(crate::FieldReader<bool, bool>);
impl RXACTIVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFOHALFEMPTY` reader - Transmit FIFO half empty."]
pub struct TXFIFOHALFEMPTY_R(crate::FieldReader<bool, bool>);
impl TXFIFOHALFEMPTY_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFIFOHALFEMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFOHALFEMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFOHALFFULL` reader - Receive FIFO half full."]
pub struct RXFIFOHALFFULL_R(crate::FieldReader<bool, bool>);
impl RXFIFOHALFFULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFOHALFFULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFOHALFFULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFOFULL` reader - Transmit FIFO full."]
pub struct TXFIFOFULL_R(crate::FieldReader<bool, bool>);
impl TXFIFOFULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFIFOFULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFOFULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFOFULL` reader - Receive FIFO full."]
pub struct RXFIFOFULL_R(crate::FieldReader<bool, bool>);
impl RXFIFOFULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFOFULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFOFULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFOEMPTY` reader - Transmit FIFO empty."]
pub struct TXFIFOEMPTY_R(crate::FieldReader<bool, bool>);
impl TXFIFOEMPTY_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFIFOEMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFOEMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFOEMPTY` reader - Receive FIFO empty."]
pub struct RXFIFOEMPTY_R(crate::FieldReader<bool, bool>);
impl RXFIFOEMPTY_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFOEMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFOEMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDATAAVLBL` reader - Data available in transmit FIFO."]
pub struct TXDATAAVLBL_R(crate::FieldReader<bool, bool>);
impl TXDATAAVLBL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXDATAAVLBL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDATAAVLBL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDATAAVLBL` reader - Data available in receive FIFO."]
pub struct RXDATAAVLBL_R(crate::FieldReader<bool, bool>);
impl RXDATAAVLBL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXDATAAVLBL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDATAAVLBL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Command response received (CRC check failed)."]
    #[inline(always)]
    pub fn cmdcrcfail(&self) -> CMDCRCFAIL_R {
        CMDCRCFAIL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data block sent/received (CRC check failed)."]
    #[inline(always)]
    pub fn datacrcfail(&self) -> DATACRCFAIL_R {
        DATACRCFAIL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Command response timeout."]
    #[inline(always)]
    pub fn cmdtimeout(&self) -> CMDTIMEOUT_R {
        CMDTIMEOUT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Data timeout."]
    #[inline(always)]
    pub fn datatimeout(&self) -> DATATIMEOUT_R {
        DATATIMEOUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit FIFO underrun error."]
    #[inline(always)]
    pub fn txunderrun(&self) -> TXUNDERRUN_R {
        TXUNDERRUN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO overrun error."]
    #[inline(always)]
    pub fn rxoverrun(&self) -> RXOVERRUN_R {
        RXOVERRUN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Command response received (CRC check passed)."]
    #[inline(always)]
    pub fn cmdrespend(&self) -> CMDRESPEND_R {
        CMDRESPEND_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Command sent (no response required)."]
    #[inline(always)]
    pub fn cmdsent(&self) -> CMDSENT_R {
        CMDSENT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Data end (data counter is zero)."]
    #[inline(always)]
    pub fn dataend(&self) -> DATAEND_R {
        DATAEND_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Start bit not detected on all data signals in wide bus mode."]
    #[inline(always)]
    pub fn startbiterr(&self) -> STARTBITERR_R {
        STARTBITERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Data block sent/received (CRC check passed)."]
    #[inline(always)]
    pub fn datablockend(&self) -> DATABLOCKEND_R {
        DATABLOCKEND_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Command transfer in progress."]
    #[inline(always)]
    pub fn cmdactive(&self) -> CMDACTIVE_R {
        CMDACTIVE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Data transmit in progress."]
    #[inline(always)]
    pub fn txactive(&self) -> TXACTIVE_R {
        TXACTIVE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Data receive in progress."]
    #[inline(always)]
    pub fn rxactive(&self) -> RXACTIVE_R {
        RXACTIVE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Transmit FIFO half empty."]
    #[inline(always)]
    pub fn txfifohalfempty(&self) -> TXFIFOHALFEMPTY_R {
        TXFIFOHALFEMPTY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Receive FIFO half full."]
    #[inline(always)]
    pub fn rxfifohalffull(&self) -> RXFIFOHALFFULL_R {
        RXFIFOHALFFULL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Transmit FIFO full."]
    #[inline(always)]
    pub fn txfifofull(&self) -> TXFIFOFULL_R {
        TXFIFOFULL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Receive FIFO full."]
    #[inline(always)]
    pub fn rxfifofull(&self) -> RXFIFOFULL_R {
        RXFIFOFULL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Transmit FIFO empty."]
    #[inline(always)]
    pub fn txfifoempty(&self) -> TXFIFOEMPTY_R {
        TXFIFOEMPTY_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Receive FIFO empty."]
    #[inline(always)]
    pub fn rxfifoempty(&self) -> RXFIFOEMPTY_R {
        RXFIFOEMPTY_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Data available in transmit FIFO."]
    #[inline(always)]
    pub fn txdataavlbl(&self) -> TXDATAAVLBL_R {
        TXDATAAVLBL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Data available in receive FIFO."]
    #[inline(always)]
    pub fn rxdataavlbl(&self) -> RXDATAAVLBL_R {
        RXDATAAVLBL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
#[doc = "Status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
