#[doc = "Register `INTR_CAUSE_HI` reader"]
pub struct R(crate::R<INTR_CAUSE_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_CAUSE_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_CAUSE_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_CAUSE_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SOF_INTR` reader - USB SOF Interrupt"]
pub struct SOF_INTR_R(crate::FieldReader<bool, bool>);
impl SOF_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOF_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUS_RESET_INTR` reader - BUS RESET Interrupt"]
pub struct BUS_RESET_INTR_R(crate::FieldReader<bool, bool>);
impl BUS_RESET_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUS_RESET_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUS_RESET_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP0_INTR` reader - EP0 Interrupt"]
pub struct EP0_INTR_R(crate::FieldReader<bool, bool>);
impl EP0_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP0_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP0_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPM_INTR` reader - LPM Interrupt"]
pub struct LPM_INTR_R(crate::FieldReader<bool, bool>);
impl LPM_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPM_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPM_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESUME_INTR` reader - Resume Interrupt"]
pub struct RESUME_INTR_R(crate::FieldReader<bool, bool>);
impl RESUME_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESUME_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESUME_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARB_EP_INTR` reader - Arbiter Endpoint Interrupt"]
pub struct ARB_EP_INTR_R(crate::FieldReader<bool, bool>);
impl ARB_EP_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARB_EP_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARB_EP_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP1_INTR` reader - EP1 Interrupt"]
pub struct EP1_INTR_R(crate::FieldReader<bool, bool>);
impl EP1_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP1_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP1_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP2_INTR` reader - EP2 Interrupt"]
pub struct EP2_INTR_R(crate::FieldReader<bool, bool>);
impl EP2_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP2_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP2_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP3_INTR` reader - EP3 Interrupt"]
pub struct EP3_INTR_R(crate::FieldReader<bool, bool>);
impl EP3_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP3_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP3_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP4_INTR` reader - EP4 Interrupt"]
pub struct EP4_INTR_R(crate::FieldReader<bool, bool>);
impl EP4_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP4_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP4_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP5_INTR` reader - EP5 Interrupt"]
pub struct EP5_INTR_R(crate::FieldReader<bool, bool>);
impl EP5_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP5_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP5_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP6_INTR` reader - EP6 Interrupt"]
pub struct EP6_INTR_R(crate::FieldReader<bool, bool>);
impl EP6_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP6_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP6_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP7_INTR` reader - EP7 Interrupt"]
pub struct EP7_INTR_R(crate::FieldReader<bool, bool>);
impl EP7_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP7_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP7_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP8_INTR` reader - EP8 Interrupt"]
pub struct EP8_INTR_R(crate::FieldReader<bool, bool>);
impl EP8_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP8_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP8_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - USB SOF Interrupt"]
    #[inline(always)]
    pub fn sof_intr(&self) -> SOF_INTR_R {
        SOF_INTR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BUS RESET Interrupt"]
    #[inline(always)]
    pub fn bus_reset_intr(&self) -> BUS_RESET_INTR_R {
        BUS_RESET_INTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - EP0 Interrupt"]
    #[inline(always)]
    pub fn ep0_intr(&self) -> EP0_INTR_R {
        EP0_INTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LPM Interrupt"]
    #[inline(always)]
    pub fn lpm_intr(&self) -> LPM_INTR_R {
        LPM_INTR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Resume Interrupt"]
    #[inline(always)]
    pub fn resume_intr(&self) -> RESUME_INTR_R {
        RESUME_INTR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Arbiter Endpoint Interrupt"]
    #[inline(always)]
    pub fn arb_ep_intr(&self) -> ARB_EP_INTR_R {
        ARB_EP_INTR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - EP1 Interrupt"]
    #[inline(always)]
    pub fn ep1_intr(&self) -> EP1_INTR_R {
        EP1_INTR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - EP2 Interrupt"]
    #[inline(always)]
    pub fn ep2_intr(&self) -> EP2_INTR_R {
        EP2_INTR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - EP3 Interrupt"]
    #[inline(always)]
    pub fn ep3_intr(&self) -> EP3_INTR_R {
        EP3_INTR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - EP4 Interrupt"]
    #[inline(always)]
    pub fn ep4_intr(&self) -> EP4_INTR_R {
        EP4_INTR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - EP5 Interrupt"]
    #[inline(always)]
    pub fn ep5_intr(&self) -> EP5_INTR_R {
        EP5_INTR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - EP6 Interrupt"]
    #[inline(always)]
    pub fn ep6_intr(&self) -> EP6_INTR_R {
        EP6_INTR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - EP7 Interrupt"]
    #[inline(always)]
    pub fn ep7_intr(&self) -> EP7_INTR_R {
        EP7_INTR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - EP8 Interrupt"]
    #[inline(always)]
    pub fn ep8_intr(&self) -> EP8_INTR_R {
        EP8_INTR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "High priority interrupt Cause register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_cause_hi](index.html) module"]
pub struct INTR_CAUSE_HI_SPEC;
impl crate::RegisterSpec for INTR_CAUSE_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_cause_hi::R](R) reader structure"]
impl crate::Readable for INTR_CAUSE_HI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_CAUSE_HI to value 0"]
impl crate::Resettable for INTR_CAUSE_HI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
