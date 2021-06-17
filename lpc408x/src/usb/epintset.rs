#[doc = "Register `EPINTSET` writer"]
pub struct W(crate::W<EPINTSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPINTSET_SPEC>;
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
impl From<crate::W<EPINTSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPINTSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPSET0` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET0_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET0_W<'a> {
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
#[doc = "Field `EPSET1` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET1_W<'a> {
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
#[doc = "Field `EPSET2` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET2_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET2_W<'a> {
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
#[doc = "Field `EPSET3` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET3_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET3_W<'a> {
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
#[doc = "Field `EPSET4` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET4_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET4_W<'a> {
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
#[doc = "Field `EPSET5` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET5_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET5_W<'a> {
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
#[doc = "Field `EPSET6` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET6_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET6_W<'a> {
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
#[doc = "Field `EPSET7` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET7_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET7_W<'a> {
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
#[doc = "Field `EPSET8` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET8_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET8_W<'a> {
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
#[doc = "Field `EPSET9` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET9_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET9_W<'a> {
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
#[doc = "Field `EPSET10` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET10_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET10_W<'a> {
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
#[doc = "Field `EPSET11` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET11_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET11_W<'a> {
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
#[doc = "Field `EPSET12` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET12_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET12_W<'a> {
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
#[doc = "Field `EPSET13` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET13_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET13_W<'a> {
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
#[doc = "Field `EPSET14` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET14_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET14_W<'a> {
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
#[doc = "Field `EPSET15` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET15_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET15_W<'a> {
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
#[doc = "Field `EPSET16` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET16_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET16_W<'a> {
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
#[doc = "Field `EPSET17` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET17_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET17_W<'a> {
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
#[doc = "Field `EPSET18` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET18_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET18_W<'a> {
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
#[doc = "Field `EPSET19` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET19_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET19_W<'a> {
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
#[doc = "Field `EPSET20` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET20_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET20_W<'a> {
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
#[doc = "Field `EPSET21` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET21_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET21_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `EPSET22` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET22_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET22_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `EPSET23` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET23_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET23_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `EPSET24` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET24_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET24_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `EPSET25` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET25_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET25_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `EPSET26` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET26_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET26_W<'a> {
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
#[doc = "Field `EPSET27` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET27_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET27_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `EPSET28` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET28_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET28_W<'a> {
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
#[doc = "Field `EPSET29` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET29_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET29_W<'a> {
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
#[doc = "Field `EPSET30` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET30_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET30_W<'a> {
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
#[doc = "Field `EPSET31` writer - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
pub struct EPSET31_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET31_W<'a> {
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
    #[doc = "Bit 0 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset0(&mut self) -> EPSET0_W {
        EPSET0_W { w: self }
    }
    #[doc = "Bit 1 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset1(&mut self) -> EPSET1_W {
        EPSET1_W { w: self }
    }
    #[doc = "Bit 2 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset2(&mut self) -> EPSET2_W {
        EPSET2_W { w: self }
    }
    #[doc = "Bit 3 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset3(&mut self) -> EPSET3_W {
        EPSET3_W { w: self }
    }
    #[doc = "Bit 4 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset4(&mut self) -> EPSET4_W {
        EPSET4_W { w: self }
    }
    #[doc = "Bit 5 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset5(&mut self) -> EPSET5_W {
        EPSET5_W { w: self }
    }
    #[doc = "Bit 6 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset6(&mut self) -> EPSET6_W {
        EPSET6_W { w: self }
    }
    #[doc = "Bit 7 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset7(&mut self) -> EPSET7_W {
        EPSET7_W { w: self }
    }
    #[doc = "Bit 8 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset8(&mut self) -> EPSET8_W {
        EPSET8_W { w: self }
    }
    #[doc = "Bit 9 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset9(&mut self) -> EPSET9_W {
        EPSET9_W { w: self }
    }
    #[doc = "Bit 10 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset10(&mut self) -> EPSET10_W {
        EPSET10_W { w: self }
    }
    #[doc = "Bit 11 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset11(&mut self) -> EPSET11_W {
        EPSET11_W { w: self }
    }
    #[doc = "Bit 12 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset12(&mut self) -> EPSET12_W {
        EPSET12_W { w: self }
    }
    #[doc = "Bit 13 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset13(&mut self) -> EPSET13_W {
        EPSET13_W { w: self }
    }
    #[doc = "Bit 14 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset14(&mut self) -> EPSET14_W {
        EPSET14_W { w: self }
    }
    #[doc = "Bit 15 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset15(&mut self) -> EPSET15_W {
        EPSET15_W { w: self }
    }
    #[doc = "Bit 16 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset16(&mut self) -> EPSET16_W {
        EPSET16_W { w: self }
    }
    #[doc = "Bit 17 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset17(&mut self) -> EPSET17_W {
        EPSET17_W { w: self }
    }
    #[doc = "Bit 18 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset18(&mut self) -> EPSET18_W {
        EPSET18_W { w: self }
    }
    #[doc = "Bit 19 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset19(&mut self) -> EPSET19_W {
        EPSET19_W { w: self }
    }
    #[doc = "Bit 20 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset20(&mut self) -> EPSET20_W {
        EPSET20_W { w: self }
    }
    #[doc = "Bit 21 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset21(&mut self) -> EPSET21_W {
        EPSET21_W { w: self }
    }
    #[doc = "Bit 22 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset22(&mut self) -> EPSET22_W {
        EPSET22_W { w: self }
    }
    #[doc = "Bit 23 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset23(&mut self) -> EPSET23_W {
        EPSET23_W { w: self }
    }
    #[doc = "Bit 24 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset24(&mut self) -> EPSET24_W {
        EPSET24_W { w: self }
    }
    #[doc = "Bit 25 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset25(&mut self) -> EPSET25_W {
        EPSET25_W { w: self }
    }
    #[doc = "Bit 26 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset26(&mut self) -> EPSET26_W {
        EPSET26_W { w: self }
    }
    #[doc = "Bit 27 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset27(&mut self) -> EPSET27_W {
        EPSET27_W { w: self }
    }
    #[doc = "Bit 28 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset28(&mut self) -> EPSET28_W {
        EPSET28_W { w: self }
    }
    #[doc = "Bit 29 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset29(&mut self) -> EPSET29_W {
        EPSET29_W { w: self }
    }
    #[doc = "Bit 30 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset30(&mut self) -> EPSET30_W {
        EPSET30_W { w: self }
    }
    #[doc = "Bit 31 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset31(&mut self) -> EPSET31_W {
        EPSET31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Endpoint Interrupt Set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epintset](index.html) module"]
pub struct EPINTSET_SPEC;
impl crate::RegisterSpec for EPINTSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [epintset::W](W) writer structure"]
impl crate::Writable for EPINTSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPINTSET to value 0"]
impl crate::Resettable for EPINTSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
