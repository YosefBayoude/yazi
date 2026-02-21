use anyhow::Result;
use yazi_config::popup::InputKind;
use yazi_macro::{act, succ};
use yazi_parser::{VoidOpt, input::CloseOpt};
use yazi_shared::data::Data;
use yazi_widgets::input::{InputError, parser::InsertOpt};

use crate::{Actor, Ctx};

pub struct FilterOpen;

impl Actor for FilterOpen {
	type Options = VoidOpt;

	const NAME: &str = "filter_open";

	fn act(cx: &mut Ctx, _: Self::Options) -> Result<Data> {
		let is_filter = cx.input.kind == InputKind::Filter;
		let is_dir = cx.hovered().map(|h| h.is_dir()).unwrap_or(false);

		if is_filter {
			if is_dir {
				if let Some(tx) = &cx.input.tx {
					let value = cx.input.snap().value.clone();
					tx.send(Err(InputError::Submitted(value))).ok();
				}
				let result = act!(mgr:enter, cx);

				let snap = cx.input.snap_mut();
				snap.value.clear();
				snap.cursor = 0;
				snap.offset = 0;
				cx.input.flush_value();
				let _ = act!(insert, cx.input, InsertOpt { append: false });

				result
			} else {
				succ!()
			}
		} else {
			act!(input:close, cx, CloseOpt { submit: true })
		}
	}
}
