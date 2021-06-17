#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Location Register"]
    pub ilr: crate::Reg<ilr::ILR_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Clock Control Register"]
    pub ccr: crate::Reg<ccr::CCR_SPEC>,
    #[doc = "0x0c - Counter Increment Interrupt Register"]
    pub ciir: crate::Reg<ciir::CIIR_SPEC>,
    #[doc = "0x10 - Alarm Mask Register"]
    pub amr: crate::Reg<amr::AMR_SPEC>,
    #[doc = "0x14 - Consolidated Time Register 0"]
    pub ctime0: crate::Reg<ctime0::CTIME0_SPEC>,
    #[doc = "0x18 - Consolidated Time Register 1"]
    pub ctime1: crate::Reg<ctime1::CTIME1_SPEC>,
    #[doc = "0x1c - Consolidated Time Register 2"]
    pub ctime2: crate::Reg<ctime2::CTIME2_SPEC>,
    #[doc = "0x20 - Seconds Counter"]
    pub sec: crate::Reg<sec::SEC_SPEC>,
    #[doc = "0x24 - Minutes Register"]
    pub min: crate::Reg<min::MIN_SPEC>,
    #[doc = "0x28 - Hours Register"]
    pub hrs: crate::Reg<hrs::HRS_SPEC>,
    #[doc = "0x2c - Day of Month Register"]
    pub dom: crate::Reg<dom::DOM_SPEC>,
    #[doc = "0x30 - Day of Week Register"]
    pub dow: crate::Reg<dow::DOW_SPEC>,
    #[doc = "0x34 - Day of Year Register"]
    pub doy: crate::Reg<doy::DOY_SPEC>,
    #[doc = "0x38 - Months Register"]
    pub month: crate::Reg<month::MONTH_SPEC>,
    #[doc = "0x3c - Years Register"]
    pub year: crate::Reg<year::YEAR_SPEC>,
    #[doc = "0x40 - Calibration Value Register"]
    pub calibration: crate::Reg<calibration::CALIBRATION_SPEC>,
    #[doc = "0x44..0x58 - General Purpose Register 0"]
    pub gpreg: [crate::Reg<gpreg::GPREG_SPEC>; 5],
    #[doc = "0x58 - RTC Auxiliary Enable register"]
    pub rtc_auxen: crate::Reg<rtc_auxen::RTC_AUXEN_SPEC>,
    #[doc = "0x5c - RTC Auxiliary control register"]
    pub rtc_aux: crate::Reg<rtc_aux::RTC_AUX_SPEC>,
    #[doc = "0x60 - Alarm value for Seconds"]
    pub asec: crate::Reg<asec::ASEC_SPEC>,
    #[doc = "0x64 - Alarm value for Minutes"]
    pub amin: crate::Reg<amin::AMIN_SPEC>,
    #[doc = "0x68 - Alarm value for Hours"]
    pub ahrs: crate::Reg<ahrs::AHRS_SPEC>,
    #[doc = "0x6c - Alarm value for Day of Month"]
    pub adom: crate::Reg<adom::ADOM_SPEC>,
    #[doc = "0x70 - Alarm value for Day of Week"]
    pub adow: crate::Reg<adow::ADOW_SPEC>,
    #[doc = "0x74 - Alarm value for Day of Year"]
    pub adoy: crate::Reg<adoy::ADOY_SPEC>,
    #[doc = "0x78 - Alarm value for Months"]
    pub amon: crate::Reg<amon::AMON_SPEC>,
    #[doc = "0x7c - Alarm value for Year"]
    pub ayrs: crate::Reg<ayrs::AYRS_SPEC>,
    #[doc = "0x80 - Event Monitor/Recorder Status register. Contains status flags for event channels and other Event Monitor/Recorder conditions."]
    pub erstatus: crate::Reg<erstatus::ERSTATUS_SPEC>,
    #[doc = "0x84 - Event Monitor/Recorder Control register. Contains bits that control actions for the event channels as well as for Event Monitor/Recorder setup."]
    pub ercontrol: crate::Reg<ercontrol::ERCONTROL_SPEC>,
    #[doc = "0x88 - Event Monitor/Recorder Counters register. Allows reading the counters associated with the event channels."]
    pub ercounters: crate::Reg<ercounters::ERCOUNTERS_SPEC>,
    _reserved30: [u8; 0x04],
    #[doc = "0x90..0x9c - Event Monitor/Recorder First Stamp register for channel 0. Retains the time stamp for the first event on channel 0."]
    pub erfirststamp: [crate::Reg<erfirststamp::ERFIRSTSTAMP_SPEC>; 3],
    _reserved31: [u8; 0x04],
    #[doc = "0xa0..0xac - Event Monitor/Recorder Last Stamp register for channel 0. Retains the time stamp for the last (i.e. most recent) event on channel 0."]
    pub erlaststamp: [crate::Reg<erlaststamp::ERLASTSTAMP_SPEC>; 3],
}
#[doc = "ILR register accessor: an alias for `Reg<ILR_SPEC>`"]
pub type ILR = crate::Reg<ilr::ILR_SPEC>;
#[doc = "Interrupt Location Register"]
pub mod ilr;
#[doc = "CCR register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Clock Control Register"]
pub mod ccr;
#[doc = "CIIR register accessor: an alias for `Reg<CIIR_SPEC>`"]
pub type CIIR = crate::Reg<ciir::CIIR_SPEC>;
#[doc = "Counter Increment Interrupt Register"]
pub mod ciir;
#[doc = "AMR register accessor: an alias for `Reg<AMR_SPEC>`"]
pub type AMR = crate::Reg<amr::AMR_SPEC>;
#[doc = "Alarm Mask Register"]
pub mod amr;
#[doc = "CTIME0 register accessor: an alias for `Reg<CTIME0_SPEC>`"]
pub type CTIME0 = crate::Reg<ctime0::CTIME0_SPEC>;
#[doc = "Consolidated Time Register 0"]
pub mod ctime0;
#[doc = "CTIME1 register accessor: an alias for `Reg<CTIME1_SPEC>`"]
pub type CTIME1 = crate::Reg<ctime1::CTIME1_SPEC>;
#[doc = "Consolidated Time Register 1"]
pub mod ctime1;
#[doc = "CTIME2 register accessor: an alias for `Reg<CTIME2_SPEC>`"]
pub type CTIME2 = crate::Reg<ctime2::CTIME2_SPEC>;
#[doc = "Consolidated Time Register 2"]
pub mod ctime2;
#[doc = "SEC register accessor: an alias for `Reg<SEC_SPEC>`"]
pub type SEC = crate::Reg<sec::SEC_SPEC>;
#[doc = "Seconds Counter"]
pub mod sec;
#[doc = "MIN register accessor: an alias for `Reg<MIN_SPEC>`"]
pub type MIN = crate::Reg<min::MIN_SPEC>;
#[doc = "Minutes Register"]
pub mod min;
#[doc = "HRS register accessor: an alias for `Reg<HRS_SPEC>`"]
pub type HRS = crate::Reg<hrs::HRS_SPEC>;
#[doc = "Hours Register"]
pub mod hrs;
#[doc = "DOM register accessor: an alias for `Reg<DOM_SPEC>`"]
pub type DOM = crate::Reg<dom::DOM_SPEC>;
#[doc = "Day of Month Register"]
pub mod dom;
#[doc = "DOW register accessor: an alias for `Reg<DOW_SPEC>`"]
pub type DOW = crate::Reg<dow::DOW_SPEC>;
#[doc = "Day of Week Register"]
pub mod dow;
#[doc = "DOY register accessor: an alias for `Reg<DOY_SPEC>`"]
pub type DOY = crate::Reg<doy::DOY_SPEC>;
#[doc = "Day of Year Register"]
pub mod doy;
#[doc = "MONTH register accessor: an alias for `Reg<MONTH_SPEC>`"]
pub type MONTH = crate::Reg<month::MONTH_SPEC>;
#[doc = "Months Register"]
pub mod month;
#[doc = "YEAR register accessor: an alias for `Reg<YEAR_SPEC>`"]
pub type YEAR = crate::Reg<year::YEAR_SPEC>;
#[doc = "Years Register"]
pub mod year;
#[doc = "CALIBRATION register accessor: an alias for `Reg<CALIBRATION_SPEC>`"]
pub type CALIBRATION = crate::Reg<calibration::CALIBRATION_SPEC>;
#[doc = "Calibration Value Register"]
pub mod calibration;
#[doc = "GPREG register accessor: an alias for `Reg<GPREG_SPEC>`"]
pub type GPREG = crate::Reg<gpreg::GPREG_SPEC>;
#[doc = "General Purpose Register 0"]
pub mod gpreg;
#[doc = "RTC_AUX register accessor: an alias for `Reg<RTC_AUX_SPEC>`"]
pub type RTC_AUX = crate::Reg<rtc_aux::RTC_AUX_SPEC>;
#[doc = "RTC Auxiliary control register"]
pub mod rtc_aux;
#[doc = "RTC_AUXEN register accessor: an alias for `Reg<RTC_AUXEN_SPEC>`"]
pub type RTC_AUXEN = crate::Reg<rtc_auxen::RTC_AUXEN_SPEC>;
#[doc = "RTC Auxiliary Enable register"]
pub mod rtc_auxen;
#[doc = "ASEC register accessor: an alias for `Reg<ASEC_SPEC>`"]
pub type ASEC = crate::Reg<asec::ASEC_SPEC>;
#[doc = "Alarm value for Seconds"]
pub mod asec;
#[doc = "AMIN register accessor: an alias for `Reg<AMIN_SPEC>`"]
pub type AMIN = crate::Reg<amin::AMIN_SPEC>;
#[doc = "Alarm value for Minutes"]
pub mod amin;
#[doc = "AHRS register accessor: an alias for `Reg<AHRS_SPEC>`"]
pub type AHRS = crate::Reg<ahrs::AHRS_SPEC>;
#[doc = "Alarm value for Hours"]
pub mod ahrs;
#[doc = "ADOM register accessor: an alias for `Reg<ADOM_SPEC>`"]
pub type ADOM = crate::Reg<adom::ADOM_SPEC>;
#[doc = "Alarm value for Day of Month"]
pub mod adom;
#[doc = "ADOW register accessor: an alias for `Reg<ADOW_SPEC>`"]
pub type ADOW = crate::Reg<adow::ADOW_SPEC>;
#[doc = "Alarm value for Day of Week"]
pub mod adow;
#[doc = "ADOY register accessor: an alias for `Reg<ADOY_SPEC>`"]
pub type ADOY = crate::Reg<adoy::ADOY_SPEC>;
#[doc = "Alarm value for Day of Year"]
pub mod adoy;
#[doc = "AMON register accessor: an alias for `Reg<AMON_SPEC>`"]
pub type AMON = crate::Reg<amon::AMON_SPEC>;
#[doc = "Alarm value for Months"]
pub mod amon;
#[doc = "AYRS register accessor: an alias for `Reg<AYRS_SPEC>`"]
pub type AYRS = crate::Reg<ayrs::AYRS_SPEC>;
#[doc = "Alarm value for Year"]
pub mod ayrs;
#[doc = "ERCONTROL register accessor: an alias for `Reg<ERCONTROL_SPEC>`"]
pub type ERCONTROL = crate::Reg<ercontrol::ERCONTROL_SPEC>;
#[doc = "Event Monitor/Recorder Control register. Contains bits that control actions for the event channels as well as for Event Monitor/Recorder setup."]
pub mod ercontrol;
#[doc = "ERSTATUS register accessor: an alias for `Reg<ERSTATUS_SPEC>`"]
pub type ERSTATUS = crate::Reg<erstatus::ERSTATUS_SPEC>;
#[doc = "Event Monitor/Recorder Status register. Contains status flags for event channels and other Event Monitor/Recorder conditions."]
pub mod erstatus;
#[doc = "ERCOUNTERS register accessor: an alias for `Reg<ERCOUNTERS_SPEC>`"]
pub type ERCOUNTERS = crate::Reg<ercounters::ERCOUNTERS_SPEC>;
#[doc = "Event Monitor/Recorder Counters register. Allows reading the counters associated with the event channels."]
pub mod ercounters;
#[doc = "ERFIRSTSTAMP register accessor: an alias for `Reg<ERFIRSTSTAMP_SPEC>`"]
pub type ERFIRSTSTAMP = crate::Reg<erfirststamp::ERFIRSTSTAMP_SPEC>;
#[doc = "Event Monitor/Recorder First Stamp register for channel 0. Retains the time stamp for the first event on channel 0."]
pub mod erfirststamp;
#[doc = "ERLASTSTAMP register accessor: an alias for `Reg<ERLASTSTAMP_SPEC>`"]
pub type ERLASTSTAMP = crate::Reg<erlaststamp::ERLASTSTAMP_SPEC>;
#[doc = "Event Monitor/Recorder Last Stamp register for channel 0. Retains the time stamp for the last (i.e. most recent) event on channel 0."]
pub mod erlaststamp;
