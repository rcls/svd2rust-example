#[doc = "Register `EXTMODE` reader"]
pub struct R(crate::R<EXTMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTMODE` writer"]
pub struct W(crate::W<EXTMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTMODE_SPEC>;
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
impl From<crate::W<EXTMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Level or edge sensitivity select for EINT0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTMODE0_A {
    #[doc = "0: Level sensitive."]
    LEVEL_SENSITIVE_ = 0,
    #[doc = "1: Edge sensitive."]
    EDGE_SENSITIVE_ = 1,
}
impl From<EXTMODE0_A> for bool {
    #[inline(always)]
    fn from(variant: EXTMODE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTMODE0` reader - Level or edge sensitivity select for EINT0."]
pub struct EXTMODE0_R(crate::FieldReader<bool, EXTMODE0_A>);
impl EXTMODE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTMODE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTMODE0_A {
        match self.bits {
            false => EXTMODE0_A::LEVEL_SENSITIVE_,
            true => EXTMODE0_A::EDGE_SENSITIVE_,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_SENSITIVE_`"]
    #[inline(always)]
    pub fn is_level_sensitive_(&self) -> bool {
        **self == EXTMODE0_A::LEVEL_SENSITIVE_
    }
    #[doc = "Checks if the value of the field is `EDGE_SENSITIVE_`"]
    #[inline(always)]
    pub fn is_edge_sensitive_(&self) -> bool {
        **self == EXTMODE0_A::EDGE_SENSITIVE_
    }
}
impl core::ops::Deref for EXTMODE0_R {
    type Target = crate::FieldReader<bool, EXTMODE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTMODE0` writer - Level or edge sensitivity select for EINT0."]
pub struct EXTMODE0_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMODE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTMODE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Level sensitive."]
    #[inline(always)]
    pub fn level_sensitive_(self) -> &'a mut W {
        self.variant(EXTMODE0_A::LEVEL_SENSITIVE_)
    }
    #[doc = "Edge sensitive."]
    #[inline(always)]
    pub fn edge_sensitive_(self) -> &'a mut W {
        self.variant(EXTMODE0_A::EDGE_SENSITIVE_)
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
#[doc = "Level or edge sensitivity select for EINT1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTMODE1_A {
    #[doc = "0: Level sensitive."]
    LEVEL_SENSITIVE_ = 0,
    #[doc = "1: Edge sensitive."]
    EDGE_SENSITIVE_ = 1,
}
impl From<EXTMODE1_A> for bool {
    #[inline(always)]
    fn from(variant: EXTMODE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTMODE1` reader - Level or edge sensitivity select for EINT1."]
pub struct EXTMODE1_R(crate::FieldReader<bool, EXTMODE1_A>);
impl EXTMODE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTMODE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTMODE1_A {
        match self.bits {
            false => EXTMODE1_A::LEVEL_SENSITIVE_,
            true => EXTMODE1_A::EDGE_SENSITIVE_,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_SENSITIVE_`"]
    #[inline(always)]
    pub fn is_level_sensitive_(&self) -> bool {
        **self == EXTMODE1_A::LEVEL_SENSITIVE_
    }
    #[doc = "Checks if the value of the field is `EDGE_SENSITIVE_`"]
    #[inline(always)]
    pub fn is_edge_sensitive_(&self) -> bool {
        **self == EXTMODE1_A::EDGE_SENSITIVE_
    }
}
impl core::ops::Deref for EXTMODE1_R {
    type Target = crate::FieldReader<bool, EXTMODE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTMODE1` writer - Level or edge sensitivity select for EINT1."]
pub struct EXTMODE1_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMODE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTMODE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Level sensitive."]
    #[inline(always)]
    pub fn level_sensitive_(self) -> &'a mut W {
        self.variant(EXTMODE1_A::LEVEL_SENSITIVE_)
    }
    #[doc = "Edge sensitive."]
    #[inline(always)]
    pub fn edge_sensitive_(self) -> &'a mut W {
        self.variant(EXTMODE1_A::EDGE_SENSITIVE_)
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
#[doc = "Level or edge sensitivity select for EINT2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTMODE2_A {
    #[doc = "0: Level sensitive."]
    LEVEL_SENSITIVE_ = 0,
    #[doc = "1: Edge sensitive."]
    EDGE_SENSITIVE_ = 1,
}
impl From<EXTMODE2_A> for bool {
    #[inline(always)]
    fn from(variant: EXTMODE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTMODE2` reader - Level or edge sensitivity select for EINT2."]
pub struct EXTMODE2_R(crate::FieldReader<bool, EXTMODE2_A>);
impl EXTMODE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTMODE2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTMODE2_A {
        match self.bits {
            false => EXTMODE2_A::LEVEL_SENSITIVE_,
            true => EXTMODE2_A::EDGE_SENSITIVE_,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_SENSITIVE_`"]
    #[inline(always)]
    pub fn is_level_sensitive_(&self) -> bool {
        **self == EXTMODE2_A::LEVEL_SENSITIVE_
    }
    #[doc = "Checks if the value of the field is `EDGE_SENSITIVE_`"]
    #[inline(always)]
    pub fn is_edge_sensitive_(&self) -> bool {
        **self == EXTMODE2_A::EDGE_SENSITIVE_
    }
}
impl core::ops::Deref for EXTMODE2_R {
    type Target = crate::FieldReader<bool, EXTMODE2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTMODE2` writer - Level or edge sensitivity select for EINT2."]
pub struct EXTMODE2_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMODE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTMODE2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Level sensitive."]
    #[inline(always)]
    pub fn level_sensitive_(self) -> &'a mut W {
        self.variant(EXTMODE2_A::LEVEL_SENSITIVE_)
    }
    #[doc = "Edge sensitive."]
    #[inline(always)]
    pub fn edge_sensitive_(self) -> &'a mut W {
        self.variant(EXTMODE2_A::EDGE_SENSITIVE_)
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
#[doc = "Level or edge sensitivity select for EINT3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTMODE3_A {
    #[doc = "0: Level sensitive."]
    LEVEL_SENSITIVE_ = 0,
    #[doc = "1: Edge sensitive."]
    EDGE_SENSITIVE_ = 1,
}
impl From<EXTMODE3_A> for bool {
    #[inline(always)]
    fn from(variant: EXTMODE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTMODE3` reader - Level or edge sensitivity select for EINT3."]
pub struct EXTMODE3_R(crate::FieldReader<bool, EXTMODE3_A>);
impl EXTMODE3_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTMODE3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTMODE3_A {
        match self.bits {
            false => EXTMODE3_A::LEVEL_SENSITIVE_,
            true => EXTMODE3_A::EDGE_SENSITIVE_,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_SENSITIVE_`"]
    #[inline(always)]
    pub fn is_level_sensitive_(&self) -> bool {
        **self == EXTMODE3_A::LEVEL_SENSITIVE_
    }
    #[doc = "Checks if the value of the field is `EDGE_SENSITIVE_`"]
    #[inline(always)]
    pub fn is_edge_sensitive_(&self) -> bool {
        **self == EXTMODE3_A::EDGE_SENSITIVE_
    }
}
impl core::ops::Deref for EXTMODE3_R {
    type Target = crate::FieldReader<bool, EXTMODE3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTMODE3` writer - Level or edge sensitivity select for EINT3."]
pub struct EXTMODE3_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMODE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTMODE3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Level sensitive."]
    #[inline(always)]
    pub fn level_sensitive_(self) -> &'a mut W {
        self.variant(EXTMODE3_A::LEVEL_SENSITIVE_)
    }
    #[doc = "Edge sensitive."]
    #[inline(always)]
    pub fn edge_sensitive_(self) -> &'a mut W {
        self.variant(EXTMODE3_A::EDGE_SENSITIVE_)
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
    #[doc = "Bit 0 - Level or edge sensitivity select for EINT0."]
    #[inline(always)]
    pub fn extmode0(&self) -> EXTMODE0_R {
        EXTMODE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Level or edge sensitivity select for EINT1."]
    #[inline(always)]
    pub fn extmode1(&self) -> EXTMODE1_R {
        EXTMODE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Level or edge sensitivity select for EINT2."]
    #[inline(always)]
    pub fn extmode2(&self) -> EXTMODE2_R {
        EXTMODE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Level or edge sensitivity select for EINT3."]
    #[inline(always)]
    pub fn extmode3(&self) -> EXTMODE3_R {
        EXTMODE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Level or edge sensitivity select for EINT0."]
    #[inline(always)]
    pub fn extmode0(&mut self) -> EXTMODE0_W {
        EXTMODE0_W { w: self }
    }
    #[doc = "Bit 1 - Level or edge sensitivity select for EINT1."]
    #[inline(always)]
    pub fn extmode1(&mut self) -> EXTMODE1_W {
        EXTMODE1_W { w: self }
    }
    #[doc = "Bit 2 - Level or edge sensitivity select for EINT2."]
    #[inline(always)]
    pub fn extmode2(&mut self) -> EXTMODE2_W {
        EXTMODE2_W { w: self }
    }
    #[doc = "Bit 3 - Level or edge sensitivity select for EINT3."]
    #[inline(always)]
    pub fn extmode3(&mut self) -> EXTMODE3_W {
        EXTMODE3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Interrupt Mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmode](index.html) module"]
pub struct EXTMODE_SPEC;
impl crate::RegisterSpec for EXTMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extmode::R](R) reader structure"]
impl crate::Readable for EXTMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extmode::W](W) writer structure"]
impl crate::Writable for EXTMODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTMODE to value 0"]
impl crate::Resettable for EXTMODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
