#[doc = "Register `SW_AMUXBUF_SEL` reader"]
pub struct R(crate::R<SW_AMUXBUF_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SW_AMUXBUF_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SW_AMUXBUF_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SW_AMUXBUF_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SW_AMUXBUF_SEL` writer"]
pub struct W(crate::W<SW_AMUXBUF_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SW_AMUXBUF_SEL_SPEC>;
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
impl From<crate::W<SW_AMUXBUF_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SW_AMUXBUF_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_IRBY` reader - Set corresponding switch"]
pub struct SW_IRBY_R(crate::FieldReader<bool, bool>);
impl SW_IRBY_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_IRBY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_IRBY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_IRBY` writer - Set corresponding switch"]
pub struct SW_IRBY_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_IRBY_W<'a> {
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
#[doc = "Field `SW_IRLB` reader - Set corresponding switch"]
pub struct SW_IRLB_R(crate::FieldReader<bool, bool>);
impl SW_IRLB_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_IRLB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_IRLB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_IRLB` writer - Set corresponding switch"]
pub struct SW_IRLB_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_IRLB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `SW_ICA` reader - Set corresponding switch"]
pub struct SW_ICA_R(crate::FieldReader<bool, bool>);
impl SW_ICA_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_ICA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_ICA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_ICA` writer - Set corresponding switch"]
pub struct SW_ICA_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_ICA_W<'a> {
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
#[doc = "Field `SW_ICB` reader - Select waveform for corresponding switch"]
pub struct SW_ICB_R(crate::FieldReader<u8, u8>);
impl SW_ICB_R {
    pub(crate) fn new(bits: u8) -> Self {
        SW_ICB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_ICB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_ICB` writer - Select waveform for corresponding switch"]
pub struct SW_ICB_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_ICB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `SW_IRLI` reader - Set corresponding switch"]
pub struct SW_IRLI_R(crate::FieldReader<bool, bool>);
impl SW_IRLI_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_IRLI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_IRLI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_IRLI` writer - Set corresponding switch"]
pub struct SW_IRLI_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_IRLI_W<'a> {
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
#[doc = "Field `SW_IRH` reader - Set corresponding switch"]
pub struct SW_IRH_R(crate::FieldReader<bool, bool>);
impl SW_IRH_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_IRH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_IRH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_IRH` writer - Set corresponding switch"]
pub struct SW_IRH_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_IRH_W<'a> {
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
#[doc = "Field `SW_IRL` reader - Set corresponding switch"]
pub struct SW_IRL_R(crate::FieldReader<bool, bool>);
impl SW_IRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_IRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_IRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_IRL` writer - Set corresponding switch"]
pub struct SW_IRL_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_IRL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irby(&self) -> SW_IRBY_R {
        SW_IRBY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irlb(&self) -> SW_IRLB_R {
        SW_IRLB_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_ica(&self) -> SW_ICA_R {
        SW_ICA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_icb(&self) -> SW_ICB_R {
        SW_ICB_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irli(&self) -> SW_IRLI_R {
        SW_IRLI_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irh(&self) -> SW_IRH_R {
        SW_IRH_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irl(&self) -> SW_IRL_R {
        SW_IRL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irby(&mut self) -> SW_IRBY_W {
        SW_IRBY_W { w: self }
    }
    #[doc = "Bit 8 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irlb(&mut self) -> SW_IRLB_W {
        SW_IRLB_W { w: self }
    }
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_ica(&mut self) -> SW_ICA_W {
        SW_ICA_W { w: self }
    }
    #[doc = "Bits 16:18 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_icb(&mut self) -> SW_ICB_W {
        SW_ICB_W { w: self }
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irli(&mut self) -> SW_IRLI_W {
        SW_IRLI_W { w: self }
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irh(&mut self) -> SW_IRH_W {
        SW_IRH_W { w: self }
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irl(&mut self) -> SW_IRL_W {
        SW_IRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Amuxbuffer switches Waveform selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_amuxbuf_sel](index.html) module"]
pub struct SW_AMUXBUF_SEL_SPEC;
impl crate::RegisterSpec for SW_AMUXBUF_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sw_amuxbuf_sel::R](R) reader structure"]
impl crate::Readable for SW_AMUXBUF_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sw_amuxbuf_sel::W](W) writer structure"]
impl crate::Writable for SW_AMUXBUF_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SW_AMUXBUF_SEL to value 0"]
impl crate::Resettable for SW_AMUXBUF_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
