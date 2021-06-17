#[doc = "Register `ARB_RW7_RA` reader"]
pub struct R(crate::R<ARB_RW7_RA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARB_RW7_RA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARB_RW7_RA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARB_RW7_RA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARB_RW7_RA` writer"]
pub struct W(crate::W<ARB_RW7_RA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARB_RW7_RA_SPEC>;
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
impl From<crate::W<ARB_RW7_RA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARB_RW7_RA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RA` reader - Read Address for EP"]
pub struct RA_R(crate::FieldReader<u8, u8>);
impl RA_R {
    pub(crate) fn new(bits: u8) -> Self {
        RA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RA` writer - Read Address for EP"]
pub struct RA_W<'a> {
    w: &'a mut W,
}
impl<'a> RA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Read Address for EP"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read Address for EP"]
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W {
        RA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Read Address value *1, *2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw7_ra](index.html) module"]
pub struct ARB_RW7_RA_SPEC;
impl crate::RegisterSpec for ARB_RW7_RA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [arb_rw7_ra::R](R) reader structure"]
impl crate::Readable for ARB_RW7_RA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [arb_rw7_ra::W](W) writer structure"]
impl crate::Writable for ARB_RW7_RA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ARB_RW7_RA to value 0"]
impl crate::Resettable for ARB_RW7_RA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
