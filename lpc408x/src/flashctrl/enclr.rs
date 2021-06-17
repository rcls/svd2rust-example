#[doc = "Register `ENCLR` writer"]
pub struct W(crate::W<ENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENCLR_SPEC>;
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
impl From<crate::W<ENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDWR_CLR_EN` writer - Clear read/write operation finished interrupt enable bit (EEPROM). 0: leave corresponding bit unchanged. 1: clear corresponding bit."]
pub struct RDWR_CLR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RDWR_CLR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `PROG1_CLR_EN` writer - Clear program operation finished interrupt enable bit for EEPROM device 1. 0: leave corresponding bit unchanged. 1: clear corresponding bit."]
pub struct PROG1_CLR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG1_CLR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
impl W {
    #[doc = "Bit 26 - Clear read/write operation finished interrupt enable bit (EEPROM). 0: leave corresponding bit unchanged. 1: clear corresponding bit."]
    #[inline(always)]
    pub fn rdwr_clr_en(&mut self) -> RDWR_CLR_EN_W {
        RDWR_CLR_EN_W { w: self }
    }
    #[doc = "Bit 28 - Clear program operation finished interrupt enable bit for EEPROM device 1. 0: leave corresponding bit unchanged. 1: clear corresponding bit."]
    #[inline(always)]
    pub fn prog1_clr_en(&mut self) -> PROG1_CLR_EN_W {
        PROG1_CLR_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM interrupt enable clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enclr](index.html) module"]
pub struct ENCLR_SPEC;
impl crate::RegisterSpec for ENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [enclr::W](W) writer structure"]
impl crate::Writable for ENCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENCLR to value 0"]
impl crate::Resettable for ENCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
