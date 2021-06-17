#[doc = "Register `CFG_IN` reader"]
pub struct R(crate::R<CFG_IN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_IN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_IN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_IN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_IN` writer"]
pub struct W(crate::W<CFG_IN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_IN_SPEC>;
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
impl From<crate::W<CFG_IN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_IN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Configures the pin 0 input buffer mode (trip points and hysteresis)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VTRIP_SEL0_0_A {
    #[doc = "0: S40S: Input buffer compatible with CMOS and I2C interfaces"]
    CMOS = 0,
    #[doc = "1: S40S: Input buffer compatible with TTL and MediaLB interfaces"]
    TTL = 1,
}
impl From<VTRIP_SEL0_0_A> for bool {
    #[inline(always)]
    fn from(variant: VTRIP_SEL0_0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VTRIP_SEL0_0` reader - Configures the pin 0 input buffer mode (trip points and hysteresis)"]
pub struct VTRIP_SEL0_0_R(crate::FieldReader<bool, VTRIP_SEL0_0_A>);
impl VTRIP_SEL0_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        VTRIP_SEL0_0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VTRIP_SEL0_0_A {
        match self.bits {
            false => VTRIP_SEL0_0_A::CMOS,
            true => VTRIP_SEL0_0_A::TTL,
        }
    }
    #[doc = "Checks if the value of the field is `CMOS`"]
    #[inline(always)]
    pub fn is_cmos(&self) -> bool {
        **self == VTRIP_SEL0_0_A::CMOS
    }
    #[doc = "Checks if the value of the field is `TTL`"]
    #[inline(always)]
    pub fn is_ttl(&self) -> bool {
        **self == VTRIP_SEL0_0_A::TTL
    }
}
impl core::ops::Deref for VTRIP_SEL0_0_R {
    type Target = crate::FieldReader<bool, VTRIP_SEL0_0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTRIP_SEL0_0` writer - Configures the pin 0 input buffer mode (trip points and hysteresis)"]
pub struct VTRIP_SEL0_0_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL0_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VTRIP_SEL0_0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "S40S: Input buffer compatible with CMOS and I2C interfaces"]
    #[inline(always)]
    pub fn cmos(self) -> &'a mut W {
        self.variant(VTRIP_SEL0_0_A::CMOS)
    }
    #[doc = "S40S: Input buffer compatible with TTL and MediaLB interfaces"]
    #[inline(always)]
    pub fn ttl(self) -> &'a mut W {
        self.variant(VTRIP_SEL0_0_A::TTL)
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
#[doc = "Field `VTRIP_SEL1_0` reader - Configures the pin 1 input buffer mode (trip points and hysteresis)"]
pub struct VTRIP_SEL1_0_R(crate::FieldReader<bool, bool>);
impl VTRIP_SEL1_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        VTRIP_SEL1_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VTRIP_SEL1_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTRIP_SEL1_0` writer - Configures the pin 1 input buffer mode (trip points and hysteresis)"]
pub struct VTRIP_SEL1_0_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL1_0_W<'a> {
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
#[doc = "Field `VTRIP_SEL2_0` reader - Configures the pin 2 input buffer mode (trip points and hysteresis)"]
pub struct VTRIP_SEL2_0_R(crate::FieldReader<bool, bool>);
impl VTRIP_SEL2_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        VTRIP_SEL2_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VTRIP_SEL2_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTRIP_SEL2_0` writer - Configures the pin 2 input buffer mode (trip points and hysteresis)"]
pub struct VTRIP_SEL2_0_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL2_0_W<'a> {
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
#[doc = "Field `VTRIP_SEL3_0` reader - Configures the pin 3 input buffer mode (trip points and hysteresis)"]
pub struct VTRIP_SEL3_0_R(crate::FieldReader<bool, bool>);
impl VTRIP_SEL3_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        VTRIP_SEL3_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VTRIP_SEL3_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTRIP_SEL3_0` writer - Configures the pin 3 input buffer mode (trip points and hysteresis)"]
pub struct VTRIP_SEL3_0_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL3_0_W<'a> {
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
#[doc = "Field `VTRIP_SEL4_0` reader - Configures the pin 4 input buffer mode (trip points and hysteresis)"]
pub struct VTRIP_SEL4_0_R(crate::FieldReader<bool, bool>);
impl VTRIP_SEL4_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        VTRIP_SEL4_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VTRIP_SEL4_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTRIP_SEL4_0` writer - Configures the pin 4 input buffer mode (trip points and hysteresis)"]
pub struct VTRIP_SEL4_0_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL4_0_W<'a> {
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
#[doc = "Field `VTRIP_SEL5_0` reader - Configures the pin 5 input buffer mode (trip points and hysteresis)"]
pub struct VTRIP_SEL5_0_R(crate::FieldReader<bool, bool>);
impl VTRIP_SEL5_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        VTRIP_SEL5_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VTRIP_SEL5_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTRIP_SEL5_0` writer - Configures the pin 5 input buffer mode (trip points and hysteresis)"]
pub struct VTRIP_SEL5_0_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL5_0_W<'a> {
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
#[doc = "Field `VTRIP_SEL6_0` reader - Configures the pin 6 input buffer mode (trip points and hysteresis)"]
pub struct VTRIP_SEL6_0_R(crate::FieldReader<bool, bool>);
impl VTRIP_SEL6_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        VTRIP_SEL6_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VTRIP_SEL6_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTRIP_SEL6_0` writer - Configures the pin 6 input buffer mode (trip points and hysteresis)"]
pub struct VTRIP_SEL6_0_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL6_0_W<'a> {
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
#[doc = "Field `VTRIP_SEL7_0` reader - Configures the pin 7 input buffer mode (trip points and hysteresis)"]
pub struct VTRIP_SEL7_0_R(crate::FieldReader<bool, bool>);
impl VTRIP_SEL7_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        VTRIP_SEL7_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VTRIP_SEL7_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTRIP_SEL7_0` writer - Configures the pin 7 input buffer mode (trip points and hysteresis)"]
pub struct VTRIP_SEL7_0_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL7_0_W<'a> {
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
    #[doc = "Bit 0 - Configures the pin 0 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel0_0(&self) -> VTRIP_SEL0_0_R {
        VTRIP_SEL0_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Configures the pin 1 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel1_0(&self) -> VTRIP_SEL1_0_R {
        VTRIP_SEL1_0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Configures the pin 2 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel2_0(&self) -> VTRIP_SEL2_0_R {
        VTRIP_SEL2_0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Configures the pin 3 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel3_0(&self) -> VTRIP_SEL3_0_R {
        VTRIP_SEL3_0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Configures the pin 4 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel4_0(&self) -> VTRIP_SEL4_0_R {
        VTRIP_SEL4_0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Configures the pin 5 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel5_0(&self) -> VTRIP_SEL5_0_R {
        VTRIP_SEL5_0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Configures the pin 6 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel6_0(&self) -> VTRIP_SEL6_0_R {
        VTRIP_SEL6_0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Configures the pin 7 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel7_0(&self) -> VTRIP_SEL7_0_R {
        VTRIP_SEL7_0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configures the pin 0 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel0_0(&mut self) -> VTRIP_SEL0_0_W {
        VTRIP_SEL0_0_W { w: self }
    }
    #[doc = "Bit 1 - Configures the pin 1 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel1_0(&mut self) -> VTRIP_SEL1_0_W {
        VTRIP_SEL1_0_W { w: self }
    }
    #[doc = "Bit 2 - Configures the pin 2 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel2_0(&mut self) -> VTRIP_SEL2_0_W {
        VTRIP_SEL2_0_W { w: self }
    }
    #[doc = "Bit 3 - Configures the pin 3 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel3_0(&mut self) -> VTRIP_SEL3_0_W {
        VTRIP_SEL3_0_W { w: self }
    }
    #[doc = "Bit 4 - Configures the pin 4 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel4_0(&mut self) -> VTRIP_SEL4_0_W {
        VTRIP_SEL4_0_W { w: self }
    }
    #[doc = "Bit 5 - Configures the pin 5 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel5_0(&mut self) -> VTRIP_SEL5_0_W {
        VTRIP_SEL5_0_W { w: self }
    }
    #[doc = "Bit 6 - Configures the pin 6 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel6_0(&mut self) -> VTRIP_SEL6_0_W {
        VTRIP_SEL6_0_W { w: self }
    }
    #[doc = "Bit 7 - Configures the pin 7 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel7_0(&mut self) -> VTRIP_SEL7_0_W {
        VTRIP_SEL7_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port input buffer configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_in](index.html) module"]
pub struct CFG_IN_SPEC;
impl crate::RegisterSpec for CFG_IN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_in::R](R) reader structure"]
impl crate::Readable for CFG_IN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_in::W](W) writer structure"]
impl crate::Writable for CFG_IN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG_IN to value 0"]
impl crate::Resettable for CFG_IN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
