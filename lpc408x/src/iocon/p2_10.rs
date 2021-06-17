#[doc = "Register `P2_10` reader"]
pub struct R(crate::R<P2_10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P2_10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P2_10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P2_10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P2_10` writer"]
pub struct W(crate::W<P2_10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P2_10_SPEC>;
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
impl From<crate::W<P2_10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P2_10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selects pin function for pin P2\\[10\\]"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNC_A {
    #[doc = "0: General purpose digital input/output pin. This pin\r\n                                    includes a 5 ns input glitch filter.A LOW on this pin while RESET is LOW forces the on-chip\r\n                                    boot loader to take over control of the part after a reset and\r\n                                    go into ISP mode. "]
    P2_10 = 0,
    #[doc = "1: External interrupt 0 input."]
    EINT0 = 1,
    #[doc = "2: Non-maskable interrupt input."]
    NMI = 2,
}
impl From<FUNC_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FUNC` reader - Selects pin function for pin P2\\[10\\]"]
pub struct FUNC_R(crate::FieldReader<u8, FUNC_A>);
impl FUNC_R {
    pub(crate) fn new(bits: u8) -> Self {
        FUNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FUNC_A> {
        match self.bits {
            0 => Some(FUNC_A::P2_10),
            1 => Some(FUNC_A::EINT0),
            2 => Some(FUNC_A::NMI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `P2_10`"]
    #[inline(always)]
    pub fn is_p2_10(&self) -> bool {
        **self == FUNC_A::P2_10
    }
    #[doc = "Checks if the value of the field is `EINT0`"]
    #[inline(always)]
    pub fn is_eint0(&self) -> bool {
        **self == FUNC_A::EINT0
    }
    #[doc = "Checks if the value of the field is `NMI`"]
    #[inline(always)]
    pub fn is_nmi(&self) -> bool {
        **self == FUNC_A::NMI
    }
}
impl core::ops::Deref for FUNC_R {
    type Target = crate::FieldReader<u8, FUNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUNC` writer - Selects pin function for pin P2\\[10\\]"]
pub struct FUNC_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FUNC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "General purpose digital input/output pin. This pin includes a 5 ns input glitch filter.A LOW on this pin while RESET is LOW forces the on-chip boot loader to take over control of the part after a reset and go into ISP mode."]
    #[inline(always)]
    pub fn p2_10(self) -> &'a mut W {
        self.variant(FUNC_A::P2_10)
    }
    #[doc = "External interrupt 0 input."]
    #[inline(always)]
    pub fn eint0(self) -> &'a mut W {
        self.variant(FUNC_A::EINT0)
    }
    #[doc = "Non-maskable interrupt input."]
    #[inline(always)]
    pub fn nmi(self) -> &'a mut W {
        self.variant(FUNC_A::NMI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P2\\[10\\]"]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function for pin P2\\[10\\]"]
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W {
        FUNC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O configuration register for pin P2\\[10\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2_10](index.html) module"]
pub struct P2_10_SPEC;
impl crate::RegisterSpec for P2_10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [p2_10::R](R) reader structure"]
impl crate::Readable for P2_10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p2_10::W](W) writer structure"]
impl crate::Writable for P2_10_SPEC {
    type Writer = W;
}
