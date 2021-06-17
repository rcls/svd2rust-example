#[doc = "Register `CWA` reader"]
pub struct R(crate::R<CWA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CWA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CWA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CWA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CWA` writer"]
pub struct W(crate::W<CWA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CWA_SPEC>;
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
impl From<crate::W<CWA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CWA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CWA` reader - Write Address for Common Area"]
pub struct CWA_R(crate::FieldReader<u8, u8>);
impl CWA_R {
    pub(crate) fn new(bits: u8) -> Self {
        CWA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CWA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CWA` writer - Write Address for Common Area"]
pub struct CWA_W<'a> {
    w: &'a mut W,
}
impl<'a> CWA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Write Address for Common Area"]
    #[inline(always)]
    pub fn cwa(&self) -> CWA_R {
        CWA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Write Address for Common Area"]
    #[inline(always)]
    pub fn cwa(&mut self) -> CWA_W {
        CWA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common Area Write Address *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cwa](index.html) module"]
pub struct CWA_SPEC;
impl crate::RegisterSpec for CWA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cwa::R](R) reader structure"]
impl crate::Readable for CWA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cwa::W](W) writer structure"]
impl crate::Writable for CWA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CWA to value 0"]
impl crate::Resettable for CWA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
