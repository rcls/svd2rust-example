#[doc = "Register `INTF_SET` writer"]
pub struct W(crate::W<INTF_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTF_SET_SPEC>;
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
impl From<crate::W<INTF_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTF_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ILIM0_F_SET` writer - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
pub struct ILIM0_F_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ILIM0_F_SET_W<'a> {
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
#[doc = "Field `IMAT0_F_SET` writer - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
pub struct IMAT0_F_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> IMAT0_F_SET_W<'a> {
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
#[doc = "Field `ICAP0_F_SET` writer - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
pub struct ICAP0_F_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ICAP0_F_SET_W<'a> {
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
#[doc = "Field `ILIM1_F_SET` writer - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
pub struct ILIM1_F_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ILIM1_F_SET_W<'a> {
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
#[doc = "Field `IMAT1_F_SET` writer - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
pub struct IMAT1_F_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> IMAT1_F_SET_W<'a> {
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
#[doc = "Field `ICAP1_F_SET` writer - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
pub struct ICAP1_F_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ICAP1_F_SET_W<'a> {
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
#[doc = "Field `ILIM2_F_SET` writer - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
pub struct ILIM2_F_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ILIM2_F_SET_W<'a> {
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
#[doc = "Field `IMAT2_F_SET` writer - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
pub struct IMAT2_F_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> IMAT2_F_SET_W<'a> {
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
#[doc = "Field `ICAP2_F_SET` writer - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
pub struct ICAP2_F_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ICAP2_F_SET_W<'a> {
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
#[doc = "Field `ABORT_F_SET` writer - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
pub struct ABORT_F_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ABORT_F_SET_W<'a> {
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
    #[doc = "Bit 0 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    pub fn ilim0_f_set(&mut self) -> ILIM0_F_SET_W {
        ILIM0_F_SET_W { w: self }
    }
    #[doc = "Bit 1 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    pub fn imat0_f_set(&mut self) -> IMAT0_F_SET_W {
        IMAT0_F_SET_W { w: self }
    }
    #[doc = "Bit 2 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    pub fn icap0_f_set(&mut self) -> ICAP0_F_SET_W {
        ICAP0_F_SET_W { w: self }
    }
    #[doc = "Bit 4 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    pub fn ilim1_f_set(&mut self) -> ILIM1_F_SET_W {
        ILIM1_F_SET_W { w: self }
    }
    #[doc = "Bit 5 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    pub fn imat1_f_set(&mut self) -> IMAT1_F_SET_W {
        IMAT1_F_SET_W { w: self }
    }
    #[doc = "Bit 6 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    pub fn icap1_f_set(&mut self) -> ICAP1_F_SET_W {
        ICAP1_F_SET_W { w: self }
    }
    #[doc = "Bit 8 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    pub fn ilim2_f_set(&mut self) -> ILIM2_F_SET_W {
        ILIM2_F_SET_W { w: self }
    }
    #[doc = "Bit 9 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    pub fn imat2_f_set(&mut self) -> IMAT2_F_SET_W {
        IMAT2_F_SET_W { w: self }
    }
    #[doc = "Bit 10 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    pub fn icap2_f_set(&mut self) -> ICAP2_F_SET_W {
        ICAP2_F_SET_W { w: self }
    }
    #[doc = "Bit 15 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    pub fn abort_f_set(&mut self) -> ABORT_F_SET_W {
        ABORT_F_SET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt flags set address\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intf_set](index.html) module"]
pub struct INTF_SET_SPEC;
impl crate::RegisterSpec for INTF_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [intf_set::W](W) writer structure"]
impl crate::Writable for INTF_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTF_SET to value 0"]
impl crate::Resettable for INTF_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
