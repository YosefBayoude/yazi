use anyhow::Result;
use yazi_config::popup::InputKind;
use yazi_macro::{act, succ};
use yazi_parser::ArrowOpt;
use yazi_parser::input::FilterCmdOpt;
use yazi_parser::mgr::{CopyOpt, CreateOpt, PasteOpt, RenameOpt, ToggleOpt, YankOpt};
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
			"quit" => act!(mgr:quit, cx),
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
			"copy_path" => act!(mgr:copy, cx, CopyOpt {
				r#type: "path".into(),
				separator: Default::default(),
				hovered: false,
			}),
			"copy_dirname" => act!(mgr:copy, cx, CopyOpt {
				r#type: "dirname".into(),
				separator: Default::default(),
				hovered: false,
			}),
			"copy_filename" => act!(mgr:copy, cx, CopyOpt {
				r#type: "filename".into(),
				separator: Default::default(),
				hovered: false,
			}),
			"copy_name" => act!(mgr:copy, cx, CopyOpt {
				r#type: "name_without_ext".into(),
				separator: Default::default(),
				hovered: false,
			}),
			_ => succ!(),
		}
	}
}
