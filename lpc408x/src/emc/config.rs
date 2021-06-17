#[doc = "Register `CONFIG` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG` writer"]
pub struct W(crate::W<CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_SPEC>;
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
impl From<crate::W<CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Endian mode. On power-on reset, the value of the endian bit is 0. All data must be flushed in the EMC before switching between little-endian and big-endian modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EM_A {
    #[doc = "0: Little-endian\r\nmode (POR reset value)."]
    LITTLEENDIAN = 0,
    #[doc = "1: Big-endian\r\nmode."]
    BIGENDIAN = 1,
}
impl From<EM_A> for bool {
    #[inline(always)]
    fn from(variant: EM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM` reader - Endian mode. On power-on reset, the value of the endian bit is 0. All data must be flushed in the EMC before switching between little-endian and big-endian modes."]
pub struct EM_R(crate::FieldReader<bool, EM_A>);
impl EM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EM_A {
        match self.bits {
            false => EM_A::LITTLEENDIAN,
            true => EM_A::BIGENDIAN,
        }
    }
    #[doc = "Checks if the value of the field is `LITTLEENDIAN`"]
    #[inline(always)]
    pub fn is_littleendian(&self) -> bool {
        **self == EM_A::LITTLEENDIAN
    }
    #[doc = "Checks if the value of the field is `BIGENDIAN`"]
    #[inline(always)]
    pub fn is_bigendian(&self) -> bool {
        **self == EM_A::BIGENDIAN
    }
}
impl core::ops::Deref for EM_R {
    type Target = crate::FieldReader<bool, EM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EM` writer - Endian mode. On power-on reset, the value of the endian bit is 0. All data must be flushed in the EMC before switching between little-endian and big-endian modes."]
pub struct EM_W<'a> {
    w: &'a mut W,
}
impl<'a> EM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Little-endian mode (POR reset value)."]
    #[inline(always)]
    pub fn littleendian(self) -> &'a mut W {
        self.variant(EM_A::LITTLEENDIAN)
    }
    #[doc = "Big-endian mode."]
    #[inline(always)]
    pub fn bigendian(self) -> &'a mut W {
        self.variant(EM_A::BIGENDIAN)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "CCLK: CLKOUT ratio. This bit must contain 0 for proper operation of the EMC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKR_A {
    #[doc = "0: 1:1(POR reset value)"]
    PORRESET = 0,
    #[doc = "1: 1:2 (this option is not available on the LPC178x/177x)"]
    DONOTUSE = 1,
}
impl From<CLKR_A> for bool {
    #[inline(always)]
    fn from(variant: CLKR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKR` reader - CCLK: CLKOUT ratio. This bit must contain 0 for proper operation of the EMC."]
pub struct CLKR_R(crate::FieldReader<bool, CLKR_A>);
impl CLKR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKR_A {
        match self.bits {
            false => CLKR_A::PORRESET,
            true => CLKR_A::DONOTUSE,
        }
    }
    #[doc = "Checks if the value of the field is `PORRESET`"]
    #[inline(always)]
    pub fn is_porreset(&self) -> bool {
        **self == CLKR_A::PORRESET
    }
    #[doc = "Checks if the value of the field is `DONOTUSE`"]
    #[inline(always)]
    pub fn is_donotuse(&self) -> bool {
        **self == CLKR_A::DONOTUSE
    }
}
impl core::ops::Deref for CLKR_R {
    type Target = crate::FieldReader<bool, CLKR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKR` writer - CCLK: CLKOUT ratio. This bit must contain 0 for proper operation of the EMC."]
pub struct CLKR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1:1(POR reset value)"]
    #[inline(always)]
    pub fn porreset(self) -> &'a mut W {
        self.variant(CLKR_A::PORRESET)
    }
    #[doc = "1:2 (this option is not available on the LPC178x/177x)"]
    #[inline(always)]
    pub fn donotuse(self) -> &'a mut W {
        self.variant(CLKR_A::DONOTUSE)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Endian mode. On power-on reset, the value of the endian bit is 0. All data must be flushed in the EMC before switching between little-endian and big-endian modes."]
    #[inline(always)]
    pub fn em(&self) -> EM_R {
        EM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - CCLK: CLKOUT ratio. This bit must contain 0 for proper operation of the EMC."]
    #[inline(always)]
    pub fn clkr(&self) -> CLKR_R {
        CLKR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Endian mode. On power-on reset, the value of the endian bit is 0. All data must be flushed in the EMC before switching between little-endian and big-endian modes."]
    #[inline(always)]
    pub fn em(&mut self) -> EM_W {
        EM_W { w: self }
    }
    #[doc = "Bit 8 - CCLK: CLKOUT ratio. This bit must contain 0 for proper operation of the EMC."]
    #[inline(always)]
    pub fn clkr(&mut self) -> CLKR_W {
        CLKR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configures operation of the memory controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config::R](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config::W](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
