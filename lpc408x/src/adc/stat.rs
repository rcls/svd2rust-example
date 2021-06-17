#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DONE0` reader - This bit mirrors the DONE status flag from the result register for A/D channel 0."]
pub struct DONE0_R(crate::FieldReader<bool, bool>);
impl DONE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        DONE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DONE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DONE1` reader - This bit mirrors the DONE status flag from the result register for A/D channel 1."]
pub struct DONE1_R(crate::FieldReader<bool, bool>);
impl DONE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DONE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DONE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DONE2` reader - This bit mirrors the DONE status flag from the result register for A/D channel 2."]
pub struct DONE2_R(crate::FieldReader<bool, bool>);
impl DONE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DONE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DONE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DONE3` reader - This bit mirrors the DONE status flag from the result register for A/D channel 3."]
pub struct DONE3_R(crate::FieldReader<bool, bool>);
impl DONE3_R {
    pub(crate) fn new(bits: bool) -> Self {
        DONE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DONE3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DONE4` reader - This bit mirrors the DONE status flag from the result register for A/D channel 4."]
pub struct DONE4_R(crate::FieldReader<bool, bool>);
impl DONE4_R {
    pub(crate) fn new(bits: bool) -> Self {
        DONE4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DONE4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DONE5` reader - This bit mirrors the DONE status flag from the result register for A/D channel 5."]
pub struct DONE5_R(crate::FieldReader<bool, bool>);
impl DONE5_R {
    pub(crate) fn new(bits: bool) -> Self {
        DONE5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DONE5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DONE6` reader - This bit mirrors the DONE status flag from the result register for A/D channel 6."]
pub struct DONE6_R(crate::FieldReader<bool, bool>);
impl DONE6_R {
    pub(crate) fn new(bits: bool) -> Self {
        DONE6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DONE6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DONE7` reader - This bit mirrors the DONE status flag from the result register for A/D channel 7."]
pub struct DONE7_R(crate::FieldReader<bool, bool>);
impl DONE7_R {
    pub(crate) fn new(bits: bool) -> Self {
        DONE7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DONE7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRUN0` reader - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 0."]
pub struct OVERRUN0_R(crate::FieldReader<bool, bool>);
impl OVERRUN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRUN1` reader - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 1."]
pub struct OVERRUN1_R(crate::FieldReader<bool, bool>);
impl OVERRUN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRUN2` reader - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 2."]
pub struct OVERRUN2_R(crate::FieldReader<bool, bool>);
impl OVERRUN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRUN3` reader - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 3."]
pub struct OVERRUN3_R(crate::FieldReader<bool, bool>);
impl OVERRUN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRUN4` reader - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 4."]
pub struct OVERRUN4_R(crate::FieldReader<bool, bool>);
impl OVERRUN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRUN5` reader - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 5."]
pub struct OVERRUN5_R(crate::FieldReader<bool, bool>);
impl OVERRUN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRUN6` reader - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 6."]
pub struct OVERRUN6_R(crate::FieldReader<bool, bool>);
impl OVERRUN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRUN7` reader - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 7."]
pub struct OVERRUN7_R(crate::FieldReader<bool, bool>);
impl OVERRUN7_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADINT` reader - This bit is the A/D interrupt flag. It is one when any of the individual A/D channel Done flags is asserted and enabled to contribute to the A/D interrupt via the ADINTEN register."]
pub struct ADINT_R(crate::FieldReader<bool, bool>);
impl ADINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - This bit mirrors the DONE status flag from the result register for A/D channel 0."]
    #[inline(always)]
    pub fn done0(&self) -> DONE0_R {
        DONE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit mirrors the DONE status flag from the result register for A/D channel 1."]
    #[inline(always)]
    pub fn done1(&self) -> DONE1_R {
        DONE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit mirrors the DONE status flag from the result register for A/D channel 2."]
    #[inline(always)]
    pub fn done2(&self) -> DONE2_R {
        DONE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit mirrors the DONE status flag from the result register for A/D channel 3."]
    #[inline(always)]
    pub fn done3(&self) -> DONE3_R {
        DONE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit mirrors the DONE status flag from the result register for A/D channel 4."]
    #[inline(always)]
    pub fn done4(&self) -> DONE4_R {
        DONE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit mirrors the DONE status flag from the result register for A/D channel 5."]
    #[inline(always)]
    pub fn done5(&self) -> DONE5_R {
        DONE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This bit mirrors the DONE status flag from the result register for A/D channel 6."]
    #[inline(always)]
    pub fn done6(&self) -> DONE6_R {
        DONE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit mirrors the DONE status flag from the result register for A/D channel 7."]
    #[inline(always)]
    pub fn done7(&self) -> DONE7_R {
        DONE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 0."]
    #[inline(always)]
    pub fn overrun0(&self) -> OVERRUN0_R {
        OVERRUN0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 1."]
    #[inline(always)]
    pub fn overrun1(&self) -> OVERRUN1_R {
        OVERRUN1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 2."]
    #[inline(always)]
    pub fn overrun2(&self) -> OVERRUN2_R {
        OVERRUN2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 3."]
    #[inline(always)]
    pub fn overrun3(&self) -> OVERRUN3_R {
        OVERRUN3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 4."]
    #[inline(always)]
    pub fn overrun4(&self) -> OVERRUN4_R {
        OVERRUN4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 5."]
    #[inline(always)]
    pub fn overrun5(&self) -> OVERRUN5_R {
        OVERRUN5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 6."]
    #[inline(always)]
    pub fn overrun6(&self) -> OVERRUN6_R {
        OVERRUN6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 7."]
    #[inline(always)]
    pub fn overrun7(&self) -> OVERRUN7_R {
        OVERRUN7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - This bit is the A/D interrupt flag. It is one when any of the individual A/D channel Done flags is asserted and enabled to contribute to the A/D interrupt via the ADINTEN register."]
    #[inline(always)]
    pub fn adint(&self) -> ADINT_R {
        ADINT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
#[doc = "A/D Status Register. This register contains DONE and OVERRUN flags for all of the A/D channels, as well as the A/D interrupt/DMA flag.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
