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
#[doc = "Field `CUR_CHAN` reader - current channel being sampled (channel 16 indicates the injection channel), only valid if BUSY."]
pub struct CUR_CHAN_R(crate::FieldReader<u8, u8>);
impl CUR_CHAN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CUR_CHAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CUR_CHAN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_VREF_NEG` reader - the current switch status, including DSI and sequencer controls, of the switch in the SARADC that shorts NEG with VREF input (see NEG_SEL)."]
pub struct SW_VREF_NEG_R(crate::FieldReader<bool, bool>);
impl SW_VREF_NEG_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_VREF_NEG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_VREF_NEG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSY` reader - If high then the SAR is busy with a conversion. This bit is always high when CONTINUOUS is set. Firmware should wait for this bit to be low before putting the SAR in power down."]
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4 - current channel being sampled (channel 16 indicates the injection channel), only valid if BUSY."]
    #[inline(always)]
    pub fn cur_chan(&self) -> CUR_CHAN_R {
        CUR_CHAN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 30 - the current switch status, including DSI and sequencer controls, of the switch in the SARADC that shorts NEG with VREF input (see NEG_SEL)."]
    #[inline(always)]
    pub fn sw_vref_neg(&self) -> SW_VREF_NEG_R {
        SW_VREF_NEG_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - If high then the SAR is busy with a conversion. This bit is always high when CONTINUOUS is set. Firmware should wait for this bit to be low before putting the SAR in power down."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Current status of internal SAR registers (mostly for debug)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
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
