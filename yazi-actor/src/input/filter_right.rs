use anyhow::Result;
use yazi_config::popup::InputKind;
use yazi_macro::act;
use yazi_parser::VoidOpt;
use yazi_shared::data::Data;
use yazi_widgets::input::parser::MoveOpt;

use crate::{Actor, Ctx};

pub struct FilterRight;

impl Actor for FilterRight {
	type Options = VoidOpt;

	const NAME: &str = "filter_right";

	fn act(cx: &mut Ctx, _: Self::Options) -> Result<Data> {
		if cx.input.kind == InputKind::Filter && cx.input.value().is_empty() {
			act!(mgr:enter, cx)
		} else {
			act!(r#move, cx.input, MoveOpt::from(1isize))
		}
	}
}
