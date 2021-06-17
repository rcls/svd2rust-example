#[doc = "Register `TX_CTRL` reader"]
pub struct R(crate::R<TX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_CTRL` writer"]
pub struct W(crate::W<TX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_CTRL_SPEC>;
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
impl From<crate::W<TX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSB_FIRST` reader - Least significant bit first ('0') or most significant bit first ('1'). This field also affects the Address field When MSB_FIRST = 1, then \\[15:0\\]
is data and \\[(ADDR_WIDTH+15):16\\]
is used for address When MSB_FIRST = 0, then \\[15:0\\]
is for data. No address field"]
pub struct MSB_FIRST_R(crate::FieldReader<bool, bool>);
impl MSB_FIRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSB_FIRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSB_FIRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSB_FIRST` writer - Least significant bit first ('0') or most significant bit first ('1'). This field also affects the Address field When MSB_FIRST = 1, then \\[15:0\\]
is data and \\[(ADDR_WIDTH+15):16\\]
is used for address When MSB_FIRST = 0, then \\[15:0\\]
is for data. No address field"]
pub struct MSB_FIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> MSB_FIRST_W<'a> {
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
#[doc = "Field `FIFO_RECONFIG` reader - Setting this bit, clears the FIFO and resets the pointer"]
pub struct FIFO_RECONFIG_R(crate::FieldReader<bool, bool>);
impl FIFO_RECONFIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_RECONFIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_RECONFIG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_RECONFIG` writer - Setting this bit, clears the FIFO and resets the pointer"]
pub struct FIFO_RECONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_RECONFIG_W<'a> {
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
#[doc = "Field `TX_ENTRIES` reader - This field determines the depth of the TX_FIFO. Allowed legal values are 8 and 16 only"]
pub struct TX_ENTRIES_R(crate::FieldReader<u8, u8>);
impl TX_ENTRIES_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_ENTRIES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_ENTRIES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_ENTRIES` writer - This field determines the depth of the TX_FIFO. Allowed legal values are 8 and 16 only"]
pub struct TX_ENTRIES_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ENTRIES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 2)) | ((value as u32 & 0x1f) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Least significant bit first ('0') or most significant bit first ('1'). This field also affects the Address field When MSB_FIRST = 1, then \\[15:0\\]
is data and \\[(ADDR_WIDTH+15):16\\]
is used for address When MSB_FIRST = 0, then \\[15:0\\]
is for data. No address field"]
    #[inline(always)]
    pub fn msb_first(&self) -> MSB_FIRST_R {
        MSB_FIRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Setting this bit, clears the FIFO and resets the pointer"]
    #[inline(always)]
    pub fn fifo_reconfig(&self) -> FIFO_RECONFIG_R {
        FIFO_RECONFIG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:6 - This field determines the depth of the TX_FIFO. Allowed legal values are 8 and 16 only"]
    #[inline(always)]
    pub fn tx_entries(&self) -> TX_ENTRIES_R {
        TX_ENTRIES_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Least significant bit first ('0') or most significant bit first ('1'). This field also affects the Address field When MSB_FIRST = 1, then \\[15:0\\]
is data and \\[(ADDR_WIDTH+15):16\\]
is used for address When MSB_FIRST = 0, then \\[15:0\\]
is for data. No address field"]
    #[inline(always)]
    pub fn msb_first(&mut self) -> MSB_FIRST_W {
        MSB_FIRST_W { w: self }
    }
    #[doc = "Bit 1 - Setting this bit, clears the FIFO and resets the pointer"]
    #[inline(always)]
    pub fn fifo_reconfig(&mut self) -> FIFO_RECONFIG_W {
        FIFO_RECONFIG_W { w: self }
    }
    #[doc = "Bits 2:6 - This field determines the depth of the TX_FIFO. Allowed legal values are 8 and 16 only"]
    #[inline(always)]
    pub fn tx_entries(&mut self) -> TX_ENTRIES_W {
        TX_ENTRIES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmitter control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_ctrl](index.html) module"]
pub struct TX_CTRL_SPEC;
impl crate::RegisterSpec for TX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_ctrl::R](R) reader structure"]
impl crate::Readable for TX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_ctrl::W](W) writer structure"]
impl crate::Writable for TX_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_CTRL to value 0x21"]
impl crate::Resettable for TX_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x21
    }
}
