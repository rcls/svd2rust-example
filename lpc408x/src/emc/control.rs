#[doc = "Register `CONTROL` reader"]
pub struct R(crate::R<CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONTROL` writer"]
pub struct W(crate::W<CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTROL_SPEC>;
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
impl From<crate::W<CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "EMC Enable. Indicates if the EMC is enabled or disabled:\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled\r\n(POR and warm reset value)."]
    ENABLED = 1,
}
impl From<E_A> for bool {
    #[inline(always)]
    fn from(variant: E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E` reader - EMC Enable. Indicates if the EMC is enabled or disabled:"]
pub struct E_R(crate::FieldReader<bool, E_A>);
impl E_R {
    pub(crate) fn new(bits: bool) -> Self {
        E_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E_A {
        match self.bits {
            false => E_A::DISABLED,
            true => E_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == E_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == E_A::ENABLED
    }
}
impl core::ops::Deref for E_R {
    type Target = crate::FieldReader<bool, E_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `E` writer - EMC Enable. Indicates if the EMC is enabled or disabled:"]
pub struct E_W<'a> {
    w: &'a mut W,
}
impl<'a> E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(E_A::DISABLED)
    }
    #[doc = "Enabled (POR and warm reset value)."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(E_A::ENABLED)
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
#[doc = "Address mirror. Indicates normal or reset memory map:\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M_A {
    #[doc = "0: Normal memory map."]
    NORMAL = 0,
    #[doc = "1: Reset memory map. Static memory EMC_CS1 is\r\nmirrored onto EMC_CS0 and EMC_DYCS0 (POR reset value)."]
    RESET = 1,
}
impl From<M_A> for bool {
    #[inline(always)]
    fn from(variant: M_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `M` reader - Address mirror. Indicates normal or reset memory map:"]
pub struct M_R(crate::FieldReader<bool, M_A>);
impl M_R {
    pub(crate) fn new(bits: bool) -> Self {
        M_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M_A {
        match self.bits {
            false => M_A::NORMAL,
            true => M_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == M_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == M_A::RESET
    }
}
impl core::ops::Deref for M_R {
    type Target = crate::FieldReader<bool, M_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M` writer - Address mirror. Indicates normal or reset memory map:"]
pub struct M_W<'a> {
    w: &'a mut W,
}
impl<'a> M_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal memory map."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(M_A::NORMAL)
    }
    #[doc = "Reset memory map. Static memory EMC_CS1 is mirrored onto EMC_CS0 and EMC_DYCS0 (POR reset value)."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(M_A::RESET)
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
#[doc = "Low-power mode. Indicates normal, or low-power mode:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L_A {
    #[doc = "0: Normal mode (warm\r\nreset value)."]
    WARMRESET = 0,
    #[doc = "1: Low-power\r\nmode. Entering low-power mode reduces memory controller power consumption.\r\nDynamic memory is refreshed as necessary. The memory controller\r\nreturns to normal functional mode by clearing the low-power mode\r\nbit (L), or by POR. This bit must only be modified when the EMC\r\nis in idle state.\\[1\\]"]
    LOWPOWER = 1,
}
impl From<L_A> for bool {
    #[inline(always)]
    fn from(variant: L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L` reader - Low-power mode. Indicates normal, or low-power mode:"]
pub struct L_R(crate::FieldReader<bool, L_A>);
impl L_R {
    pub(crate) fn new(bits: bool) -> Self {
        L_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L_A {
        match self.bits {
            false => L_A::WARMRESET,
            true => L_A::LOWPOWER,
        }
    }
    #[doc = "Checks if the value of the field is `WARMRESET`"]
    #[inline(always)]
    pub fn is_warmreset(&self) -> bool {
        **self == L_A::WARMRESET
    }
    #[doc = "Checks if the value of the field is `LOWPOWER`"]
    #[inline(always)]
    pub fn is_lowpower(&self) -> bool {
        **self == L_A::LOWPOWER
    }
}
impl core::ops::Deref for L_R {
    type Target = crate::FieldReader<bool, L_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L` writer - Low-power mode. Indicates normal, or low-power mode:"]
pub struct L_W<'a> {
    w: &'a mut W,
}
impl<'a> L_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: L_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal mode (warm reset value)."]
    #[inline(always)]
    pub fn warmreset(self) -> &'a mut W {
        self.variant(L_A::WARMRESET)
    }
    #[doc = "Low-power mode. Entering low-power mode reduces memory controller power consumption. Dynamic memory is refreshed as necessary. The memory controller returns to normal functional mode by clearing the low-power mode bit (L), or by POR. This bit must only be modified when the EMC is in idle state.\\[1\\]"]
    #[inline(always)]
    pub fn lowpower(self) -> &'a mut W {
        self.variant(L_A::LOWPOWER)
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
    #[doc = "Bit 0 - EMC Enable. Indicates if the EMC is enabled or disabled:"]
    #[inline(always)]
    pub fn e(&self) -> E_R {
        E_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Address mirror. Indicates normal or reset memory map:"]
    #[inline(always)]
    pub fn m(&self) -> M_R {
        M_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Low-power mode. Indicates normal, or low-power mode:"]
    #[inline(always)]
    pub fn l(&self) -> L_R {
        L_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EMC Enable. Indicates if the EMC is enabled or disabled:"]
    #[inline(always)]
    pub fn e(&mut self) -> E_W {
        E_W { w: self }
    }
    #[doc = "Bit 1 - Address mirror. Indicates normal or reset memory map:"]
    #[inline(always)]
    pub fn m(&mut self) -> M_W {
        M_W { w: self }
    }
    #[doc = "Bit 2 - Low-power mode. Indicates normal, or low-power mode:"]
    #[inline(always)]
    pub fn l(&mut self) -> L_W {
        L_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls operation of the memory controller.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](index.html) module"]
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [control::R](R) reader structure"]
impl crate::Readable for CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [control::W](W) writer structure"]
impl crate::Writable for CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONTROL to value 0x03"]
impl crate::Resettable for CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
