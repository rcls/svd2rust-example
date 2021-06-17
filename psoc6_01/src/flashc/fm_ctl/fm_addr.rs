#[doc = "Register `FM_ADDR` reader"]
pub struct R(crate::R<FM_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FM_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FM_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FM_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FM_ADDR` writer"]
pub struct W(crate::W<FM_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FM_ADDR_SPEC>;
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
impl From<crate::W<FM_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FM_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RA` reader - Row address."]
pub struct RA_R(crate::FieldReader<u16, u16>);
impl RA_R {
    pub(crate) fn new(bits: u16) -> Self {
        RA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RA` writer - Row address."]
pub struct RA_W<'a> {
    w: &'a mut W,
}
impl<'a> RA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `BA` reader - Bank address."]
pub struct BA_R(crate::FieldReader<u8, u8>);
impl BA_R {
    pub(crate) fn new(bits: u8) -> Self {
        BA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BA` writer - Bank address."]
pub struct BA_W<'a> {
    w: &'a mut W,
}
impl<'a> BA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `AXA` reader - Auxiliary address field: '0': regular flash memory. '1': supervisory flash memory."]
pub struct AXA_R(crate::FieldReader<bool, bool>);
impl AXA_R {
    pub(crate) fn new(bits: bool) -> Self {
        AXA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AXA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AXA` writer - Auxiliary address field: '0': regular flash memory. '1': supervisory flash memory."]
pub struct AXA_W<'a> {
    w: &'a mut W,
}
impl<'a> AXA_W<'a> {
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
    #[doc = "Bits 0:15 - Row address."]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Bank address."]
    #[inline(always)]
    pub fn ba(&self) -> BA_R {
        BA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Auxiliary address field: '0': regular flash memory. '1': supervisory flash memory."]
    #[inline(always)]
    pub fn axa(&self) -> AXA_R {
        AXA_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Row address."]
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W {
        RA_W { w: self }
    }
    #[doc = "Bits 16:23 - Bank address."]
    #[inline(always)]
    pub fn ba(&mut self) -> BA_W {
        BA_W { w: self }
    }
    #[doc = "Bit 24 - Auxiliary address field: '0': regular flash memory. '1': supervisory flash memory."]
    #[inline(always)]
    pub fn axa(&mut self) -> AXA_W {
        AXA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash macro address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fm_addr](index.html) module"]
pub struct FM_ADDR_SPEC;
impl crate::RegisterSpec for FM_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fm_addr::R](R) reader structure"]
impl crate::Readable for FM_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fm_addr::W](W) writer structure"]
impl crate::Writable for FM_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FM_ADDR to value 0"]
impl crate::Resettable for FM_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
