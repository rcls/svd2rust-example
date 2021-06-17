#[doc = "Register `TXMODE` reader"]
pub struct R(crate::R<TXMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXMODE` writer"]
pub struct W(crate::W<TXMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXMODE_SPEC>;
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
impl From<crate::W<TXMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock source selection for the transmit bit clock divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXCLKSEL_A {
    #[doc = "0: Select the TX fractional rate divider clock output as the source"]
    SELECT_THE_TX_FRACTI = 0,
    #[doc = "2: Select the RX_MCLK signal as the TX_MCLK clock source"]
    SELECT_THE_RX_MCLK_S = 2,
}
impl From<TXCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TXCLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TXCLKSEL` reader - Clock source selection for the transmit bit clock divider."]
pub struct TXCLKSEL_R(crate::FieldReader<u8, TXCLKSEL_A>);
impl TXCLKSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXCLKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXCLKSEL_A {
        match self.bits {
            0 => TXCLKSEL_A::SELECT_THE_TX_FRACTI,
            2 => TXCLKSEL_A::SELECT_THE_RX_MCLK_S,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SELECT_THE_TX_FRACTI`"]
    #[inline(always)]
    pub fn is_select_the_tx_fracti(&self) -> bool {
        **self == TXCLKSEL_A::SELECT_THE_TX_FRACTI
    }
    #[doc = "Checks if the value of the field is `SELECT_THE_RX_MCLK_S`"]
    #[inline(always)]
    pub fn is_select_the_rx_mclk_s(&self) -> bool {
        **self == TXCLKSEL_A::SELECT_THE_RX_MCLK_S
    }
}
impl core::ops::Deref for TXCLKSEL_R {
    type Target = crate::FieldReader<u8, TXCLKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXCLKSEL` writer - Clock source selection for the transmit bit clock divider."]
pub struct TXCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXCLKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select the TX fractional rate divider clock output as the source"]
    #[inline(always)]
    pub fn select_the_tx_fracti(self) -> &'a mut W {
        self.variant(TXCLKSEL_A::SELECT_THE_TX_FRACTI)
    }
    #[doc = "Select the RX_MCLK signal as the TX_MCLK clock source"]
    #[inline(always)]
    pub fn select_the_rx_mclk_s(self) -> &'a mut W {
        self.variant(TXCLKSEL_A::SELECT_THE_RX_MCLK_S)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `TX4PIN` reader - Transmit 4-pin mode selection. When 1, enables 4-pin mode."]
pub struct TX4PIN_R(crate::FieldReader<bool, bool>);
impl TX4PIN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX4PIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX4PIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX4PIN` writer - Transmit 4-pin mode selection. When 1, enables 4-pin mode."]
pub struct TX4PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX4PIN_W<'a> {
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
#[doc = "Field `TXMCENA` reader - Enable for the TX_MCLK output. When 0, output of TX_MCLK is not enabled. When 1, output of TX_MCLK is enabled."]
pub struct TXMCENA_R(crate::FieldReader<bool, bool>);
impl TXMCENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXMCENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMCENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMCENA` writer - Enable for the TX_MCLK output. When 0, output of TX_MCLK is not enabled. When 1, output of TX_MCLK is enabled."]
pub struct TXMCENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMCENA_W<'a> {
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
    #[doc = "Bits 0:1 - Clock source selection for the transmit bit clock divider."]
    #[inline(always)]
    pub fn txclksel(&self) -> TXCLKSEL_R {
        TXCLKSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Transmit 4-pin mode selection. When 1, enables 4-pin mode."]
    #[inline(always)]
    pub fn tx4pin(&self) -> TX4PIN_R {
        TX4PIN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable for the TX_MCLK output. When 0, output of TX_MCLK is not enabled. When 1, output of TX_MCLK is enabled."]
    #[inline(always)]
    pub fn txmcena(&self) -> TXMCENA_R {
        TXMCENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock source selection for the transmit bit clock divider."]
    #[inline(always)]
    pub fn txclksel(&mut self) -> TXCLKSEL_W {
        TXCLKSEL_W { w: self }
    }
    #[doc = "Bit 2 - Transmit 4-pin mode selection. When 1, enables 4-pin mode."]
    #[inline(always)]
    pub fn tx4pin(&mut self) -> TX4PIN_W {
        TX4PIN_W { w: self }
    }
    #[doc = "Bit 3 - Enable for the TX_MCLK output. When 0, output of TX_MCLK is not enabled. When 1, output of TX_MCLK is enabled."]
    #[inline(always)]
    pub fn txmcena(&mut self) -> TXMCENA_W {
        TXMCENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S Transmit mode control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txmode](index.html) module"]
pub struct TXMODE_SPEC;
impl crate::RegisterSpec for TXMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txmode::R](R) reader structure"]
impl crate::Readable for TXMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txmode::W](W) writer structure"]
impl crate::Writable for TXMODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXMODE to value 0"]
impl crate::Resettable for TXMODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
