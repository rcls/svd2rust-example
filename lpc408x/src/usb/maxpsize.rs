#[doc = "Register `MAXPSIZE` reader"]
pub struct R(crate::R<MAXPSIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAXPSIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAXPSIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAXPSIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAXPSIZE` writer"]
pub struct W(crate::W<MAXPSIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAXPSIZE_SPEC>;
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
impl From<crate::W<MAXPSIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAXPSIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPS` reader - The maximum packet size value."]
pub struct MPS_R(crate::FieldReader<u16, u16>);
impl MPS_R {
    pub(crate) fn new(bits: u16) -> Self {
        MPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPS` writer - The maximum packet size value."]
pub struct MPS_W<'a> {
    w: &'a mut W,
}
impl<'a> MPS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - The maximum packet size value."]
    #[inline(always)]
    pub fn mps(&self) -> MPS_R {
        MPS_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - The maximum packet size value."]
    #[inline(always)]
    pub fn mps(&mut self) -> MPS_W {
        MPS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB MaxPacketSize\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maxpsize](index.html) module"]
pub struct MAXPSIZE_SPEC;
impl crate::RegisterSpec for MAXPSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maxpsize::R](R) reader structure"]
impl crate::Readable for MAXPSIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maxpsize::W](W) writer structure"]
impl crate::Writable for MAXPSIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAXPSIZE to value 0x08"]
impl crate::Resettable for MAXPSIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
