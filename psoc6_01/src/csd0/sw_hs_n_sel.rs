#[doc = "Register `SW_HS_N_SEL` reader"]
pub struct R(crate::R<SW_HS_N_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SW_HS_N_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SW_HS_N_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SW_HS_N_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SW_HS_N_SEL` writer"]
pub struct W(crate::W<SW_HS_N_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SW_HS_N_SEL_SPEC>;
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
impl From<crate::W<SW_HS_N_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SW_HS_N_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_HCCC` reader - Set corresponding switch"]
pub struct SW_HCCC_R(crate::FieldReader<bool, bool>);
impl SW_HCCC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_HCCC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_HCCC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_HCCC` writer - Set corresponding switch"]
pub struct SW_HCCC_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_HCCC_W<'a> {
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
#[doc = "Field `SW_HCCD` reader - Set corresponding switch"]
pub struct SW_HCCD_R(crate::FieldReader<bool, bool>);
impl SW_HCCD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_HCCD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_HCCD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_HCCD` writer - Set corresponding switch"]
pub struct SW_HCCD_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_HCCD_W<'a> {
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
#[doc = "Field `SW_HCRH` reader - Select waveform for corresponding switch"]
pub struct SW_HCRH_R(crate::FieldReader<u8, u8>);
impl SW_HCRH_R {
    pub(crate) fn new(bits: u8) -> Self {
        SW_HCRH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_HCRH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_HCRH` writer - Select waveform for corresponding switch"]
pub struct SW_HCRH_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_HCRH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `SW_HCRL` reader - Select waveform for corresponding switch"]
pub struct SW_HCRL_R(crate::FieldReader<u8, u8>);
impl SW_HCRL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SW_HCRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_HCRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_HCRL` writer - Select waveform for corresponding switch"]
pub struct SW_HCRL_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_HCRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hccc(&self) -> SW_HCCC_R {
        SW_HCCC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hccd(&self) -> SW_HCCD_R {
        SW_HCCD_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_hcrh(&self) -> SW_HCRH_R {
        SW_HCRH_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:30 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_hcrl(&self) -> SW_HCRL_R {
        SW_HCRL_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hccc(&mut self) -> SW_HCCC_W {
        SW_HCCC_W { w: self }
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hccd(&mut self) -> SW_HCCD_W {
        SW_HCCD_W { w: self }
    }
    #[doc = "Bits 24:26 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_hcrh(&mut self) -> SW_HCRH_W {
        SW_HCRH_W { w: self }
    }
    #[doc = "Bits 28:30 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_hcrl(&mut self) -> SW_HCRL_W {
        SW_HCRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HSCMP Neg input switch Waveform selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_hs_n_sel](index.html) module"]
pub struct SW_HS_N_SEL_SPEC;
impl crate::RegisterSpec for SW_HS_N_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sw_hs_n_sel::R](R) reader structure"]
impl crate::Readable for SW_HS_N_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sw_hs_n_sel::W](W) writer structure"]
impl crate::Writable for SW_HS_N_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SW_HS_N_SEL to value 0"]
impl crate::Resettable for SW_HS_N_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
