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
        // IDEA: Don't roll new kanji that is much rarer that previous one (sic!)
        /// Force to fetch new kanji even if there's already one for today
        #[clap(short, long, default_value_t = false)]
        force: bool,

        /// Max frequency allowed
        #[clap(short, long)]
        max_frequency: Option<usize>,

        /// Display kanji as ascii
        #[clap(short, long, default_value_t = false)]
        ascii_art: bool,

        /// Set this kanji as today's
        #[clap(short, long)]
        set_kanji: Option<String>,
    },
    /// Display definition and thesaurus entry
    Gloss {
        /// Display kana as colored, leave kanji white
        #[clap(short, long, default_value_t = false)]
        colorize_kana: bool,

        /// Colorize everything
        #[clap(short, long, default_value_t = false)]
        all_color: bool,
    },
    /// Display related words
    Words {
        /// Display kana as colored, leave kanji white
        #[clap(short, long, default_value_t = false)]
        colorize_kana: bool,

        /// Colorize everything
        #[clap(short, long, default_value_t = false)]
        all_color: bool,
    },
    /// Fetch examples for today's kanji
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
    /// Show all kanji rolled up until today
    History {},
    /// Similar kanji and words, from Odyssey and other sources
    Related {},
    /// Lookup specific items in different context (separate from examples, words)
    Lookup {
        // IDEA: Lookup kanji|words|examples for provided query in local resources
        // IDEA: show gloss|exampes for specific kanji|words
    },
    /// Test functionality and random POCs
    Test {},
}
