#[doc = "Register `CAP_CLR` writer"]
pub struct W(crate::W<CAP_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAP_CLR_SPEC>;
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
impl From<crate::W<CAP_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAP_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAP_CLR0` writer - Writing a 1 to this bit clears the CAP0 register."]
pub struct CAP_CLR0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP_CLR0_W<'a> {
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
#[doc = "Field `CAP_CLR1` writer - Writing a 1 to this bit clears the CAP1 register."]
pub struct CAP_CLR1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP_CLR1_W<'a> {
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
#[doc = "Field `CAP_CLR2` writer - Writing a 1 to this bit clears the CAP2 register."]
pub struct CAP_CLR2_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP_CLR2_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Writing a 1 to this bit clears the CAP0 register."]
    #[inline(always)]
    pub fn cap_clr0(&mut self) -> CAP_CLR0_W {
        CAP_CLR0_W { w: self }
    }
    #[doc = "Bit 1 - Writing a 1 to this bit clears the CAP1 register."]
    #[inline(always)]
    pub fn cap_clr1(&mut self) -> CAP_CLR1_W {
        CAP_CLR1_W { w: self }
    }
    #[doc = "Bit 2 - Writing a 1 to this bit clears the CAP2 register."]
    #[inline(always)]
    pub fn cap_clr2(&mut self) -> CAP_CLR2_W {
        CAP_CLR2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture clear address\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap_clr](index.html) module"]
pub struct CAP_CLR_SPEC;
impl crate::RegisterSpec for CAP_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cap_clr::W](W) writer structure"]
impl crate::Writable for CAP_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAP_CLR to value 0"]
impl crate::Resettable for CAP_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
