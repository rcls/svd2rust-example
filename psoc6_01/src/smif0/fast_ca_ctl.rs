#[doc = "Register `FAST_CA_CTL` reader"]
pub struct R(crate::R<FAST_CA_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FAST_CA_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FAST_CA_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FAST_CA_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FAST_CA_CTL` writer"]
pub struct W(crate::W<FAST_CA_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FAST_CA_CTL_SPEC>;
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
impl From<crate::W<FAST_CA_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FAST_CA_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAY` reader - See SLOW_CA_CTL.WAY."]
pub struct WAY_R(crate::FieldReader<u8, u8>);
impl WAY_R {
    pub(crate) fn new(bits: u8) -> Self {
        WAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAY` writer - See SLOW_CA_CTL.WAY."]
pub struct WAY_W<'a> {
    w: &'a mut W,
}
impl<'a> WAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `SET_ADDR` reader - See SLOW_CA_CTL.SET_ADDR."]
pub struct SET_ADDR_R(crate::FieldReader<u8, u8>);
impl SET_ADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        SET_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SET_ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SET_ADDR` writer - See SLOW_CA_CTL.SET_ADDR."]
pub struct SET_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `PREF_EN` reader - See SLOW_CA_CTL.PREF_EN."]
pub struct PREF_EN_R(crate::FieldReader<bool, bool>);
impl PREF_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PREF_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREF_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREF_EN` writer - See SLOW_CA_CTL.PREF_EN."]
pub struct PREF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PREF_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `ENABLED` reader - See SLOW_CA_CTL.ENABLED."]
pub struct ENABLED_R(crate::FieldReader<bool, bool>);
impl ENABLED_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLED` writer - See SLOW_CA_CTL.ENABLED."]
pub struct ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:17 - See SLOW_CA_CTL.WAY."]
    #[inline(always)]
    pub fn way(&self) -> WAY_R {
        WAY_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - See SLOW_CA_CTL.SET_ADDR."]
    #[inline(always)]
    pub fn set_addr(&self) -> SET_ADDR_R {
        SET_ADDR_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 30 - See SLOW_CA_CTL.PREF_EN."]
    #[inline(always)]
    pub fn pref_en(&self) -> PREF_EN_R {
        PREF_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - See SLOW_CA_CTL.ENABLED."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - See SLOW_CA_CTL.WAY."]
    #[inline(always)]
    pub fn way(&mut self) -> WAY_W {
        WAY_W { w: self }
    }
    #[doc = "Bits 24:25 - See SLOW_CA_CTL.SET_ADDR."]
    #[inline(always)]
    pub fn set_addr(&mut self) -> SET_ADDR_W {
        SET_ADDR_W { w: self }
    }
    #[doc = "Bit 30 - See SLOW_CA_CTL.PREF_EN."]
    #[inline(always)]
    pub fn pref_en(&mut self) -> PREF_EN_W {
        PREF_EN_W { w: self }
    }
    #[doc = "Bit 31 - See SLOW_CA_CTL.ENABLED."]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W {
        ENABLED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fast cache control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fast_ca_ctl](index.html) module"]
pub struct FAST_CA_CTL_SPEC;
impl crate::RegisterSpec for FAST_CA_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fast_ca_ctl::R](R) reader structure"]
impl crate::Readable for FAST_CA_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fast_ca_ctl::W](W) writer structure"]
impl crate::Writable for FAST_CA_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FAST_CA_CTL to value 0xc000_0000"]
impl crate::Resettable for FAST_CA_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc000_0000
    }
}
