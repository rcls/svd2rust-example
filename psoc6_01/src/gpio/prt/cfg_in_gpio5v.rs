#[doc = "Register `CFG_IN_GPIO5V` reader"]
pub struct R(crate::R<CFG_IN_GPIO5V_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_IN_GPIO5V_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_IN_GPIO5V_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_IN_GPIO5V_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_IN_GPIO5V` writer"]
pub struct W(crate::W<CFG_IN_GPIO5V_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_IN_GPIO5V_SPEC>;
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
impl From<crate::W<CFG_IN_GPIO5V_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_IN_GPIO5V_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Configures the input buffer mode (trip points and hysteresis) for GPIO5V upper bit. Lower bit is still selected by CFG_IN.VTRIP_SEL0_0 field. 0: input buffer is not compatible with automotive. 1: input buffer is compatible with automotive. Use CFG_IN.VTRIP_SEL0_0 fields set as CMOS only when this bit needs to be set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VTRIP_SEL0_1_A {
    #[doc = "0: Input buffer not compatible with automotive (elevated Vil) interfaces."]
    DISABLE = 0,
    #[doc = "1: Input buffer compatible with automotive (elevated Vil) interfaces."]
    AUTO = 1,
}
impl From<VTRIP_SEL0_1_A> for bool {
    #[inline(always)]
    fn from(variant: VTRIP_SEL0_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VTRIP_SEL0_1` reader - Configures the input buffer mode (trip points and hysteresis) for GPIO5V upper bit. Lower bit is still selected by CFG_IN.VTRIP_SEL0_0 field. 0: input buffer is not compatible with automotive. 1: input buffer is compatible with automotive. Use CFG_IN.VTRIP_SEL0_0 fields set as CMOS only when this bit needs to be set."]
pub struct VTRIP_SEL0_1_R(crate::FieldReader<bool, VTRIP_SEL0_1_A>);
impl VTRIP_SEL0_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        VTRIP_SEL0_1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VTRIP_SEL0_1_A {
        match self.bits {
            false => VTRIP_SEL0_1_A::DISABLE,
            true => VTRIP_SEL0_1_A::AUTO,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == VTRIP_SEL0_1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        **self == VTRIP_SEL0_1_A::AUTO
    }
}
impl core::ops::Deref for VTRIP_SEL0_1_R {
    type Target = crate::FieldReader<bool, VTRIP_SEL0_1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTRIP_SEL0_1` writer - Configures the input buffer mode (trip points and hysteresis) for GPIO5V upper bit. Lower bit is still selected by CFG_IN.VTRIP_SEL0_0 field. 0: input buffer is not compatible with automotive. 1: input buffer is compatible with automotive. Use CFG_IN.VTRIP_SEL0_0 fields set as CMOS only when this bit needs to be set."]
pub struct VTRIP_SEL0_1_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL0_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VTRIP_SEL0_1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input buffer not compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(VTRIP_SEL0_1_A::DISABLE)
    }
    #[doc = "Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(VTRIP_SEL0_1_A::AUTO)
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
#[doc = "Field `VTRIP_SEL1_1` reader - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub struct VTRIP_SEL1_1_R(crate::FieldReader<bool, bool>);
impl VTRIP_SEL1_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        VTRIP_SEL1_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VTRIP_SEL1_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTRIP_SEL1_1` writer - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub struct VTRIP_SEL1_1_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL1_1_W<'a> {
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
#[doc = "Field `VTRIP_SEL2_1` reader - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub struct VTRIP_SEL2_1_R(crate::FieldReader<bool, bool>);
impl VTRIP_SEL2_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        VTRIP_SEL2_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VTRIP_SEL2_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTRIP_SEL2_1` writer - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub struct VTRIP_SEL2_1_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL2_1_W<'a> {
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
#[doc = "Field `VTRIP_SEL3_1` reader - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub struct VTRIP_SEL3_1_R(crate::FieldReader<bool, bool>);
impl VTRIP_SEL3_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        VTRIP_SEL3_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VTRIP_SEL3_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTRIP_SEL3_1` writer - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub struct VTRIP_SEL3_1_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL3_1_W<'a> {
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
#[doc = "Field `VTRIP_SEL4_1` reader - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub struct VTRIP_SEL4_1_R(crate::FieldReader<bool, bool>);
impl VTRIP_SEL4_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        VTRIP_SEL4_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VTRIP_SEL4_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTRIP_SEL4_1` writer - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub struct VTRIP_SEL4_1_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL4_1_W<'a> {
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
#[doc = "Field `VTRIP_SEL5_1` reader - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub struct VTRIP_SEL5_1_R(crate::FieldReader<bool, bool>);
impl VTRIP_SEL5_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        VTRIP_SEL5_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VTRIP_SEL5_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTRIP_SEL5_1` writer - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub struct VTRIP_SEL5_1_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL5_1_W<'a> {
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
#[doc = "Field `VTRIP_SEL6_1` reader - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub struct VTRIP_SEL6_1_R(crate::FieldReader<bool, bool>);
impl VTRIP_SEL6_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        VTRIP_SEL6_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VTRIP_SEL6_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTRIP_SEL6_1` writer - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub struct VTRIP_SEL6_1_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL6_1_W<'a> {
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
#[doc = "Field `VTRIP_SEL7_1` reader - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub struct VTRIP_SEL7_1_R(crate::FieldReader<bool, bool>);
impl VTRIP_SEL7_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        VTRIP_SEL7_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VTRIP_SEL7_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTRIP_SEL7_1` writer - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub struct VTRIP_SEL7_1_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL7_1_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Configures the input buffer mode (trip points and hysteresis) for GPIO5V upper bit. Lower bit is still selected by CFG_IN.VTRIP_SEL0_0 field. 0: input buffer is not compatible with automotive. 1: input buffer is compatible with automotive. Use CFG_IN.VTRIP_SEL0_0 fields set as CMOS only when this bit needs to be set."]
    #[inline(always)]
    pub fn vtrip_sel0_1(&self) -> VTRIP_SEL0_1_R {
        VTRIP_SEL0_1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel1_1(&self) -> VTRIP_SEL1_1_R {
        VTRIP_SEL1_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel2_1(&self) -> VTRIP_SEL2_1_R {
        VTRIP_SEL2_1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel3_1(&self) -> VTRIP_SEL3_1_R {
        VTRIP_SEL3_1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel4_1(&self) -> VTRIP_SEL4_1_R {
        VTRIP_SEL4_1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel5_1(&self) -> VTRIP_SEL5_1_R {
        VTRIP_SEL5_1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel6_1(&self) -> VTRIP_SEL6_1_R {
        VTRIP_SEL6_1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel7_1(&self) -> VTRIP_SEL7_1_R {
        VTRIP_SEL7_1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configures the input buffer mode (trip points and hysteresis) for GPIO5V upper bit. Lower bit is still selected by CFG_IN.VTRIP_SEL0_0 field. 0: input buffer is not compatible with automotive. 1: input buffer is compatible with automotive. Use CFG_IN.VTRIP_SEL0_0 fields set as CMOS only when this bit needs to be set."]
    #[inline(always)]
    pub fn vtrip_sel0_1(&mut self) -> VTRIP_SEL0_1_W {
        VTRIP_SEL0_1_W { w: self }
    }
    #[doc = "Bit 1 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel1_1(&mut self) -> VTRIP_SEL1_1_W {
        VTRIP_SEL1_1_W { w: self }
    }
    #[doc = "Bit 2 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel2_1(&mut self) -> VTRIP_SEL2_1_W {
        VTRIP_SEL2_1_W { w: self }
    }
    #[doc = "Bit 3 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel3_1(&mut self) -> VTRIP_SEL3_1_W {
        VTRIP_SEL3_1_W { w: self }
    }
    #[doc = "Bit 4 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel4_1(&mut self) -> VTRIP_SEL4_1_W {
        VTRIP_SEL4_1_W { w: self }
    }
    #[doc = "Bit 5 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel5_1(&mut self) -> VTRIP_SEL5_1_W {
        VTRIP_SEL5_1_W { w: self }
    }
    #[doc = "Bit 6 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel6_1(&mut self) -> VTRIP_SEL6_1_W {
        VTRIP_SEL6_1_W { w: self }
    }
    #[doc = "Bit 7 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel7_1(&mut self) -> VTRIP_SEL7_1_W {
        VTRIP_SEL7_1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port GPIO5V input buffer configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_in_gpio5v](index.html) module"]
pub struct CFG_IN_GPIO5V_SPEC;
impl crate::RegisterSpec for CFG_IN_GPIO5V_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_in_gpio5v::R](R) reader structure"]
impl crate::Readable for CFG_IN_GPIO5V_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_in_gpio5v::W](W) writer structure"]
impl crate::Writable for CFG_IN_GPIO5V_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG_IN_GPIO5V to value 0"]
impl crate::Resettable for CFG_IN_GPIO5V_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
