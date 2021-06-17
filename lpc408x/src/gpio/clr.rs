#[doc = "Register `CLR%s` writer"]
pub struct W(crate::W<CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLR_SPEC>;
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
impl From<crate::W<CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCLR0` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR0_W<'a> {
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
#[doc = "Field `PCLR1` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR1_W<'a> {
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
#[doc = "Field `PCLR2` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR2_W<'a> {
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
#[doc = "Field `PCLR3` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR3_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR3_W<'a> {
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
#[doc = "Field `PCLR4` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR4_W<'a> {
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
#[doc = "Field `PCLR5` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR5_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR5_W<'a> {
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
#[doc = "Field `PCLR6` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR6_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR6_W<'a> {
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
#[doc = "Field `PCLR7` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR7_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR7_W<'a> {
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
#[doc = "Field `PCLR8` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR8_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR8_W<'a> {
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
#[doc = "Field `PCLR9` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR9_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR9_W<'a> {
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
#[doc = "Field `PCLR10` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR10_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR10_W<'a> {
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
#[doc = "Field `PCLR11` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR11_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR11_W<'a> {
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
#[doc = "Field `PCLR12` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR12_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR12_W<'a> {
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
#[doc = "Field `PCLR13` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR13_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR13_W<'a> {
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
#[doc = "Field `PCLR14` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR14_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR14_W<'a> {
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
#[doc = "Field `PCLR15` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR15_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR15_W<'a> {
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
#[doc = "Field `PCLR16` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR16_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR16_W<'a> {
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
#[doc = "Field `PCLR17` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR17_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR17_W<'a> {
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
#[doc = "Field `PCLR18` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR18_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR18_W<'a> {
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
#[doc = "Field `PCLR19` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR19_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR19_W<'a> {
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
#[doc = "Field `PCLR20` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR20_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR20_W<'a> {
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
#[doc = "Field `PCLR21` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR21_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR21_W<'a> {
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
#[doc = "Field `PCLR22` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR22_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR22_W<'a> {
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
#[doc = "Field `PCLR23` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR23_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR23_W<'a> {
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
#[doc = "Field `PCLR24` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR24_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR24_W<'a> {
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
#[doc = "Field `PCLR25` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR25_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR25_W<'a> {
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
#[doc = "Field `PCLR26` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR26_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR26_W<'a> {
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
#[doc = "Field `PCLR27` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR27_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR27_W<'a> {
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
#[doc = "Field `PCLR28` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR28_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR28_W<'a> {
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
#[doc = "Field `PCLR29` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR29_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR29_W<'a> {
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
#[doc = "Field `PCLR30` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR30_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR30_W<'a> {
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
#[doc = "Field `PCLR31` writer - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
pub struct PCLR31_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLR31_W<'a> {
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
    #[doc = "Bit 0 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr0(&mut self) -> PCLR0_W {
        PCLR0_W { w: self }
    }
    #[doc = "Bit 1 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr1(&mut self) -> PCLR1_W {
        PCLR1_W { w: self }
    }
    #[doc = "Bit 2 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr2(&mut self) -> PCLR2_W {
        PCLR2_W { w: self }
    }
    #[doc = "Bit 3 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr3(&mut self) -> PCLR3_W {
        PCLR3_W { w: self }
    }
    #[doc = "Bit 4 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr4(&mut self) -> PCLR4_W {
        PCLR4_W { w: self }
    }
    #[doc = "Bit 5 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr5(&mut self) -> PCLR5_W {
        PCLR5_W { w: self }
    }
    #[doc = "Bit 6 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr6(&mut self) -> PCLR6_W {
        PCLR6_W { w: self }
    }
    #[doc = "Bit 7 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr7(&mut self) -> PCLR7_W {
        PCLR7_W { w: self }
    }
    #[doc = "Bit 8 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr8(&mut self) -> PCLR8_W {
        PCLR8_W { w: self }
    }
    #[doc = "Bit 9 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr9(&mut self) -> PCLR9_W {
        PCLR9_W { w: self }
    }
    #[doc = "Bit 10 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr10(&mut self) -> PCLR10_W {
        PCLR10_W { w: self }
    }
    #[doc = "Bit 11 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr11(&mut self) -> PCLR11_W {
        PCLR11_W { w: self }
    }
    #[doc = "Bit 12 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr12(&mut self) -> PCLR12_W {
        PCLR12_W { w: self }
    }
    #[doc = "Bit 13 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr13(&mut self) -> PCLR13_W {
        PCLR13_W { w: self }
    }
    #[doc = "Bit 14 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr14(&mut self) -> PCLR14_W {
        PCLR14_W { w: self }
    }
    #[doc = "Bit 15 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr15(&mut self) -> PCLR15_W {
        PCLR15_W { w: self }
    }
    #[doc = "Bit 16 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr16(&mut self) -> PCLR16_W {
        PCLR16_W { w: self }
    }
    #[doc = "Bit 17 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr17(&mut self) -> PCLR17_W {
        PCLR17_W { w: self }
    }
    #[doc = "Bit 18 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr18(&mut self) -> PCLR18_W {
        PCLR18_W { w: self }
    }
    #[doc = "Bit 19 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr19(&mut self) -> PCLR19_W {
        PCLR19_W { w: self }
    }
    #[doc = "Bit 20 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr20(&mut self) -> PCLR20_W {
        PCLR20_W { w: self }
    }
    #[doc = "Bit 21 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr21(&mut self) -> PCLR21_W {
        PCLR21_W { w: self }
    }
    #[doc = "Bit 22 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr22(&mut self) -> PCLR22_W {
        PCLR22_W { w: self }
    }
    #[doc = "Bit 23 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr23(&mut self) -> PCLR23_W {
        PCLR23_W { w: self }
    }
    #[doc = "Bit 24 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr24(&mut self) -> PCLR24_W {
        PCLR24_W { w: self }
    }
    #[doc = "Bit 25 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr25(&mut self) -> PCLR25_W {
        PCLR25_W { w: self }
    }
    #[doc = "Bit 26 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr26(&mut self) -> PCLR26_W {
        PCLR26_W { w: self }
    }
    #[doc = "Bit 27 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr27(&mut self) -> PCLR27_W {
        PCLR27_W { w: self }
    }
    #[doc = "Bit 28 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr28(&mut self) -> PCLR28_W {
        PCLR28_W { w: self }
    }
    #[doc = "Bit 29 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr29(&mut self) -> PCLR29_W {
        PCLR29_W { w: self }
    }
    #[doc = "Bit 30 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr30(&mut self) -> PCLR30_W {
        PCLR30_W { w: self }
    }
    #[doc = "Bit 31 - Fast GPIO output value Clear bits. Bit 0 in xCLR controls pin Px\\[0\\], bit 31 controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pclr31(&mut self) -> PCLR31_W {
        PCLR31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Output Clear register using MASK.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr](index.html) module"]
pub struct CLR_SPEC;
impl crate::RegisterSpec for CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [clr::W](W) writer structure"]
impl crate::Writable for CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLR%s to value 0"]
impl crate::Resettable for CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
