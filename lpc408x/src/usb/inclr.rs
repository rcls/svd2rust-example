#[doc = "Register `INCLR` writer"]
pub struct W(crate::W<INCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INCLR_SPEC>;
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
impl From<crate::W<INCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMR_CLR` writer - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
pub struct TMR_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR_CLR_W<'a> {
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
#[doc = "Field `REMOVE_PU_CLR` writer - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
pub struct REMOVE_PU_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> REMOVE_PU_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `HNP_FAILURE_CLR` writer - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
pub struct HNP_FAILURE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HNP_FAILURE_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `HNP_SUCCES_CLR` writer - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
pub struct HNP_SUCCES_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HNP_SUCCES_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn tmr_clr(&mut self) -> TMR_CLR_W {
        TMR_CLR_W { w: self }
    }
    #[doc = "Bit 1 - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn remove_pu_clr(&mut self) -> REMOVE_PU_CLR_W {
        REMOVE_PU_CLR_W { w: self }
    }
    #[doc = "Bit 2 - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn hnp_failure_clr(&mut self) -> HNP_FAILURE_CLR_W {
        HNP_FAILURE_CLR_W { w: self }
    }
    #[doc = "Bit 3 - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn hnp_succes_clr(&mut self) -> HNP_SUCCES_CLR_W {
        HNP_SUCCES_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG Interrupt Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inclr](index.html) module"]
pub struct INCLR_SPEC;
impl crate::RegisterSpec for INCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [inclr::W](W) writer structure"]
impl crate::Writable for INCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INCLR to value 0"]
impl crate::Resettable for INCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
