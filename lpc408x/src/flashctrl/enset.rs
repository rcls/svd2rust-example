#[doc = "Register `ENSET` writer"]
pub struct W(crate::W<ENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENSET_SPEC>;
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
impl From<crate::W<ENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDWR_SET_EN` writer - Set read/write operation finished interrupt enable bit (EEPROM). 0: leave corresponding bit unchanged. 1: set corresponding bit."]
pub struct RDWR_SET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RDWR_SET_EN_W<'a> {
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
#[doc = "Field `PROG1_SET_EN` writer - Set program operation finished interrupt enable bit for EEPROM device 1. 0: leave corresponding bit unchanged. 1: set corresponding bit."]
pub struct PROG1_SET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG1_SET_EN_W<'a> {
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
    #[doc = "Bit 26 - Set read/write operation finished interrupt enable bit (EEPROM). 0: leave corresponding bit unchanged. 1: set corresponding bit."]
    #[inline(always)]
    pub fn rdwr_set_en(&mut self) -> RDWR_SET_EN_W {
        RDWR_SET_EN_W { w: self }
    }
    #[doc = "Bit 28 - Set program operation finished interrupt enable bit for EEPROM device 1. 0: leave corresponding bit unchanged. 1: set corresponding bit."]
    #[inline(always)]
    pub fn prog1_set_en(&mut self) -> PROG1_SET_EN_W {
        PROG1_SET_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM interrupt enable set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enset](index.html) module"]
pub struct ENSET_SPEC;
impl crate::RegisterSpec for ENSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [enset::W](W) writer structure"]
impl crate::Writable for ENSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENSET to value 0"]
impl crate::Resettable for ENSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
