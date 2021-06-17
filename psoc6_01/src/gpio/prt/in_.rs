#[doc = "Register `IN` reader"]
pub struct R(crate::R<IN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IN0` reader - IO pin state for pin 0 '0': Low logic level present on pin. '1': High logic level present on pin."]
pub struct IN0_R(crate::FieldReader<bool, bool>);
impl IN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        IN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN1` reader - IO pin state for pin 1"]
pub struct IN1_R(crate::FieldReader<bool, bool>);
impl IN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        IN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN2` reader - IO pin state for pin 2"]
pub struct IN2_R(crate::FieldReader<bool, bool>);
impl IN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        IN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN3` reader - IO pin state for pin 3"]
pub struct IN3_R(crate::FieldReader<bool, bool>);
impl IN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        IN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN4` reader - IO pin state for pin 4"]
pub struct IN4_R(crate::FieldReader<bool, bool>);
impl IN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        IN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN5` reader - IO pin state for pin 5"]
pub struct IN5_R(crate::FieldReader<bool, bool>);
impl IN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        IN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN6` reader - IO pin state for pin 6"]
pub struct IN6_R(crate::FieldReader<bool, bool>);
impl IN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        IN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN7` reader - IO pin state for pin 7"]
pub struct IN7_R(crate::FieldReader<bool, bool>);
impl IN7_R {
    pub(crate) fn new(bits: bool) -> Self {
        IN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT_IN` reader - Reads of this register return the logical state of the filtered pin as selected in the INTR_CFG.FLT_SEL register."]
pub struct FLT_IN_R(crate::FieldReader<bool, bool>);
impl FLT_IN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT_IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - IO pin state for pin 0 '0': Low logic level present on pin. '1': High logic level present on pin."]
    #[inline(always)]
    pub fn in0(&self) -> IN0_R {
        IN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IO pin state for pin 1"]
    #[inline(always)]
    pub fn in1(&self) -> IN1_R {
        IN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IO pin state for pin 2"]
    #[inline(always)]
    pub fn in2(&self) -> IN2_R {
        IN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IO pin state for pin 3"]
    #[inline(always)]
    pub fn in3(&self) -> IN3_R {
        IN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IO pin state for pin 4"]
    #[inline(always)]
    pub fn in4(&self) -> IN4_R {
        IN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IO pin state for pin 5"]
    #[inline(always)]
    pub fn in5(&self) -> IN5_R {
        IN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IO pin state for pin 6"]
    #[inline(always)]
    pub fn in6(&self) -> IN6_R {
        IN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IO pin state for pin 7"]
    #[inline(always)]
    pub fn in7(&self) -> IN7_R {
        IN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Reads of this register return the logical state of the filtered pin as selected in the INTR_CFG.FLT_SEL register."]
    #[inline(always)]
    pub fn flt_in(&self) -> FLT_IN_R {
        FLT_IN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
#[doc = "Port input state register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_](index.html) module"]
pub struct IN_SPEC;
impl crate::RegisterSpec for IN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_::R](R) reader structure"]
impl crate::Readable for IN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IN to value 0"]
impl crate::Resettable for IN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
