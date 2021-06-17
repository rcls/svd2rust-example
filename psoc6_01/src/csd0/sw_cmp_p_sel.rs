#[doc = "Register `SW_CMP_P_SEL` reader"]
pub struct R(crate::R<SW_CMP_P_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SW_CMP_P_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SW_CMP_P_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SW_CMP_P_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SW_CMP_P_SEL` writer"]
pub struct W(crate::W<SW_CMP_P_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SW_CMP_P_SEL_SPEC>;
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
impl From<crate::W<SW_CMP_P_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SW_CMP_P_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_SFPM` reader - Select waveform for corresponding switch"]
pub struct SW_SFPM_R(crate::FieldReader<u8, u8>);
impl SW_SFPM_R {
    pub(crate) fn new(bits: u8) -> Self {
        SW_SFPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_SFPM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_SFPM` writer - Select waveform for corresponding switch"]
pub struct SW_SFPM_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_SFPM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `SW_SFPT` reader - Select waveform for corresponding switch"]
pub struct SW_SFPT_R(crate::FieldReader<u8, u8>);
impl SW_SFPT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SW_SFPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_SFPT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_SFPT` writer - Select waveform for corresponding switch"]
pub struct SW_SFPT_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_SFPT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `SW_SFPS` reader - Select waveform for corresponding switch"]
pub struct SW_SFPS_R(crate::FieldReader<u8, u8>);
impl SW_SFPS_R {
    pub(crate) fn new(bits: u8) -> Self {
        SW_SFPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_SFPS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_SFPS` writer - Select waveform for corresponding switch"]
pub struct SW_SFPS_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_SFPS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `SW_SFMA` reader - Set corresponding switch"]
pub struct SW_SFMA_R(crate::FieldReader<bool, bool>);
impl SW_SFMA_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_SFMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_SFMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_SFMA` writer - Set corresponding switch"]
pub struct SW_SFMA_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_SFMA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `SW_SFMB` reader - Set corresponding switch"]
pub struct SW_SFMB_R(crate::FieldReader<bool, bool>);
impl SW_SFMB_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_SFMB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_SFMB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_SFMB` writer - Set corresponding switch"]
pub struct SW_SFMB_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_SFMB_W<'a> {
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
#[doc = "Field `SW_SFCA` reader - Set corresponding switch"]
pub struct SW_SFCA_R(crate::FieldReader<bool, bool>);
impl SW_SFCA_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_SFCA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_SFCA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_SFCA` writer - Set corresponding switch"]
pub struct SW_SFCA_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_SFCA_W<'a> {
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
#[doc = "Field `SW_SFCB` reader - Set corresponding switch"]
pub struct SW_SFCB_R(crate::FieldReader<bool, bool>);
impl SW_SFCB_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_SFCB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_SFCB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_SFCB` writer - Set corresponding switch"]
pub struct SW_SFCB_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_SFCB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_sfpm(&self) -> SW_SFPM_R {
        SW_SFPM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_sfpt(&self) -> SW_SFPT_R {
        SW_SFPT_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_sfps(&self) -> SW_SFPS_R {
        SW_SFPS_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfma(&self) -> SW_SFMA_R {
        SW_SFMA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfmb(&self) -> SW_SFMB_R {
        SW_SFMB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfca(&self) -> SW_SFCA_R {
        SW_SFCA_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfcb(&self) -> SW_SFCB_R {
        SW_SFCB_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_sfpm(&mut self) -> SW_SFPM_W {
        SW_SFPM_W { w: self }
    }
    #[doc = "Bits 4:6 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_sfpt(&mut self) -> SW_SFPT_W {
        SW_SFPT_W { w: self }
    }
    #[doc = "Bits 8:10 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_sfps(&mut self) -> SW_SFPS_W {
        SW_SFPS_W { w: self }
    }
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfma(&mut self) -> SW_SFMA_W {
        SW_SFMA_W { w: self }
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfmb(&mut self) -> SW_SFMB_W {
        SW_SFMB_W { w: self }
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfca(&mut self) -> SW_SFCA_W {
        SW_SFCA_W { w: self }
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfcb(&mut self) -> SW_SFCB_W {
        SW_SFCB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSDCMP Pos Switch Waveform selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_cmp_p_sel](index.html) module"]
pub struct SW_CMP_P_SEL_SPEC;
impl crate::RegisterSpec for SW_CMP_P_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sw_cmp_p_sel::R](R) reader structure"]
impl crate::Readable for SW_CMP_P_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sw_cmp_p_sel::W](W) writer structure"]
impl crate::Writable for SW_CMP_P_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SW_CMP_P_SEL to value 0"]
impl crate::Resettable for SW_CMP_P_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
