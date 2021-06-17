#[doc = "Register `CON_CLR` writer"]
pub struct W(crate::W<CON_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CON_CLR_SPEC>;
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
impl From<crate::W<CON_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CON_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RUN0_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub struct RUN0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN0_CLR_W<'a> {
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
#[doc = "Field `CENTER0_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub struct CENTER0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CENTER0_CLR_W<'a> {
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
#[doc = "Field `POLA0_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub struct POLA0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> POLA0_CLR_W<'a> {
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
#[doc = "Field `DTE0_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub struct DTE0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DTE0_CLR_W<'a> {
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
#[doc = "Field `DISUP0_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub struct DISUP0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DISUP0_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `RUN1_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub struct RUN1_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN1_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `CENTER1_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub struct CENTER1_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CENTER1_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `POLA1_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub struct POLA1_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> POLA1_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `DTE1_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub struct DTE1_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DTE1_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `DISUP1_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub struct DISUP1_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DISUP1_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `RUN2_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub struct RUN2_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN2_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `CENTER2_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub struct CENTER2_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CENTER2_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `POLA2_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub struct POLA2_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> POLA2_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `DTE2_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub struct DTE2_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DTE2_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `DISUP2_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub struct DISUP2_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DISUP2_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `INVBDC_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub struct INVBDC_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> INVBDC_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `ACMOD_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub struct ACMOD_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMOD_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `DCMODE_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub struct DCMODE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMODE_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn run0_clr(&mut self) -> RUN0_CLR_W {
        RUN0_CLR_W { w: self }
    }
    #[doc = "Bit 1 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn center0_clr(&mut self) -> CENTER0_CLR_W {
        CENTER0_CLR_W { w: self }
    }
    #[doc = "Bit 2 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn pola0_clr(&mut self) -> POLA0_CLR_W {
        POLA0_CLR_W { w: self }
    }
    #[doc = "Bit 3 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn dte0_clr(&mut self) -> DTE0_CLR_W {
        DTE0_CLR_W { w: self }
    }
    #[doc = "Bit 4 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn disup0_clr(&mut self) -> DISUP0_CLR_W {
        DISUP0_CLR_W { w: self }
    }
    #[doc = "Bit 8 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn run1_clr(&mut self) -> RUN1_CLR_W {
        RUN1_CLR_W { w: self }
    }
    #[doc = "Bit 9 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn center1_clr(&mut self) -> CENTER1_CLR_W {
        CENTER1_CLR_W { w: self }
    }
    #[doc = "Bit 10 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn pola1_clr(&mut self) -> POLA1_CLR_W {
        POLA1_CLR_W { w: self }
    }
    #[doc = "Bit 11 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn dte1_clr(&mut self) -> DTE1_CLR_W {
        DTE1_CLR_W { w: self }
    }
    #[doc = "Bit 12 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn disup1_clr(&mut self) -> DISUP1_CLR_W {
        DISUP1_CLR_W { w: self }
    }
    #[doc = "Bit 16 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn run2_clr(&mut self) -> RUN2_CLR_W {
        RUN2_CLR_W { w: self }
    }
    #[doc = "Bit 17 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn center2_clr(&mut self) -> CENTER2_CLR_W {
        CENTER2_CLR_W { w: self }
    }
    #[doc = "Bit 18 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn pola2_clr(&mut self) -> POLA2_CLR_W {
        POLA2_CLR_W { w: self }
    }
    #[doc = "Bit 19 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn dte2_clr(&mut self) -> DTE2_CLR_W {
        DTE2_CLR_W { w: self }
    }
    #[doc = "Bit 20 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn disup2_clr(&mut self) -> DISUP2_CLR_W {
        DISUP2_CLR_W { w: self }
    }
    #[doc = "Bit 29 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn invbdc_clr(&mut self) -> INVBDC_CLR_W {
        INVBDC_CLR_W { w: self }
    }
    #[doc = "Bit 30 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn acmod_clr(&mut self) -> ACMOD_CLR_W {
        ACMOD_CLR_W { w: self }
    }
    #[doc = "Bit 31 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn dcmode_clr(&mut self) -> DCMODE_CLR_W {
        DCMODE_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Control clear address\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [con_clr](index.html) module"]
pub struct CON_CLR_SPEC;
impl crate::RegisterSpec for CON_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [con_clr::W](W) writer structure"]
impl crate::Writable for CON_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CON_CLR to value 0"]
impl crate::Resettable for CON_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
