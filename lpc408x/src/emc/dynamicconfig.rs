#[doc = "Register `DYNAMICCONFIG%s` reader"]
pub struct R(crate::R<DYNAMICCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DYNAMICCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DYNAMICCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DYNAMICCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DYNAMICCONFIG%s` writer"]
pub struct W(crate::W<DYNAMICCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DYNAMICCONFIG_SPEC>;
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
impl From<crate::W<DYNAMICCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DYNAMICCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Memory device.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MD_A {
    #[doc = "0: SDRAM (POR reset value)."]
    SDRAM_POR_RESET_VAL = 0,
    #[doc = "1: Low-power SDRAM."]
    LOW_POWER_SDRAM_ = 1,
    #[doc = "2: Reserved."]
    RESERVED_ = 2,
    #[doc = "3: Reserved."]
    RESERVED_ = 3,
}
impl From<MD_A> for u8 {
    #[inline(always)]
    fn from(variant: MD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MD` reader - Memory device."]
pub struct MD_R(crate::FieldReader<u8, MD_A>);
impl MD_R {
    pub(crate) fn new(bits: u8) -> Self {
        MD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MD_A {
        match self.bits {
            0 => MD_A::SDRAM_POR_RESET_VAL,
            1 => MD_A::LOW_POWER_SDRAM_,
            2 => MD_A::RESERVED_,
            3 => MD_A::RESERVED_,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SDRAM_POR_RESET_VAL`"]
    #[inline(always)]
    pub fn is_sdram_por_reset_val(&self) -> bool {
        **self == MD_A::SDRAM_POR_RESET_VAL
    }
    #[doc = "Checks if the value of the field is `LOW_POWER_SDRAM_`"]
    #[inline(always)]
    pub fn is_low_power_sdram_(&self) -> bool {
        **self == MD_A::LOW_POWER_SDRAM_
    }
    #[doc = "Checks if the value of the field is `RESERVED_`"]
    #[inline(always)]
    pub fn is_reserved_(&self) -> bool {
        **self == MD_A::RESERVED_
    }
    #[doc = "Checks if the value of the field is `RESERVED_`"]
    #[inline(always)]
    pub fn is_reserved_(&self) -> bool {
        **self == MD_A::RESERVED_
    }
}
impl core::ops::Deref for MD_R {
    type Target = crate::FieldReader<u8, MD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MD` writer - Memory device."]
pub struct MD_W<'a> {
    w: &'a mut W,
}
impl<'a> MD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "SDRAM (POR reset value)."]
    #[inline(always)]
    pub fn sdram_por_reset_val(self) -> &'a mut W {
        self.variant(MD_A::SDRAM_POR_RESET_VAL)
    }
    #[doc = "Low-power SDRAM."]
    #[inline(always)]
    pub fn low_power_sdram_(self) -> &'a mut W {
        self.variant(MD_A::LOW_POWER_SDRAM_)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn reserved_(self) -> &'a mut W {
        self.variant(MD_A::RESERVED_)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn reserved_(self) -> &'a mut W {
        self.variant(MD_A::RESERVED_)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `AM0` reader - See Table 133. 000000 = reset value.\\[1\\]"]
pub struct AM0_R(crate::FieldReader<u8, u8>);
impl AM0_R {
    pub(crate) fn new(bits: u8) -> Self {
        AM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AM0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AM0` writer - See Table 133. 000000 = reset value.\\[1\\]"]
pub struct AM0_W<'a> {
    w: &'a mut W,
}
impl<'a> AM0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 7)) | ((value as u32 & 0x3f) << 7);
        self.w
    }
}
#[doc = "Field `AM1` reader - See Table 133. 0 = reset value."]
pub struct AM1_R(crate::FieldReader<bool, bool>);
impl AM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        AM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AM1` writer - See Table 133. 0 = reset value."]
pub struct AM1_W<'a> {
    w: &'a mut W,
}
impl<'a> AM1_W<'a> {
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
#[doc = "Buffer enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B_A {
    #[doc = "0: Buffer disabled for accesses to this chip select (POR reset value)."]
    BUFFER_DISABLED_FOR_ = 0,
    #[doc = "1: Buffer enabled for accesses to this chip select.\\[2\\]"]
    BUFFER_ENABLED_FOR_A = 1,
}
impl From<B_A> for bool {
    #[inline(always)]
    fn from(variant: B_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B` reader - Buffer enable."]
pub struct B_R(crate::FieldReader<bool, B_A>);
impl B_R {
    pub(crate) fn new(bits: bool) -> Self {
        B_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B_A {
        match self.bits {
            false => B_A::BUFFER_DISABLED_FOR_,
            true => B_A::BUFFER_ENABLED_FOR_A,
        }
    }
    #[doc = "Checks if the value of the field is `BUFFER_DISABLED_FOR_`"]
    #[inline(always)]
    pub fn is_buffer_disabled_for_(&self) -> bool {
        **self == B_A::BUFFER_DISABLED_FOR_
    }
    #[doc = "Checks if the value of the field is `BUFFER_ENABLED_FOR_A`"]
    #[inline(always)]
    pub fn is_buffer_enabled_for_a(&self) -> bool {
        **self == B_A::BUFFER_ENABLED_FOR_A
    }
}
impl core::ops::Deref for B_R {
    type Target = crate::FieldReader<bool, B_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B` writer - Buffer enable."]
pub struct B_W<'a> {
    w: &'a mut W,
}
impl<'a> B_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: B_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Buffer disabled for accesses to this chip select (POR reset value)."]
    #[inline(always)]
    pub fn buffer_disabled_for_(self) -> &'a mut W {
        self.variant(B_A::BUFFER_DISABLED_FOR_)
    }
    #[doc = "Buffer enabled for accesses to this chip select.\\[2\\]"]
    #[inline(always)]
    pub fn buffer_enabled_for_a(self) -> &'a mut W {
        self.variant(B_A::BUFFER_ENABLED_FOR_A)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Write protect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P_A {
    #[doc = "0: Writes not protected (POR reset value)."]
    WRITES_NOT_PROTECTED = 0,
    #[doc = "1: Writes protected."]
    WRITES_PROTECTED_ = 1,
}
impl From<P_A> for bool {
    #[inline(always)]
    fn from(variant: P_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P` reader - Write protect."]
pub struct P_R(crate::FieldReader<bool, P_A>);
impl P_R {
    pub(crate) fn new(bits: bool) -> Self {
        P_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P_A {
        match self.bits {
            false => P_A::WRITES_NOT_PROTECTED,
            true => P_A::WRITES_PROTECTED_,
        }
    }
    #[doc = "Checks if the value of the field is `WRITES_NOT_PROTECTED`"]
    #[inline(always)]
    pub fn is_writes_not_protected(&self) -> bool {
        **self == P_A::WRITES_NOT_PROTECTED
    }
    #[doc = "Checks if the value of the field is `WRITES_PROTECTED_`"]
    #[inline(always)]
    pub fn is_writes_protected_(&self) -> bool {
        **self == P_A::WRITES_PROTECTED_
    }
}
impl core::ops::Deref for P_R {
    type Target = crate::FieldReader<bool, P_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P` writer - Write protect."]
pub struct P_W<'a> {
    w: &'a mut W,
}
impl<'a> P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Writes not protected (POR reset value)."]
    #[inline(always)]
    pub fn writes_not_protected(self) -> &'a mut W {
        self.variant(P_A::WRITES_NOT_PROTECTED)
    }
    #[doc = "Writes protected."]
    #[inline(always)]
    pub fn writes_protected_(self) -> &'a mut W {
        self.variant(P_A::WRITES_PROTECTED_)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:4 - Memory device."]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 7:12 - See Table 133. 000000 = reset value.\\[1\\]"]
    #[inline(always)]
    pub fn am0(&self) -> AM0_R {
        AM0_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - See Table 133. 0 = reset value."]
    #[inline(always)]
    pub fn am1(&self) -> AM1_R {
        AM1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Buffer enable."]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Write protect."]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 3:4 - Memory device."]
    #[inline(always)]
    pub fn md(&mut self) -> MD_W {
        MD_W { w: self }
    }
    #[doc = "Bits 7:12 - See Table 133. 000000 = reset value.\\[1\\]"]
    #[inline(always)]
    pub fn am0(&mut self) -> AM0_W {
        AM0_W { w: self }
    }
    #[doc = "Bit 14 - See Table 133. 0 = reset value."]
    #[inline(always)]
    pub fn am1(&mut self) -> AM1_W {
        AM1_W { w: self }
    }
    #[doc = "Bit 19 - Buffer enable."]
    #[inline(always)]
    pub fn b(&mut self) -> B_W {
        B_W { w: self }
    }
    #[doc = "Bit 20 - Write protect."]
    #[inline(always)]
    pub fn p(&mut self) -> P_W {
        P_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration information for EMC_DYCS0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicconfig](index.html) module"]
pub struct DYNAMICCONFIG_SPEC;
impl crate::RegisterSpec for DYNAMICCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dynamicconfig::R](R) reader structure"]
impl crate::Readable for DYNAMICCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dynamicconfig::W](W) writer structure"]
impl crate::Writable for DYNAMICCONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DYNAMICCONFIG%s to value 0"]
impl crate::Resettable for DYNAMICCONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
