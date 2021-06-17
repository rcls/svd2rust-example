#[doc = "Register `HOST_EP1_RW2_DR` reader"]
pub struct R(crate::R<HOST_EP1_RW2_DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_EP1_RW2_DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_EP1_RW2_DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_EP1_RW2_DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_EP1_RW2_DR` writer"]
pub struct W(crate::W<HOST_EP1_RW2_DR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_EP1_RW2_DR_SPEC>;
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
impl From<crate::W<HOST_EP1_RW2_DR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_EP1_RW2_DR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BFDT16` reader - Data Register for EP1 for 2-byte data"]
pub struct BFDT16_R(crate::FieldReader<u16, u16>);
impl BFDT16_R {
    pub(crate) fn new(bits: u16) -> Self {
        BFDT16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BFDT16_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BFDT16` writer - Data Register for EP1 for 2-byte data"]
pub struct BFDT16_W<'a> {
    w: &'a mut W,
}
impl<'a> BFDT16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Data Register for EP1 for 2-byte data"]
    #[inline(always)]
    pub fn bfdt16(&self) -> BFDT16_R {
        BFDT16_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Register for EP1 for 2-byte data"]
    #[inline(always)]
    pub fn bfdt16(&mut self) -> BFDT16_W {
        BFDT16_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Endpoint 1 Data 2-Byte Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_ep1_rw2_dr](index.html) module"]
pub struct HOST_EP1_RW2_DR_SPEC;
impl crate::RegisterSpec for HOST_EP1_RW2_DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_ep1_rw2_dr::R](R) reader structure"]
impl crate::Readable for HOST_EP1_RW2_DR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_ep1_rw2_dr::W](W) writer structure"]
impl crate::Writable for HOST_EP1_RW2_DR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_EP1_RW2_DR to value 0"]
impl crate::Resettable for HOST_EP1_RW2_DR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
