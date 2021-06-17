#[doc = "Register `CONN_CH_TX_POWER_LVL_MS` reader"]
pub struct R(crate::R<CONN_CH_TX_POWER_LVL_MS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_CH_TX_POWER_LVL_MS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_CH_TX_POWER_LVL_MS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_CH_TX_POWER_LVL_MS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_CH_TX_POWER_LVL_MS` writer"]
pub struct W(crate::W<CONN_CH_TX_POWER_LVL_MS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_CH_TX_POWER_LVL_MS_SPEC>;
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
impl From<crate::W<CONN_CH_TX_POWER_LVL_MS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_CH_TX_POWER_LVL_MS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONNCH_TRANSMIT_POWER_LVL_MS` reader - Connection channel transmit power setting Most Significant 2 bits."]
pub struct CONNCH_TRANSMIT_POWER_LVL_MS_R(crate::FieldReader<u8, u8>);
impl CONNCH_TRANSMIT_POWER_LVL_MS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CONNCH_TRANSMIT_POWER_LVL_MS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONNCH_TRANSMIT_POWER_LVL_MS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONNCH_TRANSMIT_POWER_LVL_MS` writer - Connection channel transmit power setting Most Significant 2 bits."]
pub struct CONNCH_TRANSMIT_POWER_LVL_MS_W<'a> {
    w: &'a mut W,
}
impl<'a> CONNCH_TRANSMIT_POWER_LVL_MS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Connection channel transmit power setting Most Significant 2 bits."]
    #[inline(always)]
    pub fn connch_transmit_power_lvl_ms(&self) -> CONNCH_TRANSMIT_POWER_LVL_MS_R {
        CONNCH_TRANSMIT_POWER_LVL_MS_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Connection channel transmit power setting Most Significant 2 bits."]
    #[inline(always)]
    pub fn connch_transmit_power_lvl_ms(&mut self) -> CONNCH_TRANSMIT_POWER_LVL_MS_W {
        CONNCH_TRANSMIT_POWER_LVL_MS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Connection channel transmit power setting extension\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_ch_tx_power_lvl_ms](index.html) module"]
pub struct CONN_CH_TX_POWER_LVL_MS_SPEC;
impl crate::RegisterSpec for CONN_CH_TX_POWER_LVL_MS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_ch_tx_power_lvl_ms::R](R) reader structure"]
impl crate::Readable for CONN_CH_TX_POWER_LVL_MS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_ch_tx_power_lvl_ms::W](W) writer structure"]
impl crate::Writable for CONN_CH_TX_POWER_LVL_MS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_CH_TX_POWER_LVL_MS to value 0"]
impl crate::Resettable for CONN_CH_TX_POWER_LVL_MS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
