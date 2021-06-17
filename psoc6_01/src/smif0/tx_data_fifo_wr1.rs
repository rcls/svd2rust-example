#[doc = "Register `TX_DATA_FIFO_WR1` writer"]
pub struct W(crate::W<TX_DATA_FIFO_WR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_DATA_FIFO_WR1_SPEC>;
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
impl From<crate::W<TX_DATA_FIFO_WR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_DATA_FIFO_WR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA0` writer - TX data (written to TX data FIFO)."]
pub struct DATA0_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - TX data (written to TX data FIFO)."]
    #[inline(always)]
    pub fn data0(&mut self) -> DATA0_W {
        DATA0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmitter data FIFO write\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_data_fifo_wr1](index.html) module"]
pub struct TX_DATA_FIFO_WR1_SPEC;
impl crate::RegisterSpec for TX_DATA_FIFO_WR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tx_data_fifo_wr1::W](W) writer structure"]
impl crate::Writable for TX_DATA_FIFO_WR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_DATA_FIFO_WR1 to value 0"]
impl crate::Resettable for TX_DATA_FIFO_WR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
