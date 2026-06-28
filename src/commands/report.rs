use crate::cli::{ReportFormatArg, ReportPeriodArg};
use crate::error::Result;
use crate::report::{self, ReportPeriod};
use crate::storage::{TaskStore, resolve_storage};
use crate::utils::{is_color_enabled, with_color};

pub fn run(period: ReportPeriodArg, format: ReportFormatArg, global: bool) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let ctx = resolve_storage(global, &cwd)?;
    let store = TaskStore::open(&ctx)?;

    let period = match period {
        ReportPeriodArg::Today => ReportPeriod::Today,
        ReportPeriodArg::Week => ReportPeriod::Week,
        ReportPeriodArg::Month => ReportPeriod::Month,
        ReportPeriodArg::All => ReportPeriod::All,
    };

    let shipped = report::shipped_tasks(store.tasks(), period);
    let stats = report::compute_stats(&shipped);

    let output = match format {
        ReportFormatArg::Text => {
            let color = with_color(is_color_enabled());
            report::render_text(&shipped, &stats, period, color)
        }
        ReportFormatArg::Md => report::render_markdown(&shipped, &stats, period),
    };

    print!("{output}");
    Ok(())
}
