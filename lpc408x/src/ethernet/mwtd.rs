#[doc = "Register `MWTD` writer"]
pub struct W(crate::W<MWTD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MWTD_SPEC>;
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
impl From<crate::W<MWTD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MWTD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRITEDATA` writer - WRITE DATA. When written, an MII Mgmt write cycle is performed using the 16-bit data and the pre-configured PHY and Register addresses from the MII Mgmt Address register (MADR)."]
pub struct WRITEDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITEDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - WRITE DATA. When written, an MII Mgmt write cycle is performed using the 16-bit data and the pre-configured PHY and Register addresses from the MII Mgmt Address register (MADR)."]
    #[inline(always)]
    pub fn writedata(&mut self) -> WRITEDATA_W {
        WRITEDATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MII Mgmt Write Data register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mwtd](index.html) module"]
pub struct MWTD_SPEC;
impl crate::RegisterSpec for MWTD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [mwtd::W](W) writer structure"]
impl crate::Writable for MWTD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MWTD to value 0"]
impl crate::Resettable for MWTD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
