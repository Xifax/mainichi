use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand)]
pub enum Action {
    /// Display kanji for today
    Roll {
        // TODO: !!!
        // // WIP: possible options
        // max_rarity: usize,
        // // Don't roll new kanji that is much rarer that previous one (sic!)
        // sort_by_rarity: bool,
        /// Force to fetch new kanji even if there's already one for today
        #[clap(short, long, default_value_t = false)]
        force: bool,
    },
    // Definition and so on
    Gloss {
        /// Display kana as colored, leave kanji white
        #[clap(short, long, default_value_t = false)]
        colorize_kana: bool,

        /// Colorize everything
        #[clap(short, long, default_value_t = false)]
        all_color: bool,
    },
    // Related words
    Words {
        /// Display kana as colored, leave kanji white
        #[clap(short, long, default_value_t = false)]
        colorize_kana: bool,

        /// Colorize everything
        #[clap(short, long, default_value_t = false)]
        all_color: bool,
    },
    /// Display example for today's kanji
    Examples {
        /// Number of examples to fetch
        #[clap(short, long, default_value_t = 5)]
        count: usize,

        /// Highlight ALL hiragana
        #[clap(long, default_value_t = false)]
        highlight_kana: bool,

        /// Get random COUNT examples from all those fetched from Massif
        #[clap(short, long, default_value_t = false)]
        randomize: bool,

        /// Try to get examples for related words
        #[clap(short, long, default_value_t = false)]
        words_related: bool,

        /// Item to lookup instead of today's kanji
        #[clap(short, long)]
        query: Option<String>,
    },
    /// Show kanji history
    History {},
    /// Similar kanji and words
    Related {},
    Lookup {
        // Item to lookup
        // #[clap(short, long)]
        // query: String,

        // Lookup kanji|words|examples for provided query in local resources
        // TODO: show gloss|exampes for specific kanji|words
    },
    /// Test functionality
    Test,
}
