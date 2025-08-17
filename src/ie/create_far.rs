// src/ie/create_far.rs

//! Create FAR Information Element.

use crate::ie::apply_action::ApplyAction;
use crate::ie::bar_id::BarId;
use crate::ie::duplicating_parameters::DuplicatingParameters;
use crate::ie::far_id::FarId;
use crate::ie::forwarding_parameters::ForwardingParameters;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateFar {
    pub far_id: FarId,
    pub apply_action: ApplyAction,
    pub forwarding_parameters: Option<ForwardingParameters>,
    pub duplicating_parameters: Option<DuplicatingParameters>,
    pub bar_id: Option<BarId>,
}
