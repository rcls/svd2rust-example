#[doc = "Register `BOOKMARK` writer"]
pub struct W(crate::W<BOOKMARK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOOKMARK_SPEC>;
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
impl From<crate::W<BOOKMARK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOOKMARK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOOKMARK` writer - Used by FW. Keeps the Current HV cycle sequence"]
pub struct BOOKMARK_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOKMARK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Used by FW. Keeps the Current HV cycle sequence"]
    #[inline(always)]
    pub fn bookmark(&mut self) -> BOOKMARK_W {
        BOOKMARK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bookmark register - keeps the current FW HV seq\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bookmark](index.html) module"]
pub struct BOOKMARK_SPEC;
impl crate::RegisterSpec for BOOKMARK_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [bookmark::W](W) writer structure"]
impl crate::Writable for BOOKMARK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BOOKMARK to value 0"]
impl crate::Resettable for BOOKMARK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}