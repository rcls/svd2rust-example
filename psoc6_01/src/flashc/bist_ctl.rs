#[doc = "Register `BIST_CTL` reader"]
pub struct R(crate::R<BIST_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIST_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIST_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIST_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BIST_CTL` writer"]
pub struct W(crate::W<BIST_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BIST_CTL_SPEC>;
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
impl From<crate::W<BIST_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BIST_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPCODE` reader - This field specifies how the data check should be performed after reading the data from Flash memory. 0: Read the Flash and compare the output to BIST_DATA (R0). 1: Read the Flash and compare the output to the binary complement of BIST_DATA (R1). 3: Read the Flash and compare with BIST_DATA\\[\\]
and compliment of BIST_DATA alternately (R01). The expected data of the first read is BIST_DATA, expected data of the second read is binary compliment of BIST_DATA, third read expected data is BIST_DATA, fourth read expected data is binary compliment of BIST_DATA and so on."]
pub struct OPCODE_R(crate::FieldReader<u8, u8>);
impl OPCODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        OPCODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPCODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPCODE` writer - This field specifies how the data check should be performed after reading the data from Flash memory. 0: Read the Flash and compare the output to BIST_DATA (R0). 1: Read the Flash and compare the output to the binary complement of BIST_DATA (R1). 3: Read the Flash and compare with BIST_DATA\\[\\]
and compliment of BIST_DATA alternately (R01). The expected data of the first read is BIST_DATA, expected data of the second read is binary compliment of BIST_DATA, third read expected data is BIST_DATA, fourth read expected data is binary compliment of BIST_DATA and so on."]
pub struct OPCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OPCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `UP` reader - Specifies direction in which Flash BIST steps through addresses: 0: BIST steps through the Flash from the maximum row and column addresses (as specified by a design time configuration parameter when ADDR_START_ENABLED is '0' and as specified by BIST_ADDR_START when ADDR_START_ENABLED is '1') to the minimum row and column addresses. 1: BIST steps through the Flash from the minimum row and column addresses ('0' when ADDR_START_ENABLED is '0' and as specified by BIST_ADDR_START when ADDR_START_ENABLED is '1'' to the maximum row and column addresses."]
pub struct UP_R(crate::FieldReader<bool, bool>);
impl UP_R {
    pub(crate) fn new(bits: bool) -> Self {
        UP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UP` writer - Specifies direction in which Flash BIST steps through addresses: 0: BIST steps through the Flash from the maximum row and column addresses (as specified by a design time configuration parameter when ADDR_START_ENABLED is '0' and as specified by BIST_ADDR_START when ADDR_START_ENABLED is '1') to the minimum row and column addresses. 1: BIST steps through the Flash from the minimum row and column addresses ('0' when ADDR_START_ENABLED is '0' and as specified by BIST_ADDR_START when ADDR_START_ENABLED is '1'' to the maximum row and column addresses."]
pub struct UP_W<'a> {
    w: &'a mut W,
}
impl<'a> UP_W<'a> {
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
#[doc = "Field `ROW_FIRST` reader - Specifies how the Flash BIST addresses are generated: '0': Column address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its minimum/maximum value and only then is the row address incremented/decremented. '1': Row address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its minimum/maximum value and only then is the column address incremented/decremented."]
pub struct ROW_FIRST_R(crate::FieldReader<bool, bool>);
impl ROW_FIRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        ROW_FIRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROW_FIRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROW_FIRST` writer - Specifies how the Flash BIST addresses are generated: '0': Column address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its minimum/maximum value and only then is the row address incremented/decremented. '1': Row address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its minimum/maximum value and only then is the column address incremented/decremented."]
pub struct ROW_FIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> ROW_FIRST_W<'a> {
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
#[doc = "Field `ADDR_START_ENABLED` reader - Specifies Flash BIST start addresses: '0': Row and column addresses start with their maximum/minimum values. '1': Row and column addresses start with their values as specified by BIST_ADDR_START. This feature is supported only for simple increment/decrement patterns. It is not supported with address compliment pattern (BIST_CTL.ADDR_COMPLIMENT_ENABLED) or address pattern which increments/decrements both row address and column address (BIST_CTL.INCR_DECR_BOTH) for every read."]
pub struct ADDR_START_ENABLED_R(crate::FieldReader<bool, bool>);
impl ADDR_START_ENABLED_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADDR_START_ENABLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_START_ENABLED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR_START_ENABLED` writer - Specifies Flash BIST start addresses: '0': Row and column addresses start with their maximum/minimum values. '1': Row and column addresses start with their values as specified by BIST_ADDR_START. This feature is supported only for simple increment/decrement patterns. It is not supported with address compliment pattern (BIST_CTL.ADDR_COMPLIMENT_ENABLED) or address pattern which increments/decrements both row address and column address (BIST_CTL.INCR_DECR_BOTH) for every read."]
pub struct ADDR_START_ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_START_ENABLED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `ADDR_COMPLIMENT_ENABLED` reader - Specifies to generate address compliment patterns. 0: Generate normal increment/decrement patterns. 1: Generate address patterns which interleaves compliment of previous address in between. Example: The following is an example pattern, With UP=1 and ROW_FIRST =0 00_00 11_11 00_01 11_10 00_10 11_01 ..."]
pub struct ADDR_COMPLIMENT_ENABLED_R(crate::FieldReader<bool, bool>);
impl ADDR_COMPLIMENT_ENABLED_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADDR_COMPLIMENT_ENABLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_COMPLIMENT_ENABLED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR_COMPLIMENT_ENABLED` writer - Specifies to generate address compliment patterns. 0: Generate normal increment/decrement patterns. 1: Generate address patterns which interleaves compliment of previous address in between. Example: The following is an example pattern, With UP=1 and ROW_FIRST =0 00_00 11_11 00_01 11_10 00_10 11_01 ..."]
pub struct ADDR_COMPLIMENT_ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_COMPLIMENT_ENABLED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `INCR_DECR_BOTH` reader - Specifies to generate patterns where both column address and row address are incremented/decremented simultaneously. 0: Generate normal increment/decrement patterns. 1: Generate address patterns with both row and column address changing. Example: With UP = 1 and ROW_FIRST = 0 00_00 01_01 10_10 11_11 00_01 01_10 10_11 11_00 00_10 ..."]
pub struct INCR_DECR_BOTH_R(crate::FieldReader<bool, bool>);
impl INCR_DECR_BOTH_R {
    pub(crate) fn new(bits: bool) -> Self {
        INCR_DECR_BOTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INCR_DECR_BOTH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INCR_DECR_BOTH` writer - Specifies to generate patterns where both column address and row address are incremented/decremented simultaneously. 0: Generate normal increment/decrement patterns. 1: Generate address patterns with both row and column address changing. Example: With UP = 1 and ROW_FIRST = 0 00_00 01_01 10_10 11_11 00_01 01_10 10_11 11_00 00_10 ..."]
pub struct INCR_DECR_BOTH_W<'a> {
    w: &'a mut W,
}
impl<'a> INCR_DECR_BOTH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `STOP_ON_ERROR` reader - Specifies the BIST to continue indefinitely, regardless of occurrence of errors or not. 0: BIST controller doesn't stop on the data failures, it continues regardless of the errors. 1: BIST controller stops on when the first data failure is encountered."]
pub struct STOP_ON_ERROR_R(crate::FieldReader<bool, bool>);
impl STOP_ON_ERROR_R {
    pub(crate) fn new(bits: bool) -> Self {
        STOP_ON_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOP_ON_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOP_ON_ERROR` writer - Specifies the BIST to continue indefinitely, regardless of occurrence of errors or not. 0: BIST controller doesn't stop on the data failures, it continues regardless of the errors. 1: BIST controller stops on when the first data failure is encountered."]
pub struct STOP_ON_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_ON_ERROR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - This field specifies how the data check should be performed after reading the data from Flash memory. 0: Read the Flash and compare the output to BIST_DATA (R0). 1: Read the Flash and compare the output to the binary complement of BIST_DATA (R1). 3: Read the Flash and compare with BIST_DATA\\[\\]
and compliment of BIST_DATA alternately (R01). The expected data of the first read is BIST_DATA, expected data of the second read is binary compliment of BIST_DATA, third read expected data is BIST_DATA, fourth read expected data is binary compliment of BIST_DATA and so on."]
    #[inline(always)]
    pub fn opcode(&self) -> OPCODE_R {
        OPCODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Specifies direction in which Flash BIST steps through addresses: 0: BIST steps through the Flash from the maximum row and column addresses (as specified by a design time configuration parameter when ADDR_START_ENABLED is '0' and as specified by BIST_ADDR_START when ADDR_START_ENABLED is '1') to the minimum row and column addresses. 1: BIST steps through the Flash from the minimum row and column addresses ('0' when ADDR_START_ENABLED is '0' and as specified by BIST_ADDR_START when ADDR_START_ENABLED is '1'' to the maximum row and column addresses."]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Specifies how the Flash BIST addresses are generated: '0': Column address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its minimum/maximum value and only then is the row address incremented/decremented. '1': Row address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its minimum/maximum value and only then is the column address incremented/decremented."]
    #[inline(always)]
    pub fn row_first(&self) -> ROW_FIRST_R {
        ROW_FIRST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Specifies Flash BIST start addresses: '0': Row and column addresses start with their maximum/minimum values. '1': Row and column addresses start with their values as specified by BIST_ADDR_START. This feature is supported only for simple increment/decrement patterns. It is not supported with address compliment pattern (BIST_CTL.ADDR_COMPLIMENT_ENABLED) or address pattern which increments/decrements both row address and column address (BIST_CTL.INCR_DECR_BOTH) for every read."]
    #[inline(always)]
    pub fn addr_start_enabled(&self) -> ADDR_START_ENABLED_R {
        ADDR_START_ENABLED_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Specifies to generate address compliment patterns. 0: Generate normal increment/decrement patterns. 1: Generate address patterns which interleaves compliment of previous address in between. Example: The following is an example pattern, With UP=1 and ROW_FIRST =0 00_00 11_11 00_01 11_10 00_10 11_01 ..."]
    #[inline(always)]
    pub fn addr_compliment_enabled(&self) -> ADDR_COMPLIMENT_ENABLED_R {
        ADDR_COMPLIMENT_ENABLED_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Specifies to generate patterns where both column address and row address are incremented/decremented simultaneously. 0: Generate normal increment/decrement patterns. 1: Generate address patterns with both row and column address changing. Example: With UP = 1 and ROW_FIRST = 0 00_00 01_01 10_10 11_11 00_01 01_10 10_11 11_00 00_10 ..."]
    #[inline(always)]
    pub fn incr_decr_both(&self) -> INCR_DECR_BOTH_R {
        INCR_DECR_BOTH_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Specifies the BIST to continue indefinitely, regardless of occurrence of errors or not. 0: BIST controller doesn't stop on the data failures, it continues regardless of the errors. 1: BIST controller stops on when the first data failure is encountered."]
    #[inline(always)]
    pub fn stop_on_error(&self) -> STOP_ON_ERROR_R {
        STOP_ON_ERROR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - This field specifies how the data check should be performed after reading the data from Flash memory. 0: Read the Flash and compare the output to BIST_DATA (R0). 1: Read the Flash and compare the output to the binary complement of BIST_DATA (R1). 3: Read the Flash and compare with BIST_DATA\\[\\]
and compliment of BIST_DATA alternately (R01). The expected data of the first read is BIST_DATA, expected data of the second read is binary compliment of BIST_DATA, third read expected data is BIST_DATA, fourth read expected data is binary compliment of BIST_DATA and so on."]
    #[inline(always)]
    pub fn opcode(&mut self) -> OPCODE_W {
        OPCODE_W { w: self }
    }
    #[doc = "Bit 2 - Specifies direction in which Flash BIST steps through addresses: 0: BIST steps through the Flash from the maximum row and column addresses (as specified by a design time configuration parameter when ADDR_START_ENABLED is '0' and as specified by BIST_ADDR_START when ADDR_START_ENABLED is '1') to the minimum row and column addresses. 1: BIST steps through the Flash from the minimum row and column addresses ('0' when ADDR_START_ENABLED is '0' and as specified by BIST_ADDR_START when ADDR_START_ENABLED is '1'' to the maximum row and column addresses."]
    #[inline(always)]
    pub fn up(&mut self) -> UP_W {
        UP_W { w: self }
    }
    #[doc = "Bit 3 - Specifies how the Flash BIST addresses are generated: '0': Column address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its minimum/maximum value and only then is the row address incremented/decremented. '1': Row address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its minimum/maximum value and only then is the column address incremented/decremented."]
    #[inline(always)]
    pub fn row_first(&mut self) -> ROW_FIRST_W {
        ROW_FIRST_W { w: self }
    }
    #[doc = "Bit 4 - Specifies Flash BIST start addresses: '0': Row and column addresses start with their maximum/minimum values. '1': Row and column addresses start with their values as specified by BIST_ADDR_START. This feature is supported only for simple increment/decrement patterns. It is not supported with address compliment pattern (BIST_CTL.ADDR_COMPLIMENT_ENABLED) or address pattern which increments/decrements both row address and column address (BIST_CTL.INCR_DECR_BOTH) for every read."]
    #[inline(always)]
    pub fn addr_start_enabled(&mut self) -> ADDR_START_ENABLED_W {
        ADDR_START_ENABLED_W { w: self }
    }
    #[doc = "Bit 5 - Specifies to generate address compliment patterns. 0: Generate normal increment/decrement patterns. 1: Generate address patterns which interleaves compliment of previous address in between. Example: The following is an example pattern, With UP=1 and ROW_FIRST =0 00_00 11_11 00_01 11_10 00_10 11_01 ..."]
    #[inline(always)]
    pub fn addr_compliment_enabled(&mut self) -> ADDR_COMPLIMENT_ENABLED_W {
        ADDR_COMPLIMENT_ENABLED_W { w: self }
    }
    #[doc = "Bit 6 - Specifies to generate patterns where both column address and row address are incremented/decremented simultaneously. 0: Generate normal increment/decrement patterns. 1: Generate address patterns with both row and column address changing. Example: With UP = 1 and ROW_FIRST = 0 00_00 01_01 10_10 11_11 00_01 01_10 10_11 11_00 00_10 ..."]
    #[inline(always)]
    pub fn incr_decr_both(&mut self) -> INCR_DECR_BOTH_W {
        INCR_DECR_BOTH_W { w: self }
    }
    #[doc = "Bit 7 - Specifies the BIST to continue indefinitely, regardless of occurrence of errors or not. 0: BIST controller doesn't stop on the data failures, it continues regardless of the errors. 1: BIST controller stops on when the first data failure is encountered."]
    #[inline(always)]
    pub fn stop_on_error(&mut self) -> STOP_ON_ERROR_W {
        STOP_ON_ERROR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BIST control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bist_ctl](index.html) module"]
pub struct BIST_CTL_SPEC;
impl crate::RegisterSpec for BIST_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bist_ctl::R](R) reader structure"]
impl crate::Readable for BIST_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bist_ctl::W](W) writer structure"]
impl crate::Writable for BIST_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BIST_CTL to value 0"]
impl crate::Resettable for BIST_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
