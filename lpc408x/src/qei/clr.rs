#[doc = "Register `CLR` writer"]
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
#[doc = "Field `INX_INT` writer - Writing a 1 clears the INX_Int bit in QEIINTSTAT."]
pub struct INX_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> INX_INT_W<'a> {
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
#[doc = "Field `TIM_INT` writer - Writing a 1 clears the TIN_Int bit in QEIINTSTAT."]
pub struct TIM_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM_INT_W<'a> {
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
#[doc = "Field `VELC_INT` writer - Writing a 1 clears the VELC_Int bit in QEIINTSTAT."]
pub struct VELC_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> VELC_INT_W<'a> {
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
#[doc = "Field `DIR_INT` writer - Writing a 1 clears the DIR_Int bit in QEIINTSTAT."]
pub struct DIR_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_INT_W<'a> {
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
#[doc = "Field `ERR_INT` writer - Writing a 1 clears the ERR_Int bit in QEIINTSTAT."]
pub struct ERR_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_INT_W<'a> {
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
#[doc = "Field `ENCLK_INT` writer - Writing a 1 clears the ENCLK_Int bit in QEIINTSTAT."]
pub struct ENCLK_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCLK_INT_W<'a> {
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
#[doc = "Field `POS0_INT` writer - Writing a 1 clears the POS0_Int bit in QEIINTSTAT."]
pub struct POS0_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> POS0_INT_W<'a> {
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
#[doc = "Field `POS1_INT` writer - Writing a 1 clears the POS1_Int bit in QEIINTSTAT."]
pub struct POS1_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> POS1_INT_W<'a> {
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
#[doc = "Field `POS2_INT` writer - Writing a 1 clears the POS2_Int bit in QEIINTSTAT."]
pub struct POS2_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> POS2_INT_W<'a> {
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
#[doc = "Field `REV0_INT` writer - Writing a 1 clears the REV0_Int bit in QEIINTSTAT."]
pub struct REV0_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> REV0_INT_W<'a> {
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
#[doc = "Field `POS0REV_INT` writer - Writing a 1 clears the POS0REV_Int bit in QEIINTSTAT."]
pub struct POS0REV_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> POS0REV_INT_W<'a> {
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
#[doc = "Field `POS1REV_INT` writer - Writing a 1 clears the POS1REV_Int bit in QEIINTSTAT."]
pub struct POS1REV_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> POS1REV_INT_W<'a> {
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
#[doc = "Field `POS2REV_INT` writer - Writing a 1 clears the POS2REV_Int bit in QEIINTSTAT."]
pub struct POS2REV_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> POS2REV_INT_W<'a> {
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
#[doc = "Field `REV1_INT` writer - Writing a 1 clears the REV1_Int bit in QEIINTSTAT."]
pub struct REV1_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> REV1_INT_W<'a> {
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
#[doc = "Field `REV2_INT` writer - Writing a 1 clears the REV2_Int bit in QEIINTSTAT."]
pub struct REV2_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> REV2_INT_W<'a> {
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
#[doc = "Field `MAXPOS_INT` writer - Writing a 1 clears the MAXPOS_Int bit in QEIINTSTAT."]
pub struct MAXPOS_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXPOS_INT_W<'a> {
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
    #[doc = "Bit 0 - Writing a 1 clears the INX_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn inx_int(&mut self) -> INX_INT_W {
        INX_INT_W { w: self }
    }
    #[doc = "Bit 1 - Writing a 1 clears the TIN_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn tim_int(&mut self) -> TIM_INT_W {
        TIM_INT_W { w: self }
    }
    #[doc = "Bit 2 - Writing a 1 clears the VELC_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn velc_int(&mut self) -> VELC_INT_W {
        VELC_INT_W { w: self }
    }
    #[doc = "Bit 3 - Writing a 1 clears the DIR_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn dir_int(&mut self) -> DIR_INT_W {
        DIR_INT_W { w: self }
    }
    #[doc = "Bit 4 - Writing a 1 clears the ERR_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn err_int(&mut self) -> ERR_INT_W {
        ERR_INT_W { w: self }
    }
    #[doc = "Bit 5 - Writing a 1 clears the ENCLK_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn enclk_int(&mut self) -> ENCLK_INT_W {
        ENCLK_INT_W { w: self }
    }
    #[doc = "Bit 6 - Writing a 1 clears the POS0_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn pos0_int(&mut self) -> POS0_INT_W {
        POS0_INT_W { w: self }
    }
    #[doc = "Bit 7 - Writing a 1 clears the POS1_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn pos1_int(&mut self) -> POS1_INT_W {
        POS1_INT_W { w: self }
    }
    #[doc = "Bit 8 - Writing a 1 clears the POS2_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn pos2_int(&mut self) -> POS2_INT_W {
        POS2_INT_W { w: self }
    }
    #[doc = "Bit 9 - Writing a 1 clears the REV0_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn rev0_int(&mut self) -> REV0_INT_W {
        REV0_INT_W { w: self }
    }
    #[doc = "Bit 10 - Writing a 1 clears the POS0REV_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn pos0rev_int(&mut self) -> POS0REV_INT_W {
        POS0REV_INT_W { w: self }
    }
    #[doc = "Bit 11 - Writing a 1 clears the POS1REV_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn pos1rev_int(&mut self) -> POS1REV_INT_W {
        POS1REV_INT_W { w: self }
    }
    #[doc = "Bit 12 - Writing a 1 clears the POS2REV_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn pos2rev_int(&mut self) -> POS2REV_INT_W {
        POS2REV_INT_W { w: self }
    }
    #[doc = "Bit 13 - Writing a 1 clears the REV1_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn rev1_int(&mut self) -> REV1_INT_W {
        REV1_INT_W { w: self }
    }
    #[doc = "Bit 14 - Writing a 1 clears the REV2_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn rev2_int(&mut self) -> REV2_INT_W {
        REV2_INT_W { w: self }
    }
    #[doc = "Bit 15 - Writing a 1 clears the MAXPOS_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn maxpos_int(&mut self) -> MAXPOS_INT_W {
        MAXPOS_INT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt status clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr](index.html) module"]
pub struct CLR_SPEC;
impl crate::RegisterSpec for CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [clr::W](W) writer structure"]
impl crate::Writable for CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLR to value 0"]
impl crate::Resettable for CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
