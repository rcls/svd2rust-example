#[doc = "Register `TXSTATUS` reader"]
pub struct R(crate::R<TXSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXSTATUS` writer"]
pub struct W(crate::W<TXSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXSTATUS_SPEC>;
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
impl From<crate::W<TXSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXSTAT` reader - TxStatus. MSBs of transmit status base address."]
pub struct TXSTAT_R(crate::FieldReader<u32, u32>);
impl TXSTAT_R {
    pub(crate) fn new(bits: u32) -> Self {
        TXSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXSTAT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXSTAT` writer - TxStatus. MSBs of transmit status base address."]
pub struct TXSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSTAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | ((value as u32 & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - TxStatus. MSBs of transmit status base address."]
    #[inline(always)]
    pub fn txstat(&self) -> TXSTAT_R {
        TXSTAT_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - TxStatus. MSBs of transmit status base address."]
    #[inline(always)]
    pub fn txstat(&mut self) -> TXSTAT_W {
        TXSTAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit status base address register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txstatus](index.html) module"]
pub struct TXSTATUS_SPEC;
impl crate::RegisterSpec for TXSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txstatus::R](R) reader structure"]
impl crate::Readable for TXSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txstatus::W](W) writer structure"]
impl crate::Writable for TXSTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXSTATUS to value 0"]
impl crate::Resettable for TXSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
