#[doc = "Register `CAL_CTL3` reader"]
pub struct R(crate::R<CAL_CTL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAL_CTL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAL_CTL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAL_CTL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAL_CTL3` writer"]
pub struct W(crate::W<CAL_CTL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAL_CTL3_SPEC>;
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
impl From<crate::W<CAL_CTL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAL_CTL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSC_TRIM_HV` reader - Flash macro pump clock trim control."]
pub struct OSC_TRIM_HV_R(crate::FieldReader<u8, u8>);
impl OSC_TRIM_HV_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSC_TRIM_HV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSC_TRIM_HV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSC_TRIM_HV` writer - Flash macro pump clock trim control."]
pub struct OSC_TRIM_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC_TRIM_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `OSC_RANGE_TRIM_HV` reader - 0: Oscillator High Frequency Range 1: Oscillator Low Frequency range"]
pub struct OSC_RANGE_TRIM_HV_R(crate::FieldReader<bool, bool>);
impl OSC_RANGE_TRIM_HV_R {
    pub(crate) fn new(bits: bool) -> Self {
        OSC_RANGE_TRIM_HV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSC_RANGE_TRIM_HV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSC_RANGE_TRIM_HV` writer - 0: Oscillator High Frequency Range 1: Oscillator Low Frequency range"]
pub struct OSC_RANGE_TRIM_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC_RANGE_TRIM_HV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `IDAC_HV` reader - N/A"]
pub struct IDAC_HV_R(crate::FieldReader<u8, u8>);
impl IDAC_HV_R {
    pub(crate) fn new(bits: u8) -> Self {
        IDAC_HV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDAC_HV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDAC_HV` writer - N/A"]
pub struct IDAC_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> IDAC_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | ((value as u32 & 0x0f) << 5);
        self.w
    }
}
#[doc = "Field `SDAC_HV` reader - N/A"]
pub struct SDAC_HV_R(crate::FieldReader<u8, u8>);
impl SDAC_HV_R {
    pub(crate) fn new(bits: u8) -> Self {
        SDAC_HV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDAC_HV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDAC_HV` writer - N/A"]
pub struct SDAC_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> SDAC_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "Field `ITIM_HV` reader - Trimming of timing current"]
pub struct ITIM_HV_R(crate::FieldReader<u8, u8>);
impl ITIM_HV_R {
    pub(crate) fn new(bits: u8) -> Self {
        ITIM_HV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITIM_HV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITIM_HV` writer - Trimming of timing current"]
pub struct ITIM_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> ITIM_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 11)) | ((value as u32 & 0x0f) << 11);
        self.w
    }
}
#[doc = "Field `VDDHI_HV` reader - 0': vdd<2.3V '1': vdd>=2.3V"]
pub struct VDDHI_HV_R(crate::FieldReader<bool, bool>);
impl VDDHI_HV_R {
    pub(crate) fn new(bits: bool) -> Self {
        VDDHI_HV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDDHI_HV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDDHI_HV` writer - 0': vdd<2.3V '1': vdd>=2.3V"]
pub struct VDDHI_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDHI_HV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `TURBO_PULSEW_HV` reader - Turbo pulse width trim"]
pub struct TURBO_PULSEW_HV_R(crate::FieldReader<u8, u8>);
impl TURBO_PULSEW_HV_R {
    pub(crate) fn new(bits: u8) -> Self {
        TURBO_PULSEW_HV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TURBO_PULSEW_HV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TURBO_PULSEW_HV` writer - Turbo pulse width trim"]
pub struct TURBO_PULSEW_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> TURBO_PULSEW_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `BGLO_EN_HV` reader - LO Bandgap Enable"]
pub struct BGLO_EN_HV_R(crate::FieldReader<bool, bool>);
impl BGLO_EN_HV_R {
    pub(crate) fn new(bits: bool) -> Self {
        BGLO_EN_HV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BGLO_EN_HV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BGLO_EN_HV` writer - LO Bandgap Enable"]
pub struct BGLO_EN_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> BGLO_EN_HV_W<'a> {
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
#[doc = "Field `BGHI_EN_HV` reader - HI Bandgap Enable"]
pub struct BGHI_EN_HV_R(crate::FieldReader<bool, bool>);
impl BGHI_EN_HV_R {
    pub(crate) fn new(bits: bool) -> Self {
        BGHI_EN_HV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BGHI_EN_HV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BGHI_EN_HV` writer - HI Bandgap Enable"]
pub struct BGHI_EN_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> BGHI_EN_HV_W<'a> {
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
    #[doc = "Bits 0:3 - Flash macro pump clock trim control."]
    #[inline(always)]
    pub fn osc_trim_hv(&self) -> OSC_TRIM_HV_R {
        OSC_TRIM_HV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 0: Oscillator High Frequency Range 1: Oscillator Low Frequency range"]
    #[inline(always)]
    pub fn osc_range_trim_hv(&self) -> OSC_RANGE_TRIM_HV_R {
        OSC_RANGE_TRIM_HV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:8 - N/A"]
    #[inline(always)]
    pub fn idac_hv(&self) -> IDAC_HV_R {
        IDAC_HV_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:10 - N/A"]
    #[inline(always)]
    pub fn sdac_hv(&self) -> SDAC_HV_R {
        SDAC_HV_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bits 11:14 - Trimming of timing current"]
    #[inline(always)]
    pub fn itim_hv(&self) -> ITIM_HV_R {
        ITIM_HV_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - 0': vdd<2.3V '1': vdd>=2.3V"]
    #[inline(always)]
    pub fn vddhi_hv(&self) -> VDDHI_HV_R {
        VDDHI_HV_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Turbo pulse width trim"]
    #[inline(always)]
    pub fn turbo_pulsew_hv(&self) -> TURBO_PULSEW_HV_R {
        TURBO_PULSEW_HV_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - LO Bandgap Enable"]
    #[inline(always)]
    pub fn bglo_en_hv(&self) -> BGLO_EN_HV_R {
        BGLO_EN_HV_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - HI Bandgap Enable"]
    #[inline(always)]
    pub fn bghi_en_hv(&self) -> BGHI_EN_HV_R {
        BGHI_EN_HV_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Flash macro pump clock trim control."]
    #[inline(always)]
    pub fn osc_trim_hv(&mut self) -> OSC_TRIM_HV_W {
        OSC_TRIM_HV_W { w: self }
    }
    #[doc = "Bit 4 - 0: Oscillator High Frequency Range 1: Oscillator Low Frequency range"]
    #[inline(always)]
    pub fn osc_range_trim_hv(&mut self) -> OSC_RANGE_TRIM_HV_W {
        OSC_RANGE_TRIM_HV_W { w: self }
    }
    #[doc = "Bits 5:8 - N/A"]
    #[inline(always)]
    pub fn idac_hv(&mut self) -> IDAC_HV_W {
        IDAC_HV_W { w: self }
    }
    #[doc = "Bits 9:10 - N/A"]
    #[inline(always)]
    pub fn sdac_hv(&mut self) -> SDAC_HV_W {
        SDAC_HV_W { w: self }
    }
    #[doc = "Bits 11:14 - Trimming of timing current"]
    #[inline(always)]
    pub fn itim_hv(&mut self) -> ITIM_HV_W {
        ITIM_HV_W { w: self }
    }
    #[doc = "Bit 15 - 0': vdd<2.3V '1': vdd>=2.3V"]
    #[inline(always)]
    pub fn vddhi_hv(&mut self) -> VDDHI_HV_W {
        VDDHI_HV_W { w: self }
    }
    #[doc = "Bits 16:17 - Turbo pulse width trim"]
    #[inline(always)]
    pub fn turbo_pulsew_hv(&mut self) -> TURBO_PULSEW_HV_W {
        TURBO_PULSEW_HV_W { w: self }
    }
    #[doc = "Bit 18 - LO Bandgap Enable"]
    #[inline(always)]
    pub fn bglo_en_hv(&mut self) -> BGLO_EN_HV_W {
        BGLO_EN_HV_W { w: self }
    }
    #[doc = "Bit 19 - HI Bandgap Enable"]
    #[inline(always)]
    pub fn bghi_en_hv(&mut self) -> BGHI_EN_HV_W {
        BGHI_EN_HV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cal control osc trim bits, idac, sdac, itim, bdac.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal_ctl3](index.html) module"]
pub struct CAL_CTL3_SPEC;
impl crate::RegisterSpec for CAL_CTL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cal_ctl3::R](R) reader structure"]
impl crate::Readable for CAL_CTL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cal_ctl3::W](W) writer structure"]
impl crate::Writable for CAL_CTL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAL_CTL3 to value 0xa504"]
impl crate::Resettable for CAL_CTL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa504
    }
}
