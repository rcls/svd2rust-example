#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Controls operation of the memory controller."]
    pub control: crate::Reg<control::CONTROL_SPEC>,
    #[doc = "0x04 - Provides EMC status information."]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x08 - Configures operation of the memory controller"]
    pub config: crate::Reg<config::CONFIG_SPEC>,
    _reserved3: [u8; 0x14],
    #[doc = "0x20 - Controls dynamic memory operation."]
    pub dynamiccontrol: crate::Reg<dynamiccontrol::DYNAMICCONTROL_SPEC>,
    #[doc = "0x24 - Configures dynamic memory refresh."]
    pub dynamicrefresh: crate::Reg<dynamicrefresh::DYNAMICREFRESH_SPEC>,
    #[doc = "0x28 - Configures dynamic memory read strategy."]
    pub dynamicreadconfig: crate::Reg<dynamicreadconfig::DYNAMICREADCONFIG_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x30 - Precharge command period."]
    pub dynamicrp: crate::Reg<dynamicrp::DYNAMICRP_SPEC>,
    #[doc = "0x34 - Active to precharge command period."]
    pub dynamicras: crate::Reg<dynamicras::DYNAMICRAS_SPEC>,
    #[doc = "0x38 - Self-refresh exit time."]
    pub dynamicsrex: crate::Reg<dynamicsrex::DYNAMICSREX_SPEC>,
    #[doc = "0x3c - Last-data-out to active command time."]
    pub dynamicapr: crate::Reg<dynamicapr::DYNAMICAPR_SPEC>,
    #[doc = "0x40 - Data-in to active command time."]
    pub dynamicdal: crate::Reg<dynamicdal::DYNAMICDAL_SPEC>,
    #[doc = "0x44 - Write recovery time."]
    pub dynamicwr: crate::Reg<dynamicwr::DYNAMICWR_SPEC>,
    #[doc = "0x48 - Selects the active to active command period."]
    pub dynamicrc: crate::Reg<dynamicrc::DYNAMICRC_SPEC>,
    #[doc = "0x4c - Selects the auto-refresh period."]
    pub dynamicrfc: crate::Reg<dynamicrfc::DYNAMICRFC_SPEC>,
    #[doc = "0x50 - Time for exit self-refresh to active command."]
    pub dynamicxsr: crate::Reg<dynamicxsr::DYNAMICXSR_SPEC>,
    #[doc = "0x54 - Latency for active bank A to active bank B."]
    pub dynamicrrd: crate::Reg<dynamicrrd::DYNAMICRRD_SPEC>,
    #[doc = "0x58 - Time for load mode register to active command."]
    pub dynamicmrd: crate::Reg<dynamicmrd::DYNAMICMRD_SPEC>,
    _reserved17: [u8; 0x24],
    #[doc = "0x80 - Time for long static memory read and write transfers."]
    pub staticextendedwait: crate::Reg<staticextendedwait::STATICEXTENDEDWAIT_SPEC>,
    _reserved18: [u8; 0x7c],
    #[doc = "0x100 - Configuration information for EMC_DYCS0."]
    pub dynamicconfig0: crate::Reg<dynamicconfig::DYNAMICCONFIG_SPEC>,
    #[doc = "0x104 - RAS and CAS latencies for EMC_DYCS0."]
    pub dynamicrascas0: crate::Reg<dynamicrascas::DYNAMICRASCAS_SPEC>,
    _reserved20: [u8; 0x18],
    #[doc = "0x120 - Configuration information for EMC_DYCS0."]
    pub dynamicconfig1: crate::Reg<dynamicconfig::DYNAMICCONFIG_SPEC>,
    #[doc = "0x124 - RAS and CAS latencies for EMC_DYCS0."]
    pub dynamicrascas1: crate::Reg<dynamicrascas::DYNAMICRASCAS_SPEC>,
    _reserved22: [u8; 0x18],
    #[doc = "0x140 - Configuration information for EMC_DYCS0."]
    pub dynamicconfig2: crate::Reg<dynamicconfig::DYNAMICCONFIG_SPEC>,
    #[doc = "0x144 - RAS and CAS latencies for EMC_DYCS0."]
    pub dynamicrascas2: crate::Reg<dynamicrascas::DYNAMICRASCAS_SPEC>,
    _reserved24: [u8; 0x18],
    #[doc = "0x160 - Configuration information for EMC_DYCS0."]
    pub dynamicconfig3: crate::Reg<dynamicconfig::DYNAMICCONFIG_SPEC>,
    #[doc = "0x164 - RAS and CAS latencies for EMC_DYCS0."]
    pub dynamicrascas3: crate::Reg<dynamicrascas::DYNAMICRASCAS_SPEC>,
    _reserved26: [u8; 0x98],
    #[doc = "0x200 - Configuration for EMC_CS0."]
    pub staticconfig0: crate::Reg<staticconfig::STATICCONFIG_SPEC>,
    #[doc = "0x204 - Delay from EMC_CS0 to write enable."]
    pub staticwaitwen0: crate::Reg<staticwaitwen::STATICWAITWEN_SPEC>,
    #[doc = "0x208 - Delay from EMC_CS0 or address change, whichever is later, to output enable."]
    pub staticwaitoen0: crate::Reg<staticwaitoen::STATICWAITOEN_SPEC>,
    #[doc = "0x20c - Delay from EMC_CS0 to a read access."]
    pub staticwaitrd0: crate::Reg<staticwaitrd::STATICWAITRD_SPEC>,
    #[doc = "0x210 - Delay for asynchronous page mode sequential accesses for EMC_CS0."]
    pub staticwaitpage0: crate::Reg<staticwaitpage::STATICWAITPAGE_SPEC>,
    #[doc = "0x214 - Delay from EMC_CS0 to a write access."]
    pub staticwaitwr0: crate::Reg<staticwaitwr::STATICWAITWR_SPEC>,
    #[doc = "0x218 - Number of bus turnaround cycles EMC_CS0."]
    pub staticwaitturn0: crate::Reg<staticwaitturn::STATICWAITTURN_SPEC>,
    _reserved33: [u8; 0x04],
    #[doc = "0x220 - Configuration for EMC_CS0."]
    pub staticconfig1: crate::Reg<staticconfig::STATICCONFIG_SPEC>,
    #[doc = "0x224 - Delay from EMC_CS0 to write enable."]
    pub staticwaitwen1: crate::Reg<staticwaitwen::STATICWAITWEN_SPEC>,
    #[doc = "0x228 - Delay from EMC_CS0 or address change, whichever is later, to output enable."]
    pub staticwaitoen1: crate::Reg<staticwaitoen::STATICWAITOEN_SPEC>,
    #[doc = "0x22c - Delay from EMC_CS0 to a read access."]
    pub staticwaitrd1: crate::Reg<staticwaitrd::STATICWAITRD_SPEC>,
    #[doc = "0x230 - Delay for asynchronous page mode sequential accesses for EMC_CS0."]
    pub staticwaitpage1: crate::Reg<staticwaitpage::STATICWAITPAGE_SPEC>,
    #[doc = "0x234 - Delay from EMC_CS0 to a write access."]
    pub staticwaitwr1: crate::Reg<staticwaitwr::STATICWAITWR_SPEC>,
    #[doc = "0x238 - Number of bus turnaround cycles EMC_CS0."]
    pub staticwaitturn1: crate::Reg<staticwaitturn::STATICWAITTURN_SPEC>,
    _reserved40: [u8; 0x04],
    #[doc = "0x240 - Configuration for EMC_CS0."]
    pub staticconfig2: crate::Reg<staticconfig::STATICCONFIG_SPEC>,
    #[doc = "0x244 - Delay from EMC_CS0 to write enable."]
    pub staticwaitwen2: crate::Reg<staticwaitwen::STATICWAITWEN_SPEC>,
    #[doc = "0x248 - Delay from EMC_CS0 or address change, whichever is later, to output enable."]
    pub staticwaitoen2: crate::Reg<staticwaitoen::STATICWAITOEN_SPEC>,
    #[doc = "0x24c - Delay from EMC_CS0 to a read access."]
    pub staticwaitrd2: crate::Reg<staticwaitrd::STATICWAITRD_SPEC>,
    #[doc = "0x250 - Delay for asynchronous page mode sequential accesses for EMC_CS0."]
    pub staticwaitpage2: crate::Reg<staticwaitpage::STATICWAITPAGE_SPEC>,
    #[doc = "0x254 - Delay from EMC_CS0 to a write access."]
    pub staticwaitwr2: crate::Reg<staticwaitwr::STATICWAITWR_SPEC>,
    #[doc = "0x258 - Number of bus turnaround cycles EMC_CS0."]
    pub staticwaitturn2: crate::Reg<staticwaitturn::STATICWAITTURN_SPEC>,
    _reserved47: [u8; 0x04],
    #[doc = "0x260 - Configuration for EMC_CS0."]
    pub staticconfig3: crate::Reg<staticconfig::STATICCONFIG_SPEC>,
    #[doc = "0x264 - Delay from EMC_CS0 to write enable."]
    pub staticwaitwen3: crate::Reg<staticwaitwen::STATICWAITWEN_SPEC>,
    #[doc = "0x268 - Delay from EMC_CS0 or address change, whichever is later, to output enable."]
    pub staticwaitoen3: crate::Reg<staticwaitoen::STATICWAITOEN_SPEC>,
    #[doc = "0x26c - Delay from EMC_CS0 to a read access."]
    pub staticwaitrd3: crate::Reg<staticwaitrd::STATICWAITRD_SPEC>,
    #[doc = "0x270 - Delay for asynchronous page mode sequential accesses for EMC_CS0."]
    pub staticwaitpage3: crate::Reg<staticwaitpage::STATICWAITPAGE_SPEC>,
    #[doc = "0x274 - Delay from EMC_CS0 to a write access."]
    pub staticwaitwr3: crate::Reg<staticwaitwr::STATICWAITWR_SPEC>,
    #[doc = "0x278 - Number of bus turnaround cycles EMC_CS0."]
    pub staticwaitturn3: crate::Reg<staticwaitturn::STATICWAITTURN_SPEC>,
}
#[doc = "CONTROL register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Controls operation of the memory controller."]
pub mod control;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Provides EMC status information."]
pub mod status;
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configures operation of the memory controller"]
pub mod config;
#[doc = "DYNAMICCONTROL register accessor: an alias for `Reg<DYNAMICCONTROL_SPEC>`"]
pub type DYNAMICCONTROL = crate::Reg<dynamiccontrol::DYNAMICCONTROL_SPEC>;
#[doc = "Controls dynamic memory operation."]
pub mod dynamiccontrol;
#[doc = "DYNAMICREFRESH register accessor: an alias for `Reg<DYNAMICREFRESH_SPEC>`"]
pub type DYNAMICREFRESH = crate::Reg<dynamicrefresh::DYNAMICREFRESH_SPEC>;
#[doc = "Configures dynamic memory refresh."]
pub mod dynamicrefresh;
#[doc = "DYNAMICREADCONFIG register accessor: an alias for `Reg<DYNAMICREADCONFIG_SPEC>`"]
pub type DYNAMICREADCONFIG = crate::Reg<dynamicreadconfig::DYNAMICREADCONFIG_SPEC>;
#[doc = "Configures dynamic memory read strategy."]
pub mod dynamicreadconfig;
#[doc = "DYNAMICRP register accessor: an alias for `Reg<DYNAMICRP_SPEC>`"]
pub type DYNAMICRP = crate::Reg<dynamicrp::DYNAMICRP_SPEC>;
#[doc = "Precharge command period."]
pub mod dynamicrp;
#[doc = "DYNAMICRAS register accessor: an alias for `Reg<DYNAMICRAS_SPEC>`"]
pub type DYNAMICRAS = crate::Reg<dynamicras::DYNAMICRAS_SPEC>;
#[doc = "Active to precharge command period."]
pub mod dynamicras;
#[doc = "DYNAMICSREX register accessor: an alias for `Reg<DYNAMICSREX_SPEC>`"]
pub type DYNAMICSREX = crate::Reg<dynamicsrex::DYNAMICSREX_SPEC>;
#[doc = "Self-refresh exit time."]
pub mod dynamicsrex;
#[doc = "DYNAMICAPR register accessor: an alias for `Reg<DYNAMICAPR_SPEC>`"]
pub type DYNAMICAPR = crate::Reg<dynamicapr::DYNAMICAPR_SPEC>;
#[doc = "Last-data-out to active command time."]
pub mod dynamicapr;
#[doc = "DYNAMICDAL register accessor: an alias for `Reg<DYNAMICDAL_SPEC>`"]
pub type DYNAMICDAL = crate::Reg<dynamicdal::DYNAMICDAL_SPEC>;
#[doc = "Data-in to active command time."]
pub mod dynamicdal;
#[doc = "DYNAMICWR register accessor: an alias for `Reg<DYNAMICWR_SPEC>`"]
pub type DYNAMICWR = crate::Reg<dynamicwr::DYNAMICWR_SPEC>;
#[doc = "Write recovery time."]
pub mod dynamicwr;
#[doc = "DYNAMICRC register accessor: an alias for `Reg<DYNAMICRC_SPEC>`"]
pub type DYNAMICRC = crate::Reg<dynamicrc::DYNAMICRC_SPEC>;
#[doc = "Selects the active to active command period."]
pub mod dynamicrc;
#[doc = "DYNAMICRFC register accessor: an alias for `Reg<DYNAMICRFC_SPEC>`"]
pub type DYNAMICRFC = crate::Reg<dynamicrfc::DYNAMICRFC_SPEC>;
#[doc = "Selects the auto-refresh period."]
pub mod dynamicrfc;
#[doc = "DYNAMICXSR register accessor: an alias for `Reg<DYNAMICXSR_SPEC>`"]
pub type DYNAMICXSR = crate::Reg<dynamicxsr::DYNAMICXSR_SPEC>;
#[doc = "Time for exit self-refresh to active command."]
pub mod dynamicxsr;
#[doc = "DYNAMICRRD register accessor: an alias for `Reg<DYNAMICRRD_SPEC>`"]
pub type DYNAMICRRD = crate::Reg<dynamicrrd::DYNAMICRRD_SPEC>;
#[doc = "Latency for active bank A to active bank B."]
pub mod dynamicrrd;
#[doc = "DYNAMICMRD register accessor: an alias for `Reg<DYNAMICMRD_SPEC>`"]
pub type DYNAMICMRD = crate::Reg<dynamicmrd::DYNAMICMRD_SPEC>;
#[doc = "Time for load mode register to active command."]
pub mod dynamicmrd;
#[doc = "STATICEXTENDEDWAIT register accessor: an alias for `Reg<STATICEXTENDEDWAIT_SPEC>`"]
pub type STATICEXTENDEDWAIT = crate::Reg<staticextendedwait::STATICEXTENDEDWAIT_SPEC>;
#[doc = "Time for long static memory read and write transfers."]
pub mod staticextendedwait;
#[doc = "DYNAMICCONFIG register accessor: an alias for `Reg<DYNAMICCONFIG_SPEC>`"]
pub type DYNAMICCONFIG = crate::Reg<dynamicconfig::DYNAMICCONFIG_SPEC>;
#[doc = "Configuration information for EMC_DYCS0."]
pub mod dynamicconfig;
#[doc = "DYNAMICRASCAS register accessor: an alias for `Reg<DYNAMICRASCAS_SPEC>`"]
pub type DYNAMICRASCAS = crate::Reg<dynamicrascas::DYNAMICRASCAS_SPEC>;
#[doc = "RAS and CAS latencies for EMC_DYCS0."]
pub mod dynamicrascas;
#[doc = "STATICCONFIG register accessor: an alias for `Reg<STATICCONFIG_SPEC>`"]
pub type STATICCONFIG = crate::Reg<staticconfig::STATICCONFIG_SPEC>;
#[doc = "Configuration for EMC_CS0."]
pub mod staticconfig;
#[doc = "STATICWAITWEN register accessor: an alias for `Reg<STATICWAITWEN_SPEC>`"]
pub type STATICWAITWEN = crate::Reg<staticwaitwen::STATICWAITWEN_SPEC>;
#[doc = "Delay from EMC_CS0 to write enable."]
pub mod staticwaitwen;
#[doc = "STATICWAITOEN register accessor: an alias for `Reg<STATICWAITOEN_SPEC>`"]
pub type STATICWAITOEN = crate::Reg<staticwaitoen::STATICWAITOEN_SPEC>;
#[doc = "Delay from EMC_CS0 or address change, whichever is later, to output enable."]
pub mod staticwaitoen;
#[doc = "STATICWAITRD register accessor: an alias for `Reg<STATICWAITRD_SPEC>`"]
pub type STATICWAITRD = crate::Reg<staticwaitrd::STATICWAITRD_SPEC>;
#[doc = "Delay from EMC_CS0 to a read access."]
pub mod staticwaitrd;
#[doc = "STATICWAITPAGE register accessor: an alias for `Reg<STATICWAITPAGE_SPEC>`"]
pub type STATICWAITPAGE = crate::Reg<staticwaitpage::STATICWAITPAGE_SPEC>;
#[doc = "Delay for asynchronous page mode sequential accesses for EMC_CS0."]
pub mod staticwaitpage;
#[doc = "STATICWAITWR register accessor: an alias for `Reg<STATICWAITWR_SPEC>`"]
pub type STATICWAITWR = crate::Reg<staticwaitwr::STATICWAITWR_SPEC>;
#[doc = "Delay from EMC_CS0 to a write access."]
pub mod staticwaitwr;
#[doc = "STATICWAITTURN register accessor: an alias for `Reg<STATICWAITTURN_SPEC>`"]
pub type STATICWAITTURN = crate::Reg<staticwaitturn::STATICWAITTURN_SPEC>;
#[doc = "Number of bus turnaround cycles EMC_CS0."]
pub mod staticwaitturn;
