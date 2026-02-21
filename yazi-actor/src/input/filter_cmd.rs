use anyhow::Result;
use yazi_config::popup::InputKind;
use yazi_macro::{act, succ};
use yazi_parser::ArrowOpt;
use yazi_parser::input::FilterCmdOpt;
use yazi_parser::mgr::{CopyOpt, CreateOpt, PasteOpt, RenameOpt, ToggleOpt, YankOpt};
use yazi_parser::notify::{PushLevel, PushOpt};
use yazi_proxy::NotifyProxy;
use yazi_shared::data::Data;
use yazi_widgets::input::parser::InsertOpt;

use crate::{Actor, Ctx};

pub struct FilterCmd;

impl Actor for FilterCmd {
	type Options = FilterCmdOpt;

	const NAME: &str = "filter_cmd";

	fn act(cx: &mut Ctx, opt: Self::Options) -> Result<Data> {
		let is_filter = cx.input.kind == InputKind::Filter;

		match &*opt.cmd {
			// up/down have non-filter fallbacks
			"up" if is_filter => act!(mgr:arrow, cx, ArrowOpt::from(-1isize)),
			"up" => act!(insert, cx.input, InsertOpt { append: false }),
			"down" if is_filter => act!(mgr:arrow, cx, ArrowOpt::from(1isize)),
			"down" => succ!(),
			"up5" if is_filter => act!(mgr:arrow, cx, ArrowOpt::from(-5isize)),
			"up5" => succ!(),
			"down5" if is_filter => act!(mgr:arrow, cx, ArrowOpt::from(5isize)),
			"down5" => succ!(),

			// Everything below is filter-only (no-op otherwise)
			_ if !is_filter => succ!(),
			"quit" => act!(app:quit, cx),
			"yank" => act!(mgr:yank, cx, YankOpt { cut: false }),
			"cut" => act!(mgr:yank, cx, YankOpt { cut: true }),
			"toggle" => act!(mgr:toggle, cx, ToggleOpt { url: None, state: None }),
			"create" => act!(mgr:create, cx, CreateOpt { dir: false, force: false }),
			"unselect" => act!(mgr:escape_select, cx),
			"paste" => act!(mgr:paste, cx, PasteOpt { force: false, follow: false }),
			"rename" => act!(mgr:rename, cx, RenameOpt {
				hovered: false,
				force: false,
				empty: Default::default(),
				cursor: Default::default(),
			}),
			"copy_path" => {
				let result = act!(mgr:copy, cx, CopyOpt {
					r#type: "path".into(),
					separator: Default::default(),
					hovered: false,
				});
				NotifyProxy::push(PushOpt {
					title:   "Copy".to_owned(),
					content: "Path copied to clipboard".to_owned(),
					level:   PushLevel::Info,
					timeout: std::time::Duration::from_secs(3),
				});
				result
			}
			_ => succ!(),
		}
	}
}
