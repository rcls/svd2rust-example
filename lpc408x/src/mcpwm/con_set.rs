#[doc = "Register `CON_SET` writer"]
pub struct W(crate::W<CON_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CON_SET_SPEC>;
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
impl From<crate::W<CON_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CON_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RUN0_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub struct RUN0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN0_SET_W<'a> {
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
#[doc = "Field `CENTER0_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub struct CENTER0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CENTER0_SET_W<'a> {
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
#[doc = "Field `POLA0_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub struct POLA0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> POLA0_SET_W<'a> {
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
#[doc = "Field `DTE0_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub struct DTE0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> DTE0_SET_W<'a> {
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
#[doc = "Field `DISUP0_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub struct DISUP0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> DISUP0_SET_W<'a> {
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
#[doc = "Field `RUN1_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub struct RUN1_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN1_SET_W<'a> {
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
#[doc = "Field `CENTER1_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub struct CENTER1_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CENTER1_SET_W<'a> {
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
#[doc = "Field `POLA1_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub struct POLA1_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> POLA1_SET_W<'a> {
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
#[doc = "Field `DTE1_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub struct DTE1_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> DTE1_SET_W<'a> {
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
#[doc = "Field `DISUP1_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub struct DISUP1_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> DISUP1_SET_W<'a> {
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
#[doc = "Field `RUN2_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub struct RUN2_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN2_SET_W<'a> {
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
#[doc = "Field `CENTER2_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub struct CENTER2_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CENTER2_SET_W<'a> {
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
#[doc = "Field `POLA2_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub struct POLA2_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> POLA2_SET_W<'a> {
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
#[doc = "Field `DTE2_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub struct DTE2_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> DTE2_SET_W<'a> {
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
#[doc = "Field `DISUP2_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub struct DISUP2_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> DISUP2_SET_W<'a> {
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
#[doc = "Field `INVBDC_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub struct INVBDC_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> INVBDC_SET_W<'a> {
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
#[doc = "Field `ACMODE_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub struct ACMODE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMODE_SET_W<'a> {
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
#[doc = "Field `DCMODE_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub struct DCMODE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMODE_SET_W<'a> {
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
    #[doc = "Bit 0 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn run0_set(&mut self) -> RUN0_SET_W {
        RUN0_SET_W { w: self }
    }
    #[doc = "Bit 1 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn center0_set(&mut self) -> CENTER0_SET_W {
        CENTER0_SET_W { w: self }
    }
    #[doc = "Bit 2 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn pola0_set(&mut self) -> POLA0_SET_W {
        POLA0_SET_W { w: self }
    }
    #[doc = "Bit 3 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn dte0_set(&mut self) -> DTE0_SET_W {
        DTE0_SET_W { w: self }
    }
    #[doc = "Bit 4 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn disup0_set(&mut self) -> DISUP0_SET_W {
        DISUP0_SET_W { w: self }
    }
    #[doc = "Bit 8 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn run1_set(&mut self) -> RUN1_SET_W {
        RUN1_SET_W { w: self }
    }
    #[doc = "Bit 9 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn center1_set(&mut self) -> CENTER1_SET_W {
        CENTER1_SET_W { w: self }
    }
    #[doc = "Bit 10 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn pola1_set(&mut self) -> POLA1_SET_W {
        POLA1_SET_W { w: self }
    }
    #[doc = "Bit 11 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn dte1_set(&mut self) -> DTE1_SET_W {
        DTE1_SET_W { w: self }
    }
    #[doc = "Bit 12 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn disup1_set(&mut self) -> DISUP1_SET_W {
        DISUP1_SET_W { w: self }
    }
    #[doc = "Bit 16 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn run2_set(&mut self) -> RUN2_SET_W {
        RUN2_SET_W { w: self }
    }
    #[doc = "Bit 17 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn center2_set(&mut self) -> CENTER2_SET_W {
        CENTER2_SET_W { w: self }
    }
    #[doc = "Bit 18 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn pola2_set(&mut self) -> POLA2_SET_W {
        POLA2_SET_W { w: self }
    }
    #[doc = "Bit 19 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn dte2_set(&mut self) -> DTE2_SET_W {
        DTE2_SET_W { w: self }
    }
    #[doc = "Bit 20 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn disup2_set(&mut self) -> DISUP2_SET_W {
        DISUP2_SET_W { w: self }
    }
    #[doc = "Bit 29 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn invbdc_set(&mut self) -> INVBDC_SET_W {
        INVBDC_SET_W { w: self }
    }
    #[doc = "Bit 30 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn acmode_set(&mut self) -> ACMODE_SET_W {
        ACMODE_SET_W { w: self }
    }
    #[doc = "Bit 31 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn dcmode_set(&mut self) -> DCMODE_SET_W {
        DCMODE_SET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Control set address\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [con_set](index.html) module"]
pub struct CON_SET_SPEC;
impl crate::RegisterSpec for CON_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [con_set::W](W) writer structure"]
impl crate::Writable for CON_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CON_SET to value 0"]
impl crate::Resettable for CON_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
