#[doc = "Register `UDCAH` reader"]
pub struct R(crate::R<UDCAH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UDCAH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UDCAH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UDCAH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UDCAH` writer"]
pub struct W(crate::W<UDCAH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UDCAH_SPEC>;
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
impl From<crate::W<UDCAH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UDCAH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UDCA_ADDR` reader - Start address of the UDCA."]
pub struct UDCA_ADDR_R(crate::FieldReader<u32, u32>);
impl UDCA_ADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        UDCA_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UDCA_ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UDCA_ADDR` writer - Start address of the UDCA."]
pub struct UDCA_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> UDCA_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff_ffff << 7)) | ((value as u32 & 0x01ff_ffff) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 7:31 - Start address of the UDCA."]
    #[inline(always)]
    pub fn udca_addr(&self) -> UDCA_ADDR_R {
        UDCA_ADDR_R::new(((self.bits >> 7) & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 7:31 - Start address of the UDCA."]
    #[inline(always)]
    pub fn udca_addr(&mut self) -> UDCA_ADDR_W {
        UDCA_ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB UDCA Head\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udcah](index.html) module"]
pub struct UDCAH_SPEC;
impl crate::RegisterSpec for UDCAH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [udcah::R](R) reader structure"]
impl crate::Readable for UDCAH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [udcah::W](W) writer structure"]
impl crate::Writable for UDCAH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UDCAH to value 0"]
impl crate::Resettable for UDCAH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
