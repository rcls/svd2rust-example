#[doc = "Register `DYNAMICRASCAS%s` reader"]
pub struct R(crate::R<DYNAMICRASCAS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DYNAMICRASCAS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DYNAMICRASCAS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DYNAMICRASCAS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DYNAMICRASCAS%s` writer"]
pub struct W(crate::W<DYNAMICRASCAS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DYNAMICRASCAS_SPEC>;
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
impl From<crate::W<DYNAMICRASCAS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DYNAMICRASCAS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "RAS latency (active to read/write delay).\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RAS_A {
    #[doc = "0: Reserved."]
    RESERVED_ = 0,
    #[doc = "1: One CCLK cycle."]
    ONE_CCLK_CYCLE_ = 1,
    #[doc = "2: Two CCLK cycles."]
    TWO_CCLK_CYCLES_ = 2,
    #[doc = "3: Three CCLK cycles (POR reset value)."]
    THREE_CCLK_CYCLES_P = 3,
}
impl From<RAS_A> for u8 {
    #[inline(always)]
    fn from(variant: RAS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RAS` reader - RAS latency (active to read/write delay)."]
pub struct RAS_R(crate::FieldReader<u8, RAS_A>);
impl RAS_R {
    pub(crate) fn new(bits: u8) -> Self {
        RAS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAS_A {
        match self.bits {
            0 => RAS_A::RESERVED_,
            1 => RAS_A::ONE_CCLK_CYCLE_,
            2 => RAS_A::TWO_CCLK_CYCLES_,
            3 => RAS_A::THREE_CCLK_CYCLES_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RESERVED_`"]
    #[inline(always)]
    pub fn is_reserved_(&self) -> bool {
        **self == RAS_A::RESERVED_
    }
    #[doc = "Checks if the value of the field is `ONE_CCLK_CYCLE_`"]
    #[inline(always)]
    pub fn is_one_cclk_cycle_(&self) -> bool {
        **self == RAS_A::ONE_CCLK_CYCLE_
    }
    #[doc = "Checks if the value of the field is `TWO_CCLK_CYCLES_`"]
    #[inline(always)]
    pub fn is_two_cclk_cycles_(&self) -> bool {
        **self == RAS_A::TWO_CCLK_CYCLES_
    }
    #[doc = "Checks if the value of the field is `THREE_CCLK_CYCLES_P`"]
    #[inline(always)]
    pub fn is_three_cclk_cycles_p(&self) -> bool {
        **self == RAS_A::THREE_CCLK_CYCLES_P
    }
}
impl core::ops::Deref for RAS_R {
    type Target = crate::FieldReader<u8, RAS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAS` writer - RAS latency (active to read/write delay)."]
pub struct RAS_W<'a> {
    w: &'a mut W,
}
impl<'a> RAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn reserved_(self) -> &'a mut W {
        self.variant(RAS_A::RESERVED_)
    }
    #[doc = "One CCLK cycle."]
    #[inline(always)]
    pub fn one_cclk_cycle_(self) -> &'a mut W {
        self.variant(RAS_A::ONE_CCLK_CYCLE_)
    }
    #[doc = "Two CCLK cycles."]
    #[inline(always)]
    pub fn two_cclk_cycles_(self) -> &'a mut W {
        self.variant(RAS_A::TWO_CCLK_CYCLES_)
    }
    #[doc = "Three CCLK cycles (POR reset value)."]
    #[inline(always)]
    pub fn three_cclk_cycles_p(self) -> &'a mut W {
        self.variant(RAS_A::THREE_CCLK_CYCLES_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "CAS latency.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CAS_A {
    #[doc = "0: Reserved."]
    RESERVED_ = 0,
    #[doc = "1: One CCLK cycle."]
    ONE_CCLK_CYCLE_ = 1,
    #[doc = "2: Two CCLK cycles."]
    TWO_CCLK_CYCLES_ = 2,
    #[doc = "3: Three CCLK cycles (POR reset value)."]
    THREE_CCLK_CYCLES_P = 3,
}
impl From<CAS_A> for u8 {
    #[inline(always)]
    fn from(variant: CAS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CAS` reader - CAS latency."]
pub struct CAS_R(crate::FieldReader<u8, CAS_A>);
impl CAS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CAS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAS_A {
        match self.bits {
            0 => CAS_A::RESERVED_,
            1 => CAS_A::ONE_CCLK_CYCLE_,
            2 => CAS_A::TWO_CCLK_CYCLES_,
            3 => CAS_A::THREE_CCLK_CYCLES_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RESERVED_`"]
    #[inline(always)]
    pub fn is_reserved_(&self) -> bool {
        **self == CAS_A::RESERVED_
    }
    #[doc = "Checks if the value of the field is `ONE_CCLK_CYCLE_`"]
    #[inline(always)]
    pub fn is_one_cclk_cycle_(&self) -> bool {
        **self == CAS_A::ONE_CCLK_CYCLE_
    }
    #[doc = "Checks if the value of the field is `TWO_CCLK_CYCLES_`"]
    #[inline(always)]
    pub fn is_two_cclk_cycles_(&self) -> bool {
        **self == CAS_A::TWO_CCLK_CYCLES_
    }
    #[doc = "Checks if the value of the field is `THREE_CCLK_CYCLES_P`"]
    #[inline(always)]
    pub fn is_three_cclk_cycles_p(&self) -> bool {
        **self == CAS_A::THREE_CCLK_CYCLES_P
    }
}
impl core::ops::Deref for CAS_R {
    type Target = crate::FieldReader<u8, CAS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAS` writer - CAS latency."]
pub struct CAS_W<'a> {
    w: &'a mut W,
}
impl<'a> CAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn reserved_(self) -> &'a mut W {
        self.variant(CAS_A::RESERVED_)
    }
    #[doc = "One CCLK cycle."]
    #[inline(always)]
    pub fn one_cclk_cycle_(self) -> &'a mut W {
        self.variant(CAS_A::ONE_CCLK_CYCLE_)
    }
    #[doc = "Two CCLK cycles."]
    #[inline(always)]
    pub fn two_cclk_cycles_(self) -> &'a mut W {
        self.variant(CAS_A::TWO_CCLK_CYCLES_)
    }
    #[doc = "Three CCLK cycles (POR reset value)."]
    #[inline(always)]
    pub fn three_cclk_cycles_p(self) -> &'a mut W {
        self.variant(CAS_A::THREE_CCLK_CYCLES_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - RAS latency (active to read/write delay)."]
    #[inline(always)]
    pub fn ras(&self) -> RAS_R {
        RAS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - CAS latency."]
    #[inline(always)]
    pub fn cas(&self) -> CAS_R {
        CAS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RAS latency (active to read/write delay)."]
    #[inline(always)]
    pub fn ras(&mut self) -> RAS_W {
        RAS_W { w: self }
    }
    #[doc = "Bits 8:9 - CAS latency."]
    #[inline(always)]
    pub fn cas(&mut self) -> CAS_W {
        CAS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAS and CAS latencies for EMC_DYCS0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicrascas](index.html) module"]
pub struct DYNAMICRASCAS_SPEC;
impl crate::RegisterSpec for DYNAMICRASCAS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dynamicrascas::R](R) reader structure"]
impl crate::Readable for DYNAMICRASCAS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dynamicrascas::W](W) writer structure"]
impl crate::Writable for DYNAMICRASCAS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DYNAMICRASCAS%s to value 0x0303"]
impl crate::Resettable for DYNAMICRASCAS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0303
    }
}
