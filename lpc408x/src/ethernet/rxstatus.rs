#[doc = "Register `RXSTATUS` reader"]
pub struct R(crate::R<RXSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXSTATUS` writer"]
pub struct W(crate::W<RXSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXSTATUS_SPEC>;
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
impl From<crate::W<RXSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXSTATUS` reader - MSBs of receive status base address."]
pub struct RXSTATUS_R(crate::FieldReader<u32, u32>);
impl RXSTATUS_R {
    pub(crate) fn new(bits: u32) -> Self {
        RXSTATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXSTATUS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXSTATUS` writer - MSBs of receive status base address."]
pub struct RXSTATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | ((value as u32 & 0x1fff_ffff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:31 - MSBs of receive status base address."]
    #[inline(always)]
    pub fn rxstatus(&self) -> RXSTATUS_R {
        RXSTATUS_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 3:31 - MSBs of receive status base address."]
    #[inline(always)]
    pub fn rxstatus(&mut self) -> RXSTATUS_W {
        RXSTATUS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive status base address register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxstatus](index.html) module"]
pub struct RXSTATUS_SPEC;
impl crate::RegisterSpec for RXSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxstatus::R](R) reader structure"]
impl crate::Readable for RXSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxstatus::W](W) writer structure"]
impl crate::Writable for RXSTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXSTATUS to value 0"]
impl crate::Resettable for RXSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
