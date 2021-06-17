#[doc = "Register `FLOWCONTROLSTATUS` reader"]
pub struct R(crate::R<FLOWCONTROLSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLOWCONTROLSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLOWCONTROLSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLOWCONTROLSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MCC` reader - MirrorCounterCurrent. In full duplex mode this register represents the current value of the datapath's mirror counter which counts up to the value specified by the MirrorCounter field in the FlowControlCounter register. In half duplex mode the register counts until it reaches the value of the PauseTimer bits in the FlowControlCounter register."]
pub struct MCC_R(crate::FieldReader<u16, u16>);
impl MCC_R {
    pub(crate) fn new(bits: u16) -> Self {
        MCC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - MirrorCounterCurrent. In full duplex mode this register represents the current value of the datapath's mirror counter which counts up to the value specified by the MirrorCounter field in the FlowControlCounter register. In half duplex mode the register counts until it reaches the value of the PauseTimer bits in the FlowControlCounter register."]
    #[inline(always)]
    pub fn mcc(&self) -> MCC_R {
        MCC_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Flow control status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flowcontrolstatus](index.html) module"]
pub struct FLOWCONTROLSTATUS_SPEC;
impl crate::RegisterSpec for FLOWCONTROLSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flowcontrolstatus::R](R) reader structure"]
impl crate::Readable for FLOWCONTROLSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FLOWCONTROLSTATUS to value 0"]
impl crate::Resettable for FLOWCONTROLSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
