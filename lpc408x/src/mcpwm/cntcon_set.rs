#[doc = "Register `CNTCON_SET` writer"]
pub struct W(crate::W<CNTCON_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTCON_SET_SPEC>;
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
impl From<crate::W<CNTCON_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTCON_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TC0MCI0_RE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub struct TC0MCI0_RE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> TC0MCI0_RE_SET_W<'a> {
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
#[doc = "Field `TC0MCI0_FE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub struct TC0MCI0_FE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> TC0MCI0_FE_SET_W<'a> {
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
#[doc = "Field `TC0MCI1_RE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub struct TC0MCI1_RE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> TC0MCI1_RE_SET_W<'a> {
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
#[doc = "Field `TC0MCI1_FE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub struct TC0MCI1_FE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> TC0MCI1_FE_SET_W<'a> {
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
#[doc = "Field `TC0MCI2_RE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub struct TC0MCI2_RE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> TC0MCI2_RE_SET_W<'a> {
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
#[doc = "Field `TC0MCI2_FE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub struct TC0MCI2_FE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> TC0MCI2_FE_SET_W<'a> {
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
#[doc = "Field `TC1MCI0_RE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub struct TC1MCI0_RE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> TC1MCI0_RE_SET_W<'a> {
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
#[doc = "Field `TC1MCI0_FE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub struct TC1MCI0_FE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> TC1MCI0_FE_SET_W<'a> {
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
#[doc = "Field `TC1MCI1_RE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub struct TC1MCI1_RE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> TC1MCI1_RE_SET_W<'a> {
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
#[doc = "Field `TC1MCI1_FE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub struct TC1MCI1_FE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> TC1MCI1_FE_SET_W<'a> {
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
#[doc = "Field `TC1MCI2_RE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub struct TC1MCI2_RE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> TC1MCI2_RE_SET_W<'a> {
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
#[doc = "Field `TC1MCI2_FE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub struct TC1MCI2_FE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> TC1MCI2_FE_SET_W<'a> {
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
#[doc = "Field `TC2MCI0_RE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub struct TC2MCI0_RE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> TC2MCI0_RE_SET_W<'a> {
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
#[doc = "Field `TC2MCI0_FE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub struct TC2MCI0_FE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> TC2MCI0_FE_SET_W<'a> {
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
#[doc = "Field `TC2MCI1_RE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub struct TC2MCI1_RE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> TC2MCI1_RE_SET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `TC2MCI1_FE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub struct TC2MCI1_FE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> TC2MCI1_FE_SET_W<'a> {
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
#[doc = "Field `TC2MCI2_RE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub struct TC2MCI2_RE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> TC2MCI2_RE_SET_W<'a> {
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
#[doc = "Field `TC2MCI2_FE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub struct TC2MCI2_FE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> TC2MCI2_FE_SET_W<'a> {
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
#[doc = "Field `CNTR0_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub struct CNTR0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTR0_SET_W<'a> {
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
#[doc = "Field `CNTR1_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub struct CNTR1_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTR1_SET_W<'a> {
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
#[doc = "Field `CNTR2_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub struct CNTR2_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTR2_SET_W<'a> {
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
    #[doc = "Bit 0 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc0mci0_re_set(&mut self) -> TC0MCI0_RE_SET_W {
        TC0MCI0_RE_SET_W { w: self }
    }
    #[doc = "Bit 1 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc0mci0_fe_set(&mut self) -> TC0MCI0_FE_SET_W {
        TC0MCI0_FE_SET_W { w: self }
    }
    #[doc = "Bit 2 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc0mci1_re_set(&mut self) -> TC0MCI1_RE_SET_W {
        TC0MCI1_RE_SET_W { w: self }
    }
    #[doc = "Bit 3 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc0mci1_fe_set(&mut self) -> TC0MCI1_FE_SET_W {
        TC0MCI1_FE_SET_W { w: self }
    }
    #[doc = "Bit 4 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc0mci2_re_set(&mut self) -> TC0MCI2_RE_SET_W {
        TC0MCI2_RE_SET_W { w: self }
    }
    #[doc = "Bit 5 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc0mci2_fe_set(&mut self) -> TC0MCI2_FE_SET_W {
        TC0MCI2_FE_SET_W { w: self }
    }
    #[doc = "Bit 6 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc1mci0_re_set(&mut self) -> TC1MCI0_RE_SET_W {
        TC1MCI0_RE_SET_W { w: self }
    }
    #[doc = "Bit 7 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc1mci0_fe_set(&mut self) -> TC1MCI0_FE_SET_W {
        TC1MCI0_FE_SET_W { w: self }
    }
    #[doc = "Bit 8 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc1mci1_re_set(&mut self) -> TC1MCI1_RE_SET_W {
        TC1MCI1_RE_SET_W { w: self }
    }
    #[doc = "Bit 9 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc1mci1_fe_set(&mut self) -> TC1MCI1_FE_SET_W {
        TC1MCI1_FE_SET_W { w: self }
    }
    #[doc = "Bit 10 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc1mci2_re_set(&mut self) -> TC1MCI2_RE_SET_W {
        TC1MCI2_RE_SET_W { w: self }
    }
    #[doc = "Bit 11 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc1mci2_fe_set(&mut self) -> TC1MCI2_FE_SET_W {
        TC1MCI2_FE_SET_W { w: self }
    }
    #[doc = "Bit 12 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc2mci0_re_set(&mut self) -> TC2MCI0_RE_SET_W {
        TC2MCI0_RE_SET_W { w: self }
    }
    #[doc = "Bit 13 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc2mci0_fe_set(&mut self) -> TC2MCI0_FE_SET_W {
        TC2MCI0_FE_SET_W { w: self }
    }
    #[doc = "Bit 14 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc2mci1_re_set(&mut self) -> TC2MCI1_RE_SET_W {
        TC2MCI1_RE_SET_W { w: self }
    }
    #[doc = "Bit 15 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc2mci1_fe_set(&mut self) -> TC2MCI1_FE_SET_W {
        TC2MCI1_FE_SET_W { w: self }
    }
    #[doc = "Bit 16 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc2mci2_re_set(&mut self) -> TC2MCI2_RE_SET_W {
        TC2MCI2_RE_SET_W { w: self }
    }
    #[doc = "Bit 17 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc2mci2_fe_set(&mut self) -> TC2MCI2_FE_SET_W {
        TC2MCI2_FE_SET_W { w: self }
    }
    #[doc = "Bit 29 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn cntr0_set(&mut self) -> CNTR0_SET_W {
        CNTR0_SET_W { w: self }
    }
    #[doc = "Bit 30 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn cntr1_set(&mut self) -> CNTR1_SET_W {
        CNTR1_SET_W { w: self }
    }
    #[doc = "Bit 31 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn cntr2_set(&mut self) -> CNTR2_SET_W {
        CNTR2_SET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Count Control set address\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntcon_set](index.html) module"]
pub struct CNTCON_SET_SPEC;
impl crate::RegisterSpec for CNTCON_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cntcon_set::W](W) writer structure"]
impl crate::Writable for CNTCON_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNTCON_SET to value 0"]
impl crate::Resettable for CNTCON_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
