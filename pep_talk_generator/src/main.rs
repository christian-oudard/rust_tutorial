// https://old.reddit.com/r/coolguides/comments/qacund/handy_pep_talk_guide/

use rand::{Rng, thread_rng};

const CLAUSES: &'static [&'static [&'static str]] = &[
    &[
        "Champ, ",
        "Fact: ",
        "Everybody says ",
        "Dang... ",
        "Check it: ",
        "Just saying... ",
        "Superstar, ",
        "Tiger, ",
        "Self, ",
        "Know this: ",
        "News alert: ",
        "Girl, ",
        "Ace, ",
        "Excuse me but ",
        "Experts agree: ",
        "In my opinion, ",
        "Hear ye, hear ye: ",
        "Okay, listen up: ",
    ],
    &[
        "the mere idea of you ",
        "your soul ",
        "your hair today ",
        "everything you do ",
        "your personal style ",
        "every thought you have ",
        "that sparkle in your eye ",
        "your presence here ",
        "what you got going on ",
        "the essential you ",
        "your life's journey ",
        "that saucy personality ",
        "your DNA ",
        "that brain of yours ",
        "your choice of attire ",
        "the way you roll ",
        "whatever your secret is ",
        "all of y'all ",
    ],
    &[
        "has serious game, ",
        "rains magic, ",
        "deserves the Nobel prize, ",
        "raises the roof, ",
        "breeds miracles, ",
        "is paying off big time, ",
        "shows mad skills, ",
        "just shimmer, ",
        "is a national treasure, ",
        "gets the party hopping, ",
        "is the next big thing, ",
        "roars like a lion, ",
        "is a rainbow factory, ",
        "is made of diamonds, ",
        "makes birds sing, ",
        "should be taught in school, ",
        "makes my world go 'round, ",
        "is 100% legit, ",
    ],
    &[
        "24/7.",
        "can I get an amen?",
        "and that's a fact.",
        "so treat yourself.",
        "you feel me?",
        "that's just science.",
        "would I lie?",
        "for reals.",
        "mic drop.",
        "you hidden gem.",
        "snuggle bear.",
        "period.",
        "now let's dance.",
        "high five.",
        "say it again!",
        "according to CNN.",
        "so get used to it.",
    ],
];

fn main() {
    let mut rng = thread_rng();

    for n in 0..CLAUSES.len() {
        let clause_choices = CLAUSES[n];
        let i = rng.gen_range(0..clause_choices.len());
        let clause = clause_choices[i];
        print!("{}", clause);
    }
    println!("");
}
