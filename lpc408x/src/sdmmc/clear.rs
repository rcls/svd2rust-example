#[doc = "Register `CLEAR` writer"]
pub struct W(crate::W<CLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLEAR_SPEC>;
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
impl From<crate::W<CLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDCRCFAILCLR` writer - Clears CmdCrcFail flag."]
pub struct CMDCRCFAILCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDCRCFAILCLR_W<'a> {
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
#[doc = "Field `DATACRCFAILCLR` writer - Clears DataCrcFail flag."]
pub struct DATACRCFAILCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DATACRCFAILCLR_W<'a> {
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
#[doc = "Field `CMDTIMEOUTCLR` writer - Clears CmdTimeOut flag."]
pub struct CMDTIMEOUTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDTIMEOUTCLR_W<'a> {
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
#[doc = "Field `DATATIMEOUTCLR` writer - Clears DataTimeOut flag."]
pub struct DATATIMEOUTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DATATIMEOUTCLR_W<'a> {
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
#[doc = "Field `TXUNDERRUNCLR` writer - Clears TxUnderrun flag."]
pub struct TXUNDERRUNCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUNDERRUNCLR_W<'a> {
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
#[doc = "Field `RXOVERRUNCLR` writer - Clears RxOverrun flag."]
pub struct RXOVERRUNCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVERRUNCLR_W<'a> {
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
#[doc = "Field `CMDRESPENDCLR` writer - Clears CmdRespEnd flag."]
pub struct CMDRESPENDCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDRESPENDCLR_W<'a> {
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
#[doc = "Field `CMDSENTCLR` writer - Clears CmdSent flag."]
pub struct CMDSENTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDSENTCLR_W<'a> {
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
#[doc = "Field `DATAENDCLR` writer - Clears DataEnd flag."]
pub struct DATAENDCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAENDCLR_W<'a> {
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
#[doc = "Field `STARTBITERRCLR` writer - Clears StartBitErr flag."]
pub struct STARTBITERRCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTBITERRCLR_W<'a> {
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
#[doc = "Field `DATABLOCKENDCLR` writer - Clears DataBlockEnd flag."]
pub struct DATABLOCKENDCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DATABLOCKENDCLR_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Clears CmdCrcFail flag."]
    #[inline(always)]
    pub fn cmdcrcfailclr(&mut self) -> CMDCRCFAILCLR_W {
        CMDCRCFAILCLR_W { w: self }
    }
    #[doc = "Bit 1 - Clears DataCrcFail flag."]
    #[inline(always)]
    pub fn datacrcfailclr(&mut self) -> DATACRCFAILCLR_W {
        DATACRCFAILCLR_W { w: self }
    }
    #[doc = "Bit 2 - Clears CmdTimeOut flag."]
    #[inline(always)]
    pub fn cmdtimeoutclr(&mut self) -> CMDTIMEOUTCLR_W {
        CMDTIMEOUTCLR_W { w: self }
    }
    #[doc = "Bit 3 - Clears DataTimeOut flag."]
    #[inline(always)]
    pub fn datatimeoutclr(&mut self) -> DATATIMEOUTCLR_W {
        DATATIMEOUTCLR_W { w: self }
    }
    #[doc = "Bit 4 - Clears TxUnderrun flag."]
    #[inline(always)]
    pub fn txunderrunclr(&mut self) -> TXUNDERRUNCLR_W {
        TXUNDERRUNCLR_W { w: self }
    }
    #[doc = "Bit 5 - Clears RxOverrun flag."]
    #[inline(always)]
    pub fn rxoverrunclr(&mut self) -> RXOVERRUNCLR_W {
        RXOVERRUNCLR_W { w: self }
    }
    #[doc = "Bit 6 - Clears CmdRespEnd flag."]
    #[inline(always)]
    pub fn cmdrespendclr(&mut self) -> CMDRESPENDCLR_W {
        CMDRESPENDCLR_W { w: self }
    }
    #[doc = "Bit 7 - Clears CmdSent flag."]
    #[inline(always)]
    pub fn cmdsentclr(&mut self) -> CMDSENTCLR_W {
        CMDSENTCLR_W { w: self }
    }
    #[doc = "Bit 8 - Clears DataEnd flag."]
    #[inline(always)]
    pub fn dataendclr(&mut self) -> DATAENDCLR_W {
        DATAENDCLR_W { w: self }
    }
    #[doc = "Bit 9 - Clears StartBitErr flag."]
    #[inline(always)]
    pub fn startbiterrclr(&mut self) -> STARTBITERRCLR_W {
        STARTBITERRCLR_W { w: self }
    }
    #[doc = "Bit 10 - Clears DataBlockEnd flag."]
    #[inline(always)]
    pub fn datablockendclr(&mut self) -> DATABLOCKENDCLR_W {
        DATABLOCKENDCLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clear](index.html) module"]
pub struct CLEAR_SPEC;
impl crate::RegisterSpec for CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [clear::W](W) writer structure"]
impl crate::Writable for CLEAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLEAR to value 0"]
impl crate::Resettable for CLEAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
