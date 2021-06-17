#[doc = "Register `SW_SHIELD_SEL` reader"]
pub struct R(crate::R<SW_SHIELD_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SW_SHIELD_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SW_SHIELD_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SW_SHIELD_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SW_SHIELD_SEL` writer"]
pub struct W(crate::W<SW_SHIELD_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SW_SHIELD_SEL_SPEC>;
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
impl From<crate::W<SW_SHIELD_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SW_SHIELD_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_HCAV` reader - N/A"]
pub struct SW_HCAV_R(crate::FieldReader<u8, u8>);
impl SW_HCAV_R {
    pub(crate) fn new(bits: u8) -> Self {
        SW_HCAV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_HCAV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_HCAV` writer - N/A"]
pub struct SW_HCAV_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_HCAV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `SW_HCAG` reader - Select waveform for corresponding switch"]
pub struct SW_HCAG_R(crate::FieldReader<u8, u8>);
impl SW_HCAG_R {
    pub(crate) fn new(bits: u8) -> Self {
        SW_HCAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_HCAG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_HCAG` writer - Select waveform for corresponding switch"]
pub struct SW_HCAG_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_HCAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `SW_HCBV` reader - N/A"]
pub struct SW_HCBV_R(crate::FieldReader<u8, u8>);
impl SW_HCBV_R {
    pub(crate) fn new(bits: u8) -> Self {
        SW_HCBV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_HCBV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_HCBV` writer - N/A"]
pub struct SW_HCBV_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_HCBV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `SW_HCBG` reader - Select waveform for corresponding switch, using csd_shield as base"]
pub struct SW_HCBG_R(crate::FieldReader<u8, u8>);
impl SW_HCBG_R {
    pub(crate) fn new(bits: u8) -> Self {
        SW_HCBG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_HCBG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_HCBG` writer - Select waveform for corresponding switch, using csd_shield as base"]
pub struct SW_HCBG_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_HCBG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `SW_HCCV` reader - Set corresponding switch"]
pub struct SW_HCCV_R(crate::FieldReader<bool, bool>);
impl SW_HCCV_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_HCCV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_HCCV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_HCCV` writer - Set corresponding switch"]
pub struct SW_HCCV_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_HCCV_W<'a> {
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
#[doc = "Field `SW_HCCG` reader - Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
pub struct SW_HCCG_R(crate::FieldReader<bool, bool>);
impl SW_HCCG_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_HCCG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_HCCG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_HCCG` writer - Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
pub struct SW_HCCG_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_HCCG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - N/A"]
    #[inline(always)]
    pub fn sw_hcav(&self) -> SW_HCAV_R {
        SW_HCAV_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_hcag(&self) -> SW_HCAG_R {
        SW_HCAG_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - N/A"]
    #[inline(always)]
    pub fn sw_hcbv(&self) -> SW_HCBV_R {
        SW_HCBV_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Select waveform for corresponding switch, using csd_shield as base"]
    #[inline(always)]
    pub fn sw_hcbg(&self) -> SW_HCBG_R {
        SW_HCBG_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hccv(&self) -> SW_HCCV_R {
        SW_HCCV_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
    #[inline(always)]
    pub fn sw_hccg(&self) -> SW_HCCG_R {
        SW_HCCG_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - N/A"]
    #[inline(always)]
    pub fn sw_hcav(&mut self) -> SW_HCAV_W {
        SW_HCAV_W { w: self }
    }
    #[doc = "Bits 4:6 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_hcag(&mut self) -> SW_HCAG_W {
        SW_HCAG_W { w: self }
    }
    #[doc = "Bits 8:10 - N/A"]
    #[inline(always)]
    pub fn sw_hcbv(&mut self) -> SW_HCBV_W {
        SW_HCBV_W { w: self }
    }
    #[doc = "Bits 12:14 - Select waveform for corresponding switch, using csd_shield as base"]
    #[inline(always)]
    pub fn sw_hcbg(&mut self) -> SW_HCBG_W {
        SW_HCBG_W { w: self }
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hccv(&mut self) -> SW_HCCV_W {
        SW_HCCV_W { w: self }
    }
    #[doc = "Bit 20 - Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
    #[inline(always)]
    pub fn sw_hccg(&mut self) -> SW_HCCG_W {
        SW_HCCG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shielding switches Waveform selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_shield_sel](index.html) module"]
pub struct SW_SHIELD_SEL_SPEC;
impl crate::RegisterSpec for SW_SHIELD_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sw_shield_sel::R](R) reader structure"]
impl crate::Readable for SW_SHIELD_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sw_shield_sel::W](W) writer structure"]
impl crate::Writable for SW_SHIELD_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SW_SHIELD_SEL to value 0"]
impl crate::Resettable for SW_SHIELD_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
