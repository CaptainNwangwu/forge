use console::Style;

#[derive(Debug)]
pub struct Icons {
    pub success: char,
    pub warning: char,
    pub error: char,
    pub info: char,
    pub forge: char,
}

#[derive(Debug)]
pub struct Colors {
    pub success: Style,
    pub warning: Style,
    pub error: Style,
    pub info: Style,
    pub heading: Style,
    pub dim: Style,
}

#[derive(Debug)]
pub struct Theme {
    pub icons: Icons,
    pub colors: Colors,
}

impl Theme {
    pub fn new() -> Self {
        Theme {
            colors: Colors {
                success: Style::new().green().bold(),

                warning: Style::new().yellow().bold(),

                error: Style::new().red().bold(),

                info: Style::new().cyan(),

                heading: Style::new().bold(),
                dim: Style::new().dim(),
            },
            icons: Icons {
                success: '\u{2714}', // Symbol: ✔
                warning: '\u{26A0}', // Symbol: ⚠
                error: '\u{2717}',   // Symbol: ✗
                info: '\u{2139}',    // Symbol: ℹ
                forge: '\u{1F525}',  // Symbol: 🔥
            },
        }
    }
}
