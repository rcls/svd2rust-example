#[doc = "Register `NDDRINTSET` writer"]
pub struct W(crate::W<NDDRINTSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NDDRINTSET_SPEC>;
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
impl From<crate::W<NDDRINTSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NDDRINTSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPNDDINTSET0` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET0_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET0_W<'a> {
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
#[doc = "Field `EPNDDINTSET1` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET1_W<'a> {
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
#[doc = "Field `EPNDDINTSET2` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET2_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET2_W<'a> {
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
#[doc = "Field `EPNDDINTSET3` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET3_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET3_W<'a> {
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
#[doc = "Field `EPNDDINTSET4` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET4_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET4_W<'a> {
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
#[doc = "Field `EPNDDINTSET5` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET5_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET5_W<'a> {
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
#[doc = "Field `EPNDDINTSET6` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET6_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET6_W<'a> {
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
#[doc = "Field `EPNDDINTSET7` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET7_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET7_W<'a> {
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
#[doc = "Field `EPNDDINTSET8` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET8_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET8_W<'a> {
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
#[doc = "Field `EPNDDINTSET9` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET9_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET9_W<'a> {
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
#[doc = "Field `EPNDDINTSET10` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET10_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET10_W<'a> {
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
#[doc = "Field `EPNDDINTSET11` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET11_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET11_W<'a> {
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
#[doc = "Field `EPNDDINTSET12` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET12_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET12_W<'a> {
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
#[doc = "Field `EPNDDINTSET13` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET13_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET13_W<'a> {
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
#[doc = "Field `EPNDDINTSET14` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET14_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET14_W<'a> {
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
#[doc = "Field `EPNDDINTSET15` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET15_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET15_W<'a> {
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
#[doc = "Field `EPNDDINTSET16` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET16_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET16_W<'a> {
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
#[doc = "Field `EPNDDINTSET17` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET17_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET17_W<'a> {
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
#[doc = "Field `EPNDDINTSET18` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET18_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET18_W<'a> {
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
#[doc = "Field `EPNDDINTSET19` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET19_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET19_W<'a> {
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
#[doc = "Field `EPNDDINTSET20` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET20_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET20_W<'a> {
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
#[doc = "Field `EPNDDINTSET21` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET21_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET21_W<'a> {
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
#[doc = "Field `EPNDDINTSET22` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET22_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET22_W<'a> {
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
#[doc = "Field `EPNDDINTSET23` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET23_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET23_W<'a> {
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
#[doc = "Field `EPNDDINTSET24` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET24_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET24_W<'a> {
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
#[doc = "Field `EPNDDINTSET25` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET25_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET25_W<'a> {
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
#[doc = "Field `EPNDDINTSET26` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET26_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET26_W<'a> {
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
#[doc = "Field `EPNDDINTSET27` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET27_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET27_W<'a> {
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
#[doc = "Field `EPNDDINTSET28` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET28_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET28_W<'a> {
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
#[doc = "Field `EPNDDINTSET29` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET29_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET29_W<'a> {
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
#[doc = "Field `EPNDDINTSET30` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET30_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET30_W<'a> {
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
#[doc = "Field `EPNDDINTSET31` writer - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
pub struct EPNDDINTSET31_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTSET31_W<'a> {
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
    #[doc = "Bit 0 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset0(&mut self) -> EPNDDINTSET0_W {
        EPNDDINTSET0_W { w: self }
    }
    #[doc = "Bit 1 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset1(&mut self) -> EPNDDINTSET1_W {
        EPNDDINTSET1_W { w: self }
    }
    #[doc = "Bit 2 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset2(&mut self) -> EPNDDINTSET2_W {
        EPNDDINTSET2_W { w: self }
    }
    #[doc = "Bit 3 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset3(&mut self) -> EPNDDINTSET3_W {
        EPNDDINTSET3_W { w: self }
    }
    #[doc = "Bit 4 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset4(&mut self) -> EPNDDINTSET4_W {
        EPNDDINTSET4_W { w: self }
    }
    #[doc = "Bit 5 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset5(&mut self) -> EPNDDINTSET5_W {
        EPNDDINTSET5_W { w: self }
    }
    #[doc = "Bit 6 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset6(&mut self) -> EPNDDINTSET6_W {
        EPNDDINTSET6_W { w: self }
    }
    #[doc = "Bit 7 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset7(&mut self) -> EPNDDINTSET7_W {
        EPNDDINTSET7_W { w: self }
    }
    #[doc = "Bit 8 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset8(&mut self) -> EPNDDINTSET8_W {
        EPNDDINTSET8_W { w: self }
    }
    #[doc = "Bit 9 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset9(&mut self) -> EPNDDINTSET9_W {
        EPNDDINTSET9_W { w: self }
    }
    #[doc = "Bit 10 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset10(&mut self) -> EPNDDINTSET10_W {
        EPNDDINTSET10_W { w: self }
    }
    #[doc = "Bit 11 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset11(&mut self) -> EPNDDINTSET11_W {
        EPNDDINTSET11_W { w: self }
    }
    #[doc = "Bit 12 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset12(&mut self) -> EPNDDINTSET12_W {
        EPNDDINTSET12_W { w: self }
    }
    #[doc = "Bit 13 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset13(&mut self) -> EPNDDINTSET13_W {
        EPNDDINTSET13_W { w: self }
    }
    #[doc = "Bit 14 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset14(&mut self) -> EPNDDINTSET14_W {
        EPNDDINTSET14_W { w: self }
    }
    #[doc = "Bit 15 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset15(&mut self) -> EPNDDINTSET15_W {
        EPNDDINTSET15_W { w: self }
    }
    #[doc = "Bit 16 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset16(&mut self) -> EPNDDINTSET16_W {
        EPNDDINTSET16_W { w: self }
    }
    #[doc = "Bit 17 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset17(&mut self) -> EPNDDINTSET17_W {
        EPNDDINTSET17_W { w: self }
    }
    #[doc = "Bit 18 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset18(&mut self) -> EPNDDINTSET18_W {
        EPNDDINTSET18_W { w: self }
    }
    #[doc = "Bit 19 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset19(&mut self) -> EPNDDINTSET19_W {
        EPNDDINTSET19_W { w: self }
    }
    #[doc = "Bit 20 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset20(&mut self) -> EPNDDINTSET20_W {
        EPNDDINTSET20_W { w: self }
    }
    #[doc = "Bit 21 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset21(&mut self) -> EPNDDINTSET21_W {
        EPNDDINTSET21_W { w: self }
    }
    #[doc = "Bit 22 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset22(&mut self) -> EPNDDINTSET22_W {
        EPNDDINTSET22_W { w: self }
    }
    #[doc = "Bit 23 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset23(&mut self) -> EPNDDINTSET23_W {
        EPNDDINTSET23_W { w: self }
    }
    #[doc = "Bit 24 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset24(&mut self) -> EPNDDINTSET24_W {
        EPNDDINTSET24_W { w: self }
    }
    #[doc = "Bit 25 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset25(&mut self) -> EPNDDINTSET25_W {
        EPNDDINTSET25_W { w: self }
    }
    #[doc = "Bit 26 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset26(&mut self) -> EPNDDINTSET26_W {
        EPNDDINTSET26_W { w: self }
    }
    #[doc = "Bit 27 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset27(&mut self) -> EPNDDINTSET27_W {
        EPNDDINTSET27_W { w: self }
    }
    #[doc = "Bit 28 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset28(&mut self) -> EPNDDINTSET28_W {
        EPNDDINTSET28_W { w: self }
    }
    #[doc = "Bit 29 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset29(&mut self) -> EPNDDINTSET29_W {
        EPNDDINTSET29_W { w: self }
    }
    #[doc = "Bit 30 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset30(&mut self) -> EPNDDINTSET30_W {
        EPNDDINTSET30_W { w: self }
    }
    #[doc = "Bit 31 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintset31(&mut self) -> EPNDDINTSET31_W {
        EPNDDINTSET31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB New DD Request Interrupt Set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nddrintset](index.html) module"]
pub struct NDDRINTSET_SPEC;
impl crate::RegisterSpec for NDDRINTSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [nddrintset::W](W) writer structure"]
impl crate::Writable for NDDRINTSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NDDRINTSET to value 0"]
impl crate::Resettable for NDDRINTSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
