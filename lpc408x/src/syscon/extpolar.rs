#[doc = "Register `EXTPOLAR` reader"]
pub struct R(crate::R<EXTPOLAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTPOLAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTPOLAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTPOLAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTPOLAR` writer"]
pub struct W(crate::W<EXTPOLAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTPOLAR_SPEC>;
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
impl From<crate::W<EXTPOLAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTPOLAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "External interrupt polarity for EINT0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTPOLAR0_A {
    #[doc = "0: Low-active or falling-edge sensitive (depending on EXTMODE0)."]
    LOW_ACTIVE_OR_FALLIN = 0,
    #[doc = "1: High-active or rising-edge sensitive (depending on EXTMODE0)."]
    HIGH_ACTIVE_OR_RISIN = 1,
}
impl From<EXTPOLAR0_A> for bool {
    #[inline(always)]
    fn from(variant: EXTPOLAR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTPOLAR0` reader - External interrupt polarity for EINT0."]
pub struct EXTPOLAR0_R(crate::FieldReader<bool, EXTPOLAR0_A>);
impl EXTPOLAR0_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTPOLAR0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTPOLAR0_A {
        match self.bits {
            false => EXTPOLAR0_A::LOW_ACTIVE_OR_FALLIN,
            true => EXTPOLAR0_A::HIGH_ACTIVE_OR_RISIN,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_ACTIVE_OR_FALLIN`"]
    #[inline(always)]
    pub fn is_low_active_or_fallin(&self) -> bool {
        **self == EXTPOLAR0_A::LOW_ACTIVE_OR_FALLIN
    }
    #[doc = "Checks if the value of the field is `HIGH_ACTIVE_OR_RISIN`"]
    #[inline(always)]
    pub fn is_high_active_or_risin(&self) -> bool {
        **self == EXTPOLAR0_A::HIGH_ACTIVE_OR_RISIN
    }
}
impl core::ops::Deref for EXTPOLAR0_R {
    type Target = crate::FieldReader<bool, EXTPOLAR0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTPOLAR0` writer - External interrupt polarity for EINT0."]
pub struct EXTPOLAR0_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTPOLAR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTPOLAR0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low-active or falling-edge sensitive (depending on EXTMODE0)."]
    #[inline(always)]
    pub fn low_active_or_fallin(self) -> &'a mut W {
        self.variant(EXTPOLAR0_A::LOW_ACTIVE_OR_FALLIN)
    }
    #[doc = "High-active or rising-edge sensitive (depending on EXTMODE0)."]
    #[inline(always)]
    pub fn high_active_or_risin(self) -> &'a mut W {
        self.variant(EXTPOLAR0_A::HIGH_ACTIVE_OR_RISIN)
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
#[doc = "External interrupt polarity for EINT1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTPOLAR1_A {
    #[doc = "0: Low-active or falling-edge sensitive (depending on EXTMODE1)."]
    LOW_ACTIVE_OR_FALLIN = 0,
    #[doc = "1: High-active or rising-edge sensitive (depending on EXTMODE1)."]
    HIGH_ACTIVE_OR_RISIN = 1,
}
impl From<EXTPOLAR1_A> for bool {
    #[inline(always)]
    fn from(variant: EXTPOLAR1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTPOLAR1` reader - External interrupt polarity for EINT1."]
pub struct EXTPOLAR1_R(crate::FieldReader<bool, EXTPOLAR1_A>);
impl EXTPOLAR1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTPOLAR1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTPOLAR1_A {
        match self.bits {
            false => EXTPOLAR1_A::LOW_ACTIVE_OR_FALLIN,
            true => EXTPOLAR1_A::HIGH_ACTIVE_OR_RISIN,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_ACTIVE_OR_FALLIN`"]
    #[inline(always)]
    pub fn is_low_active_or_fallin(&self) -> bool {
        **self == EXTPOLAR1_A::LOW_ACTIVE_OR_FALLIN
    }
    #[doc = "Checks if the value of the field is `HIGH_ACTIVE_OR_RISIN`"]
    #[inline(always)]
    pub fn is_high_active_or_risin(&self) -> bool {
        **self == EXTPOLAR1_A::HIGH_ACTIVE_OR_RISIN
    }
}
impl core::ops::Deref for EXTPOLAR1_R {
    type Target = crate::FieldReader<bool, EXTPOLAR1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTPOLAR1` writer - External interrupt polarity for EINT1."]
pub struct EXTPOLAR1_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTPOLAR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTPOLAR1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low-active or falling-edge sensitive (depending on EXTMODE1)."]
    #[inline(always)]
    pub fn low_active_or_fallin(self) -> &'a mut W {
        self.variant(EXTPOLAR1_A::LOW_ACTIVE_OR_FALLIN)
    }
    #[doc = "High-active or rising-edge sensitive (depending on EXTMODE1)."]
    #[inline(always)]
    pub fn high_active_or_risin(self) -> &'a mut W {
        self.variant(EXTPOLAR1_A::HIGH_ACTIVE_OR_RISIN)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "External interrupt polarity for EINT2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTPOLAR2_A {
    #[doc = "0: Low-active or falling-edge sensitive (depending on EXTMODE2)."]
    LOW_ACTIVE_OR_FALLIN = 0,
    #[doc = "1: High-active or rising-edge sensitive (depending on EXTMODE2)."]
    HIGH_ACTIVE_OR_RISIN = 1,
}
impl From<EXTPOLAR2_A> for bool {
    #[inline(always)]
    fn from(variant: EXTPOLAR2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTPOLAR2` reader - External interrupt polarity for EINT2."]
pub struct EXTPOLAR2_R(crate::FieldReader<bool, EXTPOLAR2_A>);
impl EXTPOLAR2_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTPOLAR2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTPOLAR2_A {
        match self.bits {
            false => EXTPOLAR2_A::LOW_ACTIVE_OR_FALLIN,
            true => EXTPOLAR2_A::HIGH_ACTIVE_OR_RISIN,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_ACTIVE_OR_FALLIN`"]
    #[inline(always)]
    pub fn is_low_active_or_fallin(&self) -> bool {
        **self == EXTPOLAR2_A::LOW_ACTIVE_OR_FALLIN
    }
    #[doc = "Checks if the value of the field is `HIGH_ACTIVE_OR_RISIN`"]
    #[inline(always)]
    pub fn is_high_active_or_risin(&self) -> bool {
        **self == EXTPOLAR2_A::HIGH_ACTIVE_OR_RISIN
    }
}
impl core::ops::Deref for EXTPOLAR2_R {
    type Target = crate::FieldReader<bool, EXTPOLAR2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTPOLAR2` writer - External interrupt polarity for EINT2."]
pub struct EXTPOLAR2_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTPOLAR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTPOLAR2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low-active or falling-edge sensitive (depending on EXTMODE2)."]
    #[inline(always)]
    pub fn low_active_or_fallin(self) -> &'a mut W {
        self.variant(EXTPOLAR2_A::LOW_ACTIVE_OR_FALLIN)
    }
    #[doc = "High-active or rising-edge sensitive (depending on EXTMODE2)."]
    #[inline(always)]
    pub fn high_active_or_risin(self) -> &'a mut W {
        self.variant(EXTPOLAR2_A::HIGH_ACTIVE_OR_RISIN)
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
#[doc = "External interrupt polarity for EINT3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTPOLAR3_A {
    #[doc = "0: Low-active or falling-edge sensitive (depending on EXTMODE3)."]
    LOW_ACTIVE_OR_FALLIN = 0,
    #[doc = "1: High-active or rising-edge sensitive (depending on EXTMODE3)."]
    HIGH_ACTIVE_OR_RISIN = 1,
}
impl From<EXTPOLAR3_A> for bool {
    #[inline(always)]
    fn from(variant: EXTPOLAR3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTPOLAR3` reader - External interrupt polarity for EINT3."]
pub struct EXTPOLAR3_R(crate::FieldReader<bool, EXTPOLAR3_A>);
impl EXTPOLAR3_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTPOLAR3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTPOLAR3_A {
        match self.bits {
            false => EXTPOLAR3_A::LOW_ACTIVE_OR_FALLIN,
            true => EXTPOLAR3_A::HIGH_ACTIVE_OR_RISIN,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_ACTIVE_OR_FALLIN`"]
    #[inline(always)]
    pub fn is_low_active_or_fallin(&self) -> bool {
        **self == EXTPOLAR3_A::LOW_ACTIVE_OR_FALLIN
    }
    #[doc = "Checks if the value of the field is `HIGH_ACTIVE_OR_RISIN`"]
    #[inline(always)]
    pub fn is_high_active_or_risin(&self) -> bool {
        **self == EXTPOLAR3_A::HIGH_ACTIVE_OR_RISIN
    }
}
impl core::ops::Deref for EXTPOLAR3_R {
    type Target = crate::FieldReader<bool, EXTPOLAR3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTPOLAR3` writer - External interrupt polarity for EINT3."]
pub struct EXTPOLAR3_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTPOLAR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTPOLAR3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low-active or falling-edge sensitive (depending on EXTMODE3)."]
    #[inline(always)]
    pub fn low_active_or_fallin(self) -> &'a mut W {
        self.variant(EXTPOLAR3_A::LOW_ACTIVE_OR_FALLIN)
    }
    #[doc = "High-active or rising-edge sensitive (depending on EXTMODE3)."]
    #[inline(always)]
    pub fn high_active_or_risin(self) -> &'a mut W {
        self.variant(EXTPOLAR3_A::HIGH_ACTIVE_OR_RISIN)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - External interrupt polarity for EINT0."]
    #[inline(always)]
    pub fn extpolar0(&self) -> EXTPOLAR0_R {
        EXTPOLAR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - External interrupt polarity for EINT1."]
    #[inline(always)]
    pub fn extpolar1(&self) -> EXTPOLAR1_R {
        EXTPOLAR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External interrupt polarity for EINT2."]
    #[inline(always)]
    pub fn extpolar2(&self) -> EXTPOLAR2_R {
        EXTPOLAR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External interrupt polarity for EINT3."]
    #[inline(always)]
    pub fn extpolar3(&self) -> EXTPOLAR3_R {
        EXTPOLAR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External interrupt polarity for EINT0."]
    #[inline(always)]
    pub fn extpolar0(&mut self) -> EXTPOLAR0_W {
        EXTPOLAR0_W { w: self }
    }
    #[doc = "Bit 1 - External interrupt polarity for EINT1."]
    #[inline(always)]
    pub fn extpolar1(&mut self) -> EXTPOLAR1_W {
        EXTPOLAR1_W { w: self }
    }
    #[doc = "Bit 2 - External interrupt polarity for EINT2."]
    #[inline(always)]
    pub fn extpolar2(&mut self) -> EXTPOLAR2_W {
        EXTPOLAR2_W { w: self }
    }
    #[doc = "Bit 3 - External interrupt polarity for EINT3."]
    #[inline(always)]
    pub fn extpolar3(&mut self) -> EXTPOLAR3_W {
        EXTPOLAR3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Interrupt Polarity Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extpolar](index.html) module"]
pub struct EXTPOLAR_SPEC;
impl crate::RegisterSpec for EXTPOLAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extpolar::R](R) reader structure"]
impl crate::Readable for EXTPOLAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extpolar::W](W) writer structure"]
impl crate::Writable for EXTPOLAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTPOLAR to value 0"]
impl crate::Resettable for EXTPOLAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
