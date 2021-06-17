#[doc = "Register `ARB_RW6_WA_MSB` reader"]
pub struct R(crate::R<ARB_RW6_WA_MSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARB_RW6_WA_MSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARB_RW6_WA_MSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARB_RW6_WA_MSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARB_RW6_WA_MSB` writer"]
pub struct W(crate::W<ARB_RW6_WA_MSB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARB_RW6_WA_MSB_SPEC>;
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
impl From<crate::W<ARB_RW6_WA_MSB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARB_RW6_WA_MSB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WA_MSB` reader - Write Address for EP"]
pub struct WA_MSB_R(crate::FieldReader<bool, bool>);
impl WA_MSB_R {
    pub(crate) fn new(bits: bool) -> Self {
        WA_MSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WA_MSB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WA_MSB` writer - Write Address for EP"]
pub struct WA_MSB_W<'a> {
    w: &'a mut W,
}
impl<'a> WA_MSB_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write Address for EP"]
    #[inline(always)]
    pub fn wa_msb(&self) -> WA_MSB_R {
        WA_MSB_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write Address for EP"]
    #[inline(always)]
    pub fn wa_msb(&mut self) -> WA_MSB_W {
        WA_MSB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Write Address value *1, *2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw6_wa_msb](index.html) module"]
pub struct ARB_RW6_WA_MSB_SPEC;
impl crate::RegisterSpec for ARB_RW6_WA_MSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [arb_rw6_wa_msb::R](R) reader structure"]
impl crate::Readable for ARB_RW6_WA_MSB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [arb_rw6_wa_msb::W](W) writer structure"]
impl crate::Writable for ARB_RW6_WA_MSB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ARB_RW6_WA_MSB to value 0"]
impl crate::Resettable for ARB_RW6_WA_MSB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
