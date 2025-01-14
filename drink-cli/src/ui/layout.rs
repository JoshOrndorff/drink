use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Margin},
    Frame,
};

use crate::{
    app_state::AppState,
    ui::{contracts, current_env, footer, help, output, user_input},
};

pub(super) fn layout<B: Backend>(f: &mut Frame<B>, app_state: &mut AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(4, 20),
                Constraint::Ratio(12, 20),
                Constraint::Ratio(2, 20),
                Constraint::Ratio(2, 20),
            ]
            .as_ref(),
        )
        .split(f.size());

    if !app_state.ui_state.output_scrolling {
        app_state.ui_state.output_offset =
            (app_state.ui_state.output.len() as u16).saturating_sub(chunks[1].height - 2) as i16;
    }

    let subchunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(chunks[0].inner(&Margin {
            horizontal: 0,
            vertical: 0,
        }));
    f.render_widget(current_env::build(app_state), subchunks[0]);
    f.render_widget(contracts::build(app_state), subchunks[1]);

    if app_state.ui_state.show_help {
        f.render_widget(help::build(app_state), chunks[1]);
    } else {
        f.render_widget(output::build(app_state), chunks[1]);
    }

    f.render_widget(user_input::build(app_state), chunks[2]);
    f.render_widget(footer::build(app_state), chunks[3]);
}
