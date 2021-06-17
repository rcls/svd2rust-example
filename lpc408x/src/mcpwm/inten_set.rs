#[doc = "Register `INTEN_SET` writer"]
pub struct W(crate::W<INTEN_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SET_SPEC>;
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
impl From<crate::W<INTEN_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ILIM0_SET` writer - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
pub struct ILIM0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ILIM0_SET_W<'a> {
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
#[doc = "Field `IMAT0_SET` writer - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
pub struct IMAT0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> IMAT0_SET_W<'a> {
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
#[doc = "Field `ICAP0_SET` writer - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
pub struct ICAP0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ICAP0_SET_W<'a> {
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
#[doc = "Field `ILIM1_SET` writer - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
pub struct ILIM1_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ILIM1_SET_W<'a> {
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
#[doc = "Field `IMAT1_SET` writer - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
pub struct IMAT1_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> IMAT1_SET_W<'a> {
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
#[doc = "Field `ICAP1_SET` writer - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
pub struct ICAP1_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ICAP1_SET_W<'a> {
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
#[doc = "Field `ILIM2_SET` writer - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
pub struct ILIM2_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ILIM2_SET_W<'a> {
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
#[doc = "Field `IMAT2_SET` writer - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
pub struct IMAT2_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> IMAT2_SET_W<'a> {
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
#[doc = "Field `ICAP2_SET` writer - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
pub struct ICAP2_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ICAP2_SET_W<'a> {
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
#[doc = "Field `ABORT_SET` writer - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
pub struct ABORT_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ABORT_SET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    pub fn ilim0_set(&mut self) -> ILIM0_SET_W {
        ILIM0_SET_W { w: self }
    }
    #[doc = "Bit 1 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    pub fn imat0_set(&mut self) -> IMAT0_SET_W {
        IMAT0_SET_W { w: self }
    }
    #[doc = "Bit 2 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    pub fn icap0_set(&mut self) -> ICAP0_SET_W {
        ICAP0_SET_W { w: self }
    }
    #[doc = "Bit 4 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    pub fn ilim1_set(&mut self) -> ILIM1_SET_W {
        ILIM1_SET_W { w: self }
    }
    #[doc = "Bit 5 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    pub fn imat1_set(&mut self) -> IMAT1_SET_W {
        IMAT1_SET_W { w: self }
    }
    #[doc = "Bit 6 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    pub fn icap1_set(&mut self) -> ICAP1_SET_W {
        ICAP1_SET_W { w: self }
    }
    #[doc = "Bit 9 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    pub fn ilim2_set(&mut self) -> ILIM2_SET_W {
        ILIM2_SET_W { w: self }
    }
    #[doc = "Bit 10 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    pub fn imat2_set(&mut self) -> IMAT2_SET_W {
        IMAT2_SET_W { w: self }
    }
    #[doc = "Bit 11 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    pub fn icap2_set(&mut self) -> ICAP2_SET_W {
        ICAP2_SET_W { w: self }
    }
    #[doc = "Bit 15 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    pub fn abort_set(&mut self) -> ABORT_SET_W {
        ABORT_SET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable set address\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten_set](index.html) module"]
pub struct INTEN_SET_SPEC;
impl crate::RegisterSpec for INTEN_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [inten_set::W](W) writer structure"]
impl crate::Writable for INTEN_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN_SET to value 0"]
impl crate::Resettable for INTEN_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
