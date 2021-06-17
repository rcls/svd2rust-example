#[doc = "Register `SW_FW_MOD_SEL` reader"]
pub struct R(crate::R<SW_FW_MOD_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SW_FW_MOD_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SW_FW_MOD_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SW_FW_MOD_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SW_FW_MOD_SEL` writer"]
pub struct W(crate::W<SW_FW_MOD_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SW_FW_MOD_SEL_SPEC>;
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
impl From<crate::W<SW_FW_MOD_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SW_FW_MOD_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_F1PM` reader - Set corresponding switch"]
pub struct SW_F1PM_R(crate::FieldReader<bool, bool>);
impl SW_F1PM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_F1PM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_F1PM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_F1PM` writer - Set corresponding switch"]
pub struct SW_F1PM_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_F1PM_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `SW_F1MA` reader - Select waveform for corresponding switch"]
pub struct SW_F1MA_R(crate::FieldReader<u8, u8>);
impl SW_F1MA_R {
    pub(crate) fn new(bits: u8) -> Self {
        SW_F1MA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_F1MA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_F1MA` writer - Select waveform for corresponding switch"]
pub struct SW_F1MA_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_F1MA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `SW_F1CA` reader - Select waveform for corresponding switch"]
pub struct SW_F1CA_R(crate::FieldReader<u8, u8>);
impl SW_F1CA_R {
    pub(crate) fn new(bits: u8) -> Self {
        SW_F1CA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_F1CA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_F1CA` writer - Select waveform for corresponding switch"]
pub struct SW_F1CA_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_F1CA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `SW_C1CC` reader - Set corresponding switch"]
pub struct SW_C1CC_R(crate::FieldReader<bool, bool>);
impl SW_C1CC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_C1CC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_C1CC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_C1CC` writer - Set corresponding switch"]
pub struct SW_C1CC_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_C1CC_W<'a> {
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
#[doc = "Field `SW_C1CD` reader - Set corresponding switch"]
pub struct SW_C1CD_R(crate::FieldReader<bool, bool>);
impl SW_C1CD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_C1CD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_C1CD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_C1CD` writer - Set corresponding switch"]
pub struct SW_C1CD_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_C1CD_W<'a> {
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
#[doc = "Field `SW_C1F1` reader - Set corresponding switch"]
pub struct SW_C1F1_R(crate::FieldReader<bool, bool>);
impl SW_C1F1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_C1F1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_C1F1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_C1F1` writer - Set corresponding switch"]
pub struct SW_C1F1_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_C1F1_W<'a> {
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
    #[doc = "Bit 0 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_f1pm(&self) -> SW_F1PM_R {
        SW_F1PM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f1ma(&self) -> SW_F1MA_R {
        SW_F1MA_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f1ca(&self) -> SW_F1CA_R {
        SW_F1CA_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c1cc(&self) -> SW_C1CC_R {
        SW_C1CC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c1cd(&self) -> SW_C1CD_R {
        SW_C1CD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c1f1(&self) -> SW_C1F1_R {
        SW_C1F1_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_f1pm(&mut self) -> SW_F1PM_W {
        SW_F1PM_W { w: self }
    }
    #[doc = "Bits 8:10 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f1ma(&mut self) -> SW_F1MA_W {
        SW_F1MA_W { w: self }
    }
    #[doc = "Bits 16:18 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f1ca(&mut self) -> SW_F1CA_W {
        SW_F1CA_W { w: self }
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c1cc(&mut self) -> SW_C1CC_W {
        SW_C1CC_W { w: self }
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c1cd(&mut self) -> SW_C1CD_W {
        SW_C1CD_W { w: self }
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c1f1(&mut self) -> SW_C1F1_W {
        SW_C1F1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Full Wave Cmod Switch Waveform selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_fw_mod_sel](index.html) module"]
pub struct SW_FW_MOD_SEL_SPEC;
impl crate::RegisterSpec for SW_FW_MOD_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sw_fw_mod_sel::R](R) reader structure"]
impl crate::Readable for SW_FW_MOD_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sw_fw_mod_sel::W](W) writer structure"]
impl crate::Writable for SW_FW_MOD_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SW_FW_MOD_SEL to value 0"]
impl crate::Resettable for SW_FW_MOD_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
