use anyhow::Result;
use yazi_config::popup::InputKind;
use yazi_macro::act;
use yazi_shared::data::Data;
use yazi_widgets::input::{InputError, parser::BackspaceOpt};

use crate::{Actor, Ctx};

pub struct FilterLeave;

impl Actor for FilterLeave {
	type Options = BackspaceOpt;

	const NAME: &str = "filter_leave";

	fn act(cx: &mut Ctx, opt: Self::Options) -> Result<Data> {
		let is_filter = cx.input.kind == InputKind::Filter;
		let is_empty = cx.input.value().is_empty();

		if is_filter && is_empty && !opt.under {
			if let Some(tx) = &cx.input.tx {
				tx.send(Err(InputError::Submitted(String::new()))).ok();
			}
			act!(mgr:leave, cx)
		} else {
			act!(backspace, cx.input, opt)
		}
	}
}
