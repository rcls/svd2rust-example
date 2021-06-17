#[doc = "Register `WL_ENABLE` reader"]
pub struct R(crate::R<WL_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WL_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WL_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WL_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WL_ENABLE` writer"]
pub struct W(crate::W<WL_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WL_ENABLE_SPEC>;
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
impl From<crate::W<WL_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WL_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WL_ENABLE` reader - Stores the valid entry bit corresponding to each of the eight device address stored in the whitelist. 1 - White list entry is Valid 0 - White list entry is Invalid"]
pub struct WL_ENABLE_R(crate::FieldReader<u16, u16>);
impl WL_ENABLE_R {
    pub(crate) fn new(bits: u16) -> Self {
        WL_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WL_ENABLE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WL_ENABLE` writer - Stores the valid entry bit corresponding to each of the eight device address stored in the whitelist. 1 - White list entry is Valid 0 - White list entry is Invalid"]
pub struct WL_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> WL_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Stores the valid entry bit corresponding to each of the eight device address stored in the whitelist. 1 - White list entry is Valid 0 - White list entry is Invalid"]
    #[inline(always)]
    pub fn wl_enable(&self) -> WL_ENABLE_R {
        WL_ENABLE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Stores the valid entry bit corresponding to each of the eight device address stored in the whitelist. 1 - White list entry is Valid 0 - White list entry is Invalid"]
    #[inline(always)]
    pub fn wl_enable(&mut self) -> WL_ENABLE_W {
        WL_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "whitelist valid entry bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wl_enable](index.html) module"]
pub struct WL_ENABLE_SPEC;
impl crate::RegisterSpec for WL_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wl_enable::R](R) reader structure"]
impl crate::Readable for WL_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wl_enable::W](W) writer structure"]
impl crate::Writable for WL_ENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WL_ENABLE to value 0"]
impl crate::Resettable for WL_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
