#[doc = "Register `AFMR` reader"]
pub struct R(crate::R<AFMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFMR` writer"]
pub struct W(crate::W<AFMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFMR_SPEC>;
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
impl From<crate::W<AFMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACCOFF` reader - if AccBP is 0, the Acceptance Filter is not operational. All Rx messages on all CAN buses are ignored."]
pub struct ACCOFF_R(crate::FieldReader<bool, bool>);
impl ACCOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACCOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACCOFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACCOFF` writer - if AccBP is 0, the Acceptance Filter is not operational. All Rx messages on all CAN buses are ignored."]
pub struct ACCOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCOFF_W<'a> {
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
#[doc = "Field `ACCBP` reader - All Rx messages are accepted on enabled CAN controllers. Software must set this bit before modifying the contents of any of the registers described below, and before modifying the contents of Lookup Table RAM in any way other than setting or clearing Disable bits in Standard Identifier entries. When both this bit and AccOff are 0, the Acceptance filter operates to screen received CAN Identifiers."]
pub struct ACCBP_R(crate::FieldReader<bool, bool>);
impl ACCBP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACCBP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACCBP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACCBP` writer - All Rx messages are accepted on enabled CAN controllers. Software must set this bit before modifying the contents of any of the registers described below, and before modifying the contents of Lookup Table RAM in any way other than setting or clearing Disable bits in Standard Identifier entries. When both this bit and AccOff are 0, the Acceptance filter operates to screen received CAN Identifiers."]
pub struct ACCBP_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCBP_W<'a> {
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
#[doc = "FullCAN mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EFCAN_A {
    #[doc = "0: Software must read all messages for all enabled IDs on all enabled CAN buses, from the receiving CAN controllers."]
    SOFTWARE_MUST_READ_A = 0,
    #[doc = "1: The Acceptance Filter itself will take care of receiving and storing messages for selected Standard ID values on selected CAN buses. See Section 21.16 FullCAN mode on page 576."]
    THE_ACCEPTANCE_FILTE = 1,
}
impl From<EFCAN_A> for bool {
    #[inline(always)]
    fn from(variant: EFCAN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EFCAN` reader - FullCAN mode"]
pub struct EFCAN_R(crate::FieldReader<bool, EFCAN_A>);
impl EFCAN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EFCAN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EFCAN_A {
        match self.bits {
            false => EFCAN_A::SOFTWARE_MUST_READ_A,
            true => EFCAN_A::THE_ACCEPTANCE_FILTE,
        }
    }
    #[doc = "Checks if the value of the field is `SOFTWARE_MUST_READ_A`"]
    #[inline(always)]
    pub fn is_software_must_read_a(&self) -> bool {
        **self == EFCAN_A::SOFTWARE_MUST_READ_A
    }
    #[doc = "Checks if the value of the field is `THE_ACCEPTANCE_FILTE`"]
    #[inline(always)]
    pub fn is_the_acceptance_filte(&self) -> bool {
        **self == EFCAN_A::THE_ACCEPTANCE_FILTE
    }
}
impl core::ops::Deref for EFCAN_R {
    type Target = crate::FieldReader<bool, EFCAN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFCAN` writer - FullCAN mode"]
pub struct EFCAN_W<'a> {
    w: &'a mut W,
}
impl<'a> EFCAN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EFCAN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Software must read all messages for all enabled IDs on all enabled CAN buses, from the receiving CAN controllers."]
    #[inline(always)]
    pub fn software_must_read_a(self) -> &'a mut W {
        self.variant(EFCAN_A::SOFTWARE_MUST_READ_A)
    }
    #[doc = "The Acceptance Filter itself will take care of receiving and storing messages for selected Standard ID values on selected CAN buses. See Section 21.16 FullCAN mode on page 576."]
    #[inline(always)]
    pub fn the_acceptance_filte(self) -> &'a mut W {
        self.variant(EFCAN_A::THE_ACCEPTANCE_FILTE)
    }
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
impl R {
    #[doc = "Bit 0 - if AccBP is 0, the Acceptance Filter is not operational. All Rx messages on all CAN buses are ignored."]
    #[inline(always)]
    pub fn accoff(&self) -> ACCOFF_R {
        ACCOFF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - All Rx messages are accepted on enabled CAN controllers. Software must set this bit before modifying the contents of any of the registers described below, and before modifying the contents of Lookup Table RAM in any way other than setting or clearing Disable bits in Standard Identifier entries. When both this bit and AccOff are 0, the Acceptance filter operates to screen received CAN Identifiers."]
    #[inline(always)]
    pub fn accbp(&self) -> ACCBP_R {
        ACCBP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FullCAN mode"]
    #[inline(always)]
    pub fn efcan(&self) -> EFCAN_R {
        EFCAN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - if AccBP is 0, the Acceptance Filter is not operational. All Rx messages on all CAN buses are ignored."]
    #[inline(always)]
    pub fn accoff(&mut self) -> ACCOFF_W {
        ACCOFF_W { w: self }
    }
    #[doc = "Bit 1 - All Rx messages are accepted on enabled CAN controllers. Software must set this bit before modifying the contents of any of the registers described below, and before modifying the contents of Lookup Table RAM in any way other than setting or clearing Disable bits in Standard Identifier entries. When both this bit and AccOff are 0, the Acceptance filter operates to screen received CAN Identifiers."]
    #[inline(always)]
    pub fn accbp(&mut self) -> ACCBP_W {
        ACCBP_W { w: self }
    }
    #[doc = "Bit 2 - FullCAN mode"]
    #[inline(always)]
    pub fn efcan(&mut self) -> EFCAN_W {
        EFCAN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Acceptance Filter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afmr](index.html) module"]
pub struct AFMR_SPEC;
impl crate::RegisterSpec for AFMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afmr::R](R) reader structure"]
impl crate::Readable for AFMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afmr::W](W) writer structure"]
impl crate::Writable for AFMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFMR to value 0"]
impl crate::Resettable for AFMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
