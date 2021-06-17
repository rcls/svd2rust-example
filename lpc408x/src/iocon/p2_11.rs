#[doc = "Register `P2_11` reader"]
pub struct R(crate::R<P2_11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P2_11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P2_11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P2_11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P2_11` writer"]
pub struct W(crate::W<P2_11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P2_11_SPEC>;
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
impl From<crate::W<P2_11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P2_11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selects pin function for pin P2\\[11\\]"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNC_A {
    #[doc = "0: General purpose digital input/output pin. This pin\r\n                                    includes a 5 ns input glitch filter."]
    P2_11 = 0,
    #[doc = "1: External interrupt 1 input."]
    EINT1 = 1,
    #[doc = "2: Data line 1 for SD card interface."]
    SD_DAT_1 = 2,
    #[doc = "3: Transmit Clock. It is driven by the master and received                                     by the slave. Corresponds to the signal SCK in the                                             I2S-bus                                         specification."]
    I2S_TX_SCK = 3,
    #[doc = "7: LCD clock."]
    LCD_CLKIN = 7,
}
impl From<FUNC_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FUNC` reader - Selects pin function for pin P2\\[11\\]"]
pub struct FUNC_R(crate::FieldReader<u8, FUNC_A>);
impl FUNC_R {
    pub(crate) fn new(bits: u8) -> Self {
        FUNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FUNC_A> {
        match self.bits {
            0 => Some(FUNC_A::P2_11),
            1 => Some(FUNC_A::EINT1),
            2 => Some(FUNC_A::SD_DAT_1),
            3 => Some(FUNC_A::I2S_TX_SCK),
            7 => Some(FUNC_A::LCD_CLKIN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `P2_11`"]
    #[inline(always)]
    pub fn is_p2_11(&self) -> bool {
        **self == FUNC_A::P2_11
    }
    #[doc = "Checks if the value of the field is `EINT1`"]
    #[inline(always)]
    pub fn is_eint1(&self) -> bool {
        **self == FUNC_A::EINT1
    }
    #[doc = "Checks if the value of the field is `SD_DAT_1`"]
    #[inline(always)]
    pub fn is_sd_dat_1(&self) -> bool {
        **self == FUNC_A::SD_DAT_1
    }
    #[doc = "Checks if the value of the field is `I2S_TX_SCK`"]
    #[inline(always)]
    pub fn is_i2s_tx_sck(&self) -> bool {
        **self == FUNC_A::I2S_TX_SCK
    }
    #[doc = "Checks if the value of the field is `LCD_CLKIN`"]
    #[inline(always)]
    pub fn is_lcd_clkin(&self) -> bool {
        **self == FUNC_A::LCD_CLKIN
    }
}
impl core::ops::Deref for FUNC_R {
    type Target = crate::FieldReader<u8, FUNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUNC` writer - Selects pin function for pin P2\\[11\\]"]
pub struct FUNC_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FUNC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "General purpose digital input/output pin. This pin includes a 5 ns input glitch filter."]
    #[inline(always)]
    pub fn p2_11(self) -> &'a mut W {
        self.variant(FUNC_A::P2_11)
    }
    #[doc = "External interrupt 1 input."]
    #[inline(always)]
    pub fn eint1(self) -> &'a mut W {
        self.variant(FUNC_A::EINT1)
    }
    #[doc = "Data line 1 for SD card interface."]
    #[inline(always)]
    pub fn sd_dat_1(self) -> &'a mut W {
        self.variant(FUNC_A::SD_DAT_1)
    }
    #[doc = "Transmit Clock. It is driven by the master and received by the slave. Corresponds to the signal SCK in the I2S-bus specification."]
    #[inline(always)]
    pub fn i2s_tx_sck(self) -> &'a mut W {
        self.variant(FUNC_A::I2S_TX_SCK)
    }
    #[doc = "LCD clock."]
    #[inline(always)]
    pub fn lcd_clkin(self) -> &'a mut W {
        self.variant(FUNC_A::LCD_CLKIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P2\\[11\\]"]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function for pin P2\\[11\\]"]
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
#[doc = "I/O configuration register for pin P2\\[11\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2_11](index.html) module"]
pub struct P2_11_SPEC;
impl crate::RegisterSpec for P2_11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [p2_11::R](R) reader structure"]
impl crate::Readable for P2_11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p2_11::W](W) writer structure"]
impl crate::Writable for P2_11_SPEC {
    type Writer = W;
}
