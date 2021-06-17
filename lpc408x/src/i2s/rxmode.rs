#[doc = "Register `RXMODE` reader"]
pub struct R(crate::R<RXMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXMODE` writer"]
pub struct W(crate::W<RXMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXMODE_SPEC>;
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
impl From<crate::W<RXMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock source selection for the receive bit clock divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXCLKSEL_A {
    #[doc = "0: Select the RX fractional rate divider clock output as the source"]
    SELECT_THE_RX_FRACTI = 0,
    #[doc = "2: Select the TX_MCLK signal as the RX_MCLK clock source"]
    SELECT_THE_TX_MCLK_S = 2,
}
impl From<RXCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RXCLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RXCLKSEL` reader - Clock source selection for the receive bit clock divider."]
pub struct RXCLKSEL_R(crate::FieldReader<u8, RXCLKSEL_A>);
impl RXCLKSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXCLKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXCLKSEL_A {
        match self.bits {
            0 => RXCLKSEL_A::SELECT_THE_RX_FRACTI,
            2 => RXCLKSEL_A::SELECT_THE_TX_MCLK_S,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SELECT_THE_RX_FRACTI`"]
    #[inline(always)]
    pub fn is_select_the_rx_fracti(&self) -> bool {
        **self == RXCLKSEL_A::SELECT_THE_RX_FRACTI
    }
    #[doc = "Checks if the value of the field is `SELECT_THE_TX_MCLK_S`"]
    #[inline(always)]
    pub fn is_select_the_tx_mclk_s(&self) -> bool {
        **self == RXCLKSEL_A::SELECT_THE_TX_MCLK_S
    }
}
impl core::ops::Deref for RXCLKSEL_R {
    type Target = crate::FieldReader<u8, RXCLKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXCLKSEL` writer - Clock source selection for the receive bit clock divider."]
pub struct RXCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXCLKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select the RX fractional rate divider clock output as the source"]
    #[inline(always)]
    pub fn select_the_rx_fracti(self) -> &'a mut W {
        self.variant(RXCLKSEL_A::SELECT_THE_RX_FRACTI)
    }
    #[doc = "Select the TX_MCLK signal as the RX_MCLK clock source"]
    #[inline(always)]
    pub fn select_the_tx_mclk_s(self) -> &'a mut W {
        self.variant(RXCLKSEL_A::SELECT_THE_TX_MCLK_S)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `RX4PIN` reader - Receive 4-pin mode selection. When 1, enables 4-pin mode."]
pub struct RX4PIN_R(crate::FieldReader<bool, bool>);
impl RX4PIN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX4PIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX4PIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX4PIN` writer - Receive 4-pin mode selection. When 1, enables 4-pin mode."]
pub struct RX4PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX4PIN_W<'a> {
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
#[doc = "Field `RXMCENA` reader - Enable for the RX_MCLK output. When 0, output of RX_MCLK is not enabled. When 1, output of RX_MCLK is enabled."]
pub struct RXMCENA_R(crate::FieldReader<bool, bool>);
impl RXMCENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXMCENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXMCENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXMCENA` writer - Enable for the RX_MCLK output. When 0, output of RX_MCLK is not enabled. When 1, output of RX_MCLK is enabled."]
pub struct RXMCENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RXMCENA_W<'a> {
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
    #[doc = "Bits 0:1 - Clock source selection for the receive bit clock divider."]
    #[inline(always)]
    pub fn rxclksel(&self) -> RXCLKSEL_R {
        RXCLKSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Receive 4-pin mode selection. When 1, enables 4-pin mode."]
    #[inline(always)]
    pub fn rx4pin(&self) -> RX4PIN_R {
        RX4PIN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable for the RX_MCLK output. When 0, output of RX_MCLK is not enabled. When 1, output of RX_MCLK is enabled."]
    #[inline(always)]
    pub fn rxmcena(&self) -> RXMCENA_R {
        RXMCENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock source selection for the receive bit clock divider."]
    #[inline(always)]
    pub fn rxclksel(&mut self) -> RXCLKSEL_W {
        RXCLKSEL_W { w: self }
    }
    #[doc = "Bit 2 - Receive 4-pin mode selection. When 1, enables 4-pin mode."]
    #[inline(always)]
    pub fn rx4pin(&mut self) -> RX4PIN_W {
        RX4PIN_W { w: self }
    }
    #[doc = "Bit 3 - Enable for the RX_MCLK output. When 0, output of RX_MCLK is not enabled. When 1, output of RX_MCLK is enabled."]
    #[inline(always)]
    pub fn rxmcena(&mut self) -> RXMCENA_W {
        RXMCENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S Receive mode control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxmode](index.html) module"]
pub struct RXMODE_SPEC;
impl crate::RegisterSpec for RXMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxmode::R](R) reader structure"]
impl crate::Readable for RXMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxmode::W](W) writer structure"]
impl crate::Writable for RXMODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXMODE to value 0"]
impl crate::Resettable for RXMODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
