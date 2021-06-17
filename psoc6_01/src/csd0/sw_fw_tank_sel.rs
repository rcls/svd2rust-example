#[doc = "Register `SW_FW_TANK_SEL` reader"]
pub struct R(crate::R<SW_FW_TANK_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SW_FW_TANK_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SW_FW_TANK_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SW_FW_TANK_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SW_FW_TANK_SEL` writer"]
pub struct W(crate::W<SW_FW_TANK_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SW_FW_TANK_SEL_SPEC>;
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
impl From<crate::W<SW_FW_TANK_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SW_FW_TANK_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_F2PT` reader - Set corresponding switch"]
pub struct SW_F2PT_R(crate::FieldReader<bool, bool>);
impl SW_F2PT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_F2PT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_F2PT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_F2PT` writer - Set corresponding switch"]
pub struct SW_F2PT_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_F2PT_W<'a> {
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
#[doc = "Field `SW_F2MA` reader - Select waveform for corresponding switch"]
pub struct SW_F2MA_R(crate::FieldReader<u8, u8>);
impl SW_F2MA_R {
    pub(crate) fn new(bits: u8) -> Self {
        SW_F2MA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_F2MA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_F2MA` writer - Select waveform for corresponding switch"]
pub struct SW_F2MA_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_F2MA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `SW_F2CA` reader - Select waveform for corresponding switch"]
pub struct SW_F2CA_R(crate::FieldReader<u8, u8>);
impl SW_F2CA_R {
    pub(crate) fn new(bits: u8) -> Self {
        SW_F2CA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_F2CA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_F2CA` writer - Select waveform for corresponding switch"]
pub struct SW_F2CA_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_F2CA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `SW_F2CB` reader - Select waveform for corresponding switch"]
pub struct SW_F2CB_R(crate::FieldReader<u8, u8>);
impl SW_F2CB_R {
    pub(crate) fn new(bits: u8) -> Self {
        SW_F2CB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_F2CB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_F2CB` writer - Select waveform for corresponding switch"]
pub struct SW_F2CB_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_F2CB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `SW_C2CC` reader - Set corresponding switch"]
pub struct SW_C2CC_R(crate::FieldReader<bool, bool>);
impl SW_C2CC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_C2CC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_C2CC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_C2CC` writer - Set corresponding switch"]
pub struct SW_C2CC_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_C2CC_W<'a> {
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
#[doc = "Field `SW_C2CD` reader - Set corresponding switch"]
pub struct SW_C2CD_R(crate::FieldReader<bool, bool>);
impl SW_C2CD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_C2CD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_C2CD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_C2CD` writer - Set corresponding switch"]
pub struct SW_C2CD_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_C2CD_W<'a> {
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
#[doc = "Field `SW_C2F2` reader - Set corresponding switch"]
pub struct SW_C2F2_R(crate::FieldReader<bool, bool>);
impl SW_C2F2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_C2F2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_C2F2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_C2F2` writer - Set corresponding switch"]
pub struct SW_C2F2_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_C2F2_W<'a> {
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
    pub fn sw_f2pt(&self) -> SW_F2PT_R {
        SW_F2PT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f2ma(&self) -> SW_F2MA_R {
        SW_F2MA_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f2ca(&self) -> SW_F2CA_R {
        SW_F2CA_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f2cb(&self) -> SW_F2CB_R {
        SW_F2CB_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c2cc(&self) -> SW_C2CC_R {
        SW_C2CC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c2cd(&self) -> SW_C2CD_R {
        SW_C2CD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c2f2(&self) -> SW_C2F2_R {
        SW_C2F2_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_f2pt(&mut self) -> SW_F2PT_W {
        SW_F2PT_W { w: self }
    }
    #[doc = "Bits 8:10 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f2ma(&mut self) -> SW_F2MA_W {
        SW_F2MA_W { w: self }
    }
    #[doc = "Bits 12:14 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f2ca(&mut self) -> SW_F2CA_W {
        SW_F2CA_W { w: self }
    }
    #[doc = "Bits 16:18 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f2cb(&mut self) -> SW_F2CB_W {
        SW_F2CB_W { w: self }
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c2cc(&mut self) -> SW_C2CC_W {
        SW_C2CC_W { w: self }
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c2cd(&mut self) -> SW_C2CD_W {
        SW_C2CD_W { w: self }
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c2f2(&mut self) -> SW_C2F2_W {
        SW_C2F2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Full Wave Csh_tank Switch Waveform selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_fw_tank_sel](index.html) module"]
pub struct SW_FW_TANK_SEL_SPEC;
impl crate::RegisterSpec for SW_FW_TANK_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sw_fw_tank_sel::R](R) reader structure"]
impl crate::Readable for SW_FW_TANK_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sw_fw_tank_sel::W](W) writer structure"]
impl crate::Writable for SW_FW_TANK_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SW_FW_TANK_SEL to value 0"]
impl crate::Resettable for SW_FW_TANK_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
