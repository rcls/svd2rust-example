#[doc = "Register `SENSE_DUTY` reader"]
pub struct R(crate::R<SENSE_DUTY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SENSE_DUTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SENSE_DUTY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SENSE_DUTY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SENSE_DUTY` writer"]
pub struct W(crate::W<SENSE_DUTY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SENSE_DUTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SENSE_DUTY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SENSE_DUTY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SENSE_WIDTH` reader - Defines the length of the first phase of the sense clock in clk_csd cycles. A value of 0 disables this feature and the duty cycle of csd_sense will be 50 percent, which is equal to SENSE_WIDTH = (SENSE_DIV+1)/2, or when clock dithering is used that becomes \\[(SENSE_DIV+1) + (LFSR_OUT << LSFR_SCALE)\\]/2. At all times it must be assured that the phases are at least 2 clk_csd cycles (1 for non overlap, if used), if this rule is violated the result is undefined. Note that this feature is not available when SEL_LFSR_MSB (PRS) is selected."]
pub struct SENSE_WIDTH_R(crate::FieldReader<u16, u16>);
impl SENSE_WIDTH_R {
    pub(crate) fn new(bits: u16) -> Self {
        SENSE_WIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENSE_WIDTH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE_WIDTH` writer - Defines the length of the first phase of the sense clock in clk_csd cycles. A value of 0 disables this feature and the duty cycle of csd_sense will be 50 percent, which is equal to SENSE_WIDTH = (SENSE_DIV+1)/2, or when clock dithering is used that becomes \\[(SENSE_DIV+1) + (LFSR_OUT << LSFR_SCALE)\\]/2. At all times it must be assured that the phases are at least 2 clk_csd cycles (1 for non overlap, if used), if this rule is violated the result is undefined. Note that this feature is not available when SEL_LFSR_MSB (PRS) is selected."]
pub struct SENSE_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `SENSE_POL` reader - Polarity of the sense clock 0 = start with low phase (typical for regular negative transfer CSD) 1 = start with high phase"]
pub struct SENSE_POL_R(crate::FieldReader<bool, bool>);
impl SENSE_POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SENSE_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENSE_POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE_POL` writer - Polarity of the sense clock 0 = start with low phase (typical for regular negative transfer CSD) 1 = start with high phase"]
pub struct SENSE_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE_POL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `OVERLAP_PHI1` reader - NonOverlap or not for Phi1 (csd_sense=0). 0 = Non-overlap for Phi1, the Phi1 signal is csd_sense inverted except that the signal goes low 1 clk_sample before csd_sense goes high. Intended usage: new low EMI CSD/CSX with static GPIO. 1 = 'Overlap' (= not non-overlap) for Phi1, the Phi1 signal is csd_sense inverted. Intended usage: legacy CSD with GPIO switching, the GPIO internal circuit ensures that the switches are non-overlapping."]
pub struct OVERLAP_PHI1_R(crate::FieldReader<bool, bool>);
impl OVERLAP_PHI1_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERLAP_PHI1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERLAP_PHI1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERLAP_PHI1` writer - NonOverlap or not for Phi1 (csd_sense=0). 0 = Non-overlap for Phi1, the Phi1 signal is csd_sense inverted except that the signal goes low 1 clk_sample before csd_sense goes high. Intended usage: new low EMI CSD/CSX with static GPIO. 1 = 'Overlap' (= not non-overlap) for Phi1, the Phi1 signal is csd_sense inverted. Intended usage: legacy CSD with GPIO switching, the GPIO internal circuit ensures that the switches are non-overlapping."]
pub struct OVERLAP_PHI1_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERLAP_PHI1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `OVERLAP_PHI2` reader - Same as OVERLAP_PHI1 but for Phi2 (csd_sense=1)."]
pub struct OVERLAP_PHI2_R(crate::FieldReader<bool, bool>);
impl OVERLAP_PHI2_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERLAP_PHI2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERLAP_PHI2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERLAP_PHI2` writer - Same as OVERLAP_PHI1 but for Phi2 (csd_sense=1)."]
pub struct OVERLAP_PHI2_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERLAP_PHI2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Defines the length of the first phase of the sense clock in clk_csd cycles. A value of 0 disables this feature and the duty cycle of csd_sense will be 50 percent, which is equal to SENSE_WIDTH = (SENSE_DIV+1)/2, or when clock dithering is used that becomes \\[(SENSE_DIV+1) + (LFSR_OUT << LSFR_SCALE)\\]/2. At all times it must be assured that the phases are at least 2 clk_csd cycles (1 for non overlap, if used), if this rule is violated the result is undefined. Note that this feature is not available when SEL_LFSR_MSB (PRS) is selected."]
    #[inline(always)]
    pub fn sense_width(&self) -> SENSE_WIDTH_R {
        SENSE_WIDTH_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - Polarity of the sense clock 0 = start with low phase (typical for regular negative transfer CSD) 1 = start with high phase"]
    #[inline(always)]
    pub fn sense_pol(&self) -> SENSE_POL_R {
        SENSE_POL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - NonOverlap or not for Phi1 (csd_sense=0). 0 = Non-overlap for Phi1, the Phi1 signal is csd_sense inverted except that the signal goes low 1 clk_sample before csd_sense goes high. Intended usage: new low EMI CSD/CSX with static GPIO. 1 = 'Overlap' (= not non-overlap) for Phi1, the Phi1 signal is csd_sense inverted. Intended usage: legacy CSD with GPIO switching, the GPIO internal circuit ensures that the switches are non-overlapping."]
    #[inline(always)]
    pub fn overlap_phi1(&self) -> OVERLAP_PHI1_R {
        OVERLAP_PHI1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Same as OVERLAP_PHI1 but for Phi2 (csd_sense=1)."]
    #[inline(always)]
    pub fn overlap_phi2(&self) -> OVERLAP_PHI2_R {
        OVERLAP_PHI2_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Defines the length of the first phase of the sense clock in clk_csd cycles. A value of 0 disables this feature and the duty cycle of csd_sense will be 50 percent, which is equal to SENSE_WIDTH = (SENSE_DIV+1)/2, or when clock dithering is used that becomes \\[(SENSE_DIV+1) + (LFSR_OUT << LSFR_SCALE)\\]/2. At all times it must be assured that the phases are at least 2 clk_csd cycles (1 for non overlap, if used), if this rule is violated the result is undefined. Note that this feature is not available when SEL_LFSR_MSB (PRS) is selected."]
    #[inline(always)]
    pub fn sense_width(&mut self) -> SENSE_WIDTH_W {
        SENSE_WIDTH_W { w: self }
    }
    #[doc = "Bit 16 - Polarity of the sense clock 0 = start with low phase (typical for regular negative transfer CSD) 1 = start with high phase"]
    #[inline(always)]
    pub fn sense_pol(&mut self) -> SENSE_POL_W {
        SENSE_POL_W { w: self }
    }
    #[doc = "Bit 18 - NonOverlap or not for Phi1 (csd_sense=0). 0 = Non-overlap for Phi1, the Phi1 signal is csd_sense inverted except that the signal goes low 1 clk_sample before csd_sense goes high. Intended usage: new low EMI CSD/CSX with static GPIO. 1 = 'Overlap' (= not non-overlap) for Phi1, the Phi1 signal is csd_sense inverted. Intended usage: legacy CSD with GPIO switching, the GPIO internal circuit ensures that the switches are non-overlapping."]
    #[inline(always)]
    pub fn overlap_phi1(&mut self) -> OVERLAP_PHI1_W {
        OVERLAP_PHI1_W { w: self }
    }
    #[doc = "Bit 19 - Same as OVERLAP_PHI1 but for Phi2 (csd_sense=1)."]
    #[inline(always)]
    pub fn overlap_phi2(&mut self) -> OVERLAP_PHI2_W {
        OVERLAP_PHI2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sense clock duty cycle\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sense_duty](index.html) module"]
pub struct SENSE_DUTY_SPEC;
impl crate::RegisterSpec for SENSE_DUTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sense_duty::R](R) reader structure"]
impl crate::Readable for SENSE_DUTY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sense_duty::W](W) writer structure"]
impl crate::Writable for SENSE_DUTY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SENSE_DUTY to value 0"]
impl crate::Resettable for SENSE_DUTY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
