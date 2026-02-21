use anyhow::Result;
use yazi_config::popup::InputKind;
use yazi_macro::act;
use yazi_parser::VoidOpt;
use yazi_shared::data::Data;
use yazi_widgets::input::parser::MoveOpt;

use crate::{Actor, Ctx};

pub struct FilterLeft;

impl Actor for FilterLeft {
	type Options = VoidOpt;

	const NAME: &str = "filter_left";

	fn act(cx: &mut Ctx, _: Self::Options) -> Result<Data> {
		if cx.input.kind == InputKind::Filter && cx.input.value().is_empty() {
			act!(mgr:leave, cx)
		} else {
			act!(r#move, cx.input, MoveOpt::from(-1isize))
		}
	}
}
