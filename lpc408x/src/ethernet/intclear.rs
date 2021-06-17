#[doc = "Register `INTCLEAR` writer"]
pub struct W(crate::W<INTCLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTCLEAR_SPEC>;
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
impl From<crate::W<INTCLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTCLEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXOVERRUNINTCLR` writer - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
pub struct RXOVERRUNINTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVERRUNINTCLR_W<'a> {
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
#[doc = "Field `RXERRORINTCLR` writer - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
pub struct RXERRORINTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXERRORINTCLR_W<'a> {
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
#[doc = "Field `RXFINISHEDINTCLR` writer - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
pub struct RXFINISHEDINTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFINISHEDINTCLR_W<'a> {
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
#[doc = "Field `RXDONEINTCLR` writer - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
pub struct RXDONEINTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDONEINTCLR_W<'a> {
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
#[doc = "Field `TXUNDERRUNINTCLR` writer - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
pub struct TXUNDERRUNINTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUNDERRUNINTCLR_W<'a> {
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
#[doc = "Field `TXERRORINTCLR` writer - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
pub struct TXERRORINTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXERRORINTCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `TXFINISHEDINTCLR` writer - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
pub struct TXFINISHEDINTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFINISHEDINTCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `TXDONEINTCLR` writer - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
pub struct TXDONEINTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDONEINTCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `SOFTINTCLR` writer - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
pub struct SOFTINTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTINTCLR_W<'a> {
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
#[doc = "Field `WAKEUPINTCLR` writer - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
pub struct WAKEUPINTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPINTCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn rxoverrunintclr(&mut self) -> RXOVERRUNINTCLR_W {
        RXOVERRUNINTCLR_W { w: self }
    }
    #[doc = "Bit 1 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn rxerrorintclr(&mut self) -> RXERRORINTCLR_W {
        RXERRORINTCLR_W { w: self }
    }
    #[doc = "Bit 2 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn rxfinishedintclr(&mut self) -> RXFINISHEDINTCLR_W {
        RXFINISHEDINTCLR_W { w: self }
    }
    #[doc = "Bit 3 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn rxdoneintclr(&mut self) -> RXDONEINTCLR_W {
        RXDONEINTCLR_W { w: self }
    }
    #[doc = "Bit 4 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn txunderrunintclr(&mut self) -> TXUNDERRUNINTCLR_W {
        TXUNDERRUNINTCLR_W { w: self }
    }
    #[doc = "Bit 5 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn txerrorintclr(&mut self) -> TXERRORINTCLR_W {
        TXERRORINTCLR_W { w: self }
    }
    #[doc = "Bit 6 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn txfinishedintclr(&mut self) -> TXFINISHEDINTCLR_W {
        TXFINISHEDINTCLR_W { w: self }
    }
    #[doc = "Bit 7 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn txdoneintclr(&mut self) -> TXDONEINTCLR_W {
        TXDONEINTCLR_W { w: self }
    }
    #[doc = "Bit 12 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn softintclr(&mut self) -> SOFTINTCLR_W {
        SOFTINTCLR_W { w: self }
    }
    #[doc = "Bit 13 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn wakeupintclr(&mut self) -> WAKEUPINTCLR_W {
        WAKEUPINTCLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clear register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intclear](index.html) module"]
pub struct INTCLEAR_SPEC;
impl crate::RegisterSpec for INTCLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [intclear::W](W) writer structure"]
impl crate::Writable for INTCLEAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTCLEAR to value 0"]
impl crate::Resettable for INTCLEAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
