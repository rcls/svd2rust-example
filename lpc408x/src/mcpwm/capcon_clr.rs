#[doc = "Register `CAPCON_CLR` writer"]
pub struct W(crate::W<CAPCON_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAPCON_CLR_SPEC>;
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
impl From<crate::W<CAPCON_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAPCON_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAP0MCI0_RE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub struct CAP0MCI0_RE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP0MCI0_RE_CLR_W<'a> {
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
#[doc = "Field `CAP0MCI0_FE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub struct CAP0MCI0_FE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP0MCI0_FE_CLR_W<'a> {
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
#[doc = "Field `CAP0MCI1_RE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub struct CAP0MCI1_RE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP0MCI1_RE_CLR_W<'a> {
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
#[doc = "Field `CAP0MCI1_FE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub struct CAP0MCI1_FE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP0MCI1_FE_CLR_W<'a> {
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
#[doc = "Field `CAP0MCI2_RE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub struct CAP0MCI2_RE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP0MCI2_RE_CLR_W<'a> {
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
#[doc = "Field `CAP0MCI2_FE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub struct CAP0MCI2_FE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP0MCI2_FE_CLR_W<'a> {
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
#[doc = "Field `CAP1MCI0_RE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub struct CAP1MCI0_RE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1MCI0_RE_CLR_W<'a> {
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
#[doc = "Field `CAP1MCI0_FE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub struct CAP1MCI0_FE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1MCI0_FE_CLR_W<'a> {
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
#[doc = "Field `CAP1MCI1_RE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub struct CAP1MCI1_RE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1MCI1_RE_CLR_W<'a> {
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
#[doc = "Field `CAP1MCI1_FE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub struct CAP1MCI1_FE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1MCI1_FE_CLR_W<'a> {
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
#[doc = "Field `CAP1MCI2_RE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub struct CAP1MCI2_RE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1MCI2_RE_CLR_W<'a> {
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
#[doc = "Field `CAP1MCI2_FE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub struct CAP1MCI2_FE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1MCI2_FE_CLR_W<'a> {
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
#[doc = "Field `CAP2MCI0_RE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub struct CAP2MCI0_RE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP2MCI0_RE_CLR_W<'a> {
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
#[doc = "Field `CAP2MCI0_FE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub struct CAP2MCI0_FE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP2MCI0_FE_CLR_W<'a> {
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
#[doc = "Field `CAP2MCI1_RE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub struct CAP2MCI1_RE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP2MCI1_RE_CLR_W<'a> {
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
#[doc = "Field `CAP2MCI1_FE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub struct CAP2MCI1_FE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP2MCI1_FE_CLR_W<'a> {
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
#[doc = "Field `CAP2MCI2_RE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub struct CAP2MCI2_RE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP2MCI2_RE_CLR_W<'a> {
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
#[doc = "Field `CAP2MCI2_FE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub struct CAP2MCI2_FE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP2MCI2_FE_CLR_W<'a> {
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
#[doc = "Field `RT0_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub struct RT0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RT0_CLR_W<'a> {
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
#[doc = "Field `RT1_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub struct RT1_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RT1_CLR_W<'a> {
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
#[doc = "Field `RT2_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub struct RT2_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RT2_CLR_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap0mci0_re_clr(&mut self) -> CAP0MCI0_RE_CLR_W {
        CAP0MCI0_RE_CLR_W { w: self }
    }
    #[doc = "Bit 1 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap0mci0_fe_clr(&mut self) -> CAP0MCI0_FE_CLR_W {
        CAP0MCI0_FE_CLR_W { w: self }
    }
    #[doc = "Bit 2 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap0mci1_re_clr(&mut self) -> CAP0MCI1_RE_CLR_W {
        CAP0MCI1_RE_CLR_W { w: self }
    }
    #[doc = "Bit 3 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap0mci1_fe_clr(&mut self) -> CAP0MCI1_FE_CLR_W {
        CAP0MCI1_FE_CLR_W { w: self }
    }
    #[doc = "Bit 4 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap0mci2_re_clr(&mut self) -> CAP0MCI2_RE_CLR_W {
        CAP0MCI2_RE_CLR_W { w: self }
    }
    #[doc = "Bit 5 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap0mci2_fe_clr(&mut self) -> CAP0MCI2_FE_CLR_W {
        CAP0MCI2_FE_CLR_W { w: self }
    }
    #[doc = "Bit 6 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap1mci0_re_clr(&mut self) -> CAP1MCI0_RE_CLR_W {
        CAP1MCI0_RE_CLR_W { w: self }
    }
    #[doc = "Bit 7 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap1mci0_fe_clr(&mut self) -> CAP1MCI0_FE_CLR_W {
        CAP1MCI0_FE_CLR_W { w: self }
    }
    #[doc = "Bit 8 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap1mci1_re_clr(&mut self) -> CAP1MCI1_RE_CLR_W {
        CAP1MCI1_RE_CLR_W { w: self }
    }
    #[doc = "Bit 9 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap1mci1_fe_clr(&mut self) -> CAP1MCI1_FE_CLR_W {
        CAP1MCI1_FE_CLR_W { w: self }
    }
    #[doc = "Bit 10 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap1mci2_re_clr(&mut self) -> CAP1MCI2_RE_CLR_W {
        CAP1MCI2_RE_CLR_W { w: self }
    }
    #[doc = "Bit 11 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap1mci2_fe_clr(&mut self) -> CAP1MCI2_FE_CLR_W {
        CAP1MCI2_FE_CLR_W { w: self }
    }
    #[doc = "Bit 12 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap2mci0_re_clr(&mut self) -> CAP2MCI0_RE_CLR_W {
        CAP2MCI0_RE_CLR_W { w: self }
    }
    #[doc = "Bit 13 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap2mci0_fe_clr(&mut self) -> CAP2MCI0_FE_CLR_W {
        CAP2MCI0_FE_CLR_W { w: self }
    }
    #[doc = "Bit 14 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap2mci1_re_clr(&mut self) -> CAP2MCI1_RE_CLR_W {
        CAP2MCI1_RE_CLR_W { w: self }
    }
    #[doc = "Bit 15 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap2mci1_fe_clr(&mut self) -> CAP2MCI1_FE_CLR_W {
        CAP2MCI1_FE_CLR_W { w: self }
    }
    #[doc = "Bit 16 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap2mci2_re_clr(&mut self) -> CAP2MCI2_RE_CLR_W {
        CAP2MCI2_RE_CLR_W { w: self }
    }
    #[doc = "Bit 17 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap2mci2_fe_clr(&mut self) -> CAP2MCI2_FE_CLR_W {
        CAP2MCI2_FE_CLR_W { w: self }
    }
    #[doc = "Bit 18 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn rt0_clr(&mut self) -> RT0_CLR_W {
        RT0_CLR_W { w: self }
    }
    #[doc = "Bit 19 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn rt1_clr(&mut self) -> RT1_CLR_W {
        RT1_CLR_W { w: self }
    }
    #[doc = "Bit 20 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn rt2_clr(&mut self) -> RT2_CLR_W {
        RT2_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Control clear address\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capcon_clr](index.html) module"]
pub struct CAPCON_CLR_SPEC;
impl crate::RegisterSpec for CAPCON_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [capcon_clr::W](W) writer structure"]
impl crate::Writable for CAPCON_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAPCON_CLR to value 0"]
impl crate::Resettable for CAPCON_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
