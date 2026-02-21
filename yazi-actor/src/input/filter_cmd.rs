use anyhow::Result;
use yazi_config::popup::InputKind;
use yazi_macro::{act, succ};
use yazi_parser::input::FilterCmdOpt;
use yazi_parser::mgr::{CopyOpt, CreateOpt, PasteOpt, RenameOpt, ToggleOpt, YankOpt};
use yazi_shared::data::Data;

use crate::{Actor, Ctx};

pub struct FilterCmd;

impl Actor for FilterCmd {
	type Options = FilterCmdOpt;

	const NAME: &str = "filter_cmd";

	fn act(cx: &mut Ctx, opt: Self::Options) -> Result<Data> {
		if cx.input.kind != InputKind::Filter {
			succ!();
		}

		match &*opt.cmd {
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
			"copy_path" => act!(mgr:copy, cx, CopyOpt {
				r#type: "path".into(),
				separator: Default::default(),
				hovered: false,
			}),
			_ => succ!(),
		}
	}
}
