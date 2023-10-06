use fluffer::{async_trait, App, Context, Fluff, GemBytes};
use rand::Rng;

enum RollError {
    BadRoll,
    InvalidSides(i32),
    InvalidRolls(i32),
}

#[async_trait]
impl GemBytes for RollError {
    async fn gem_bytes(self) -> Vec<u8> {
        match self {
            RollError::BadRoll => format!("10 Bad roll. Try again.\r\n").into_bytes(),
            RollError::InvalidRolls(r) => {
                format!("10 Invalid number of rolls: {r}. Try again.\r\n").into_bytes()
            }
            RollError::InvalidSides(s) => {
                format!("10 Invalid sides: {s}. Try again.\r\n").into_bytes()
            }
        }
    }
}

struct Roll {
    roll_count: i32,
    sides:      i32,
    bonus:      i32,
    total:      i32,
    rolls:      Vec<i32>,
}

impl Roll {
    fn new(roll_count: i32, sides: i32, bonus: i32) -> Self {
        let mut rng = rand::thread_rng();
        let mut rolls: Vec<i32> = Vec::new();
        let mut total: i32 = 0;
        for _ in 1..=roll_count {
            let r = rng.gen_range(1..=sides);
            rolls.push(r);
            total += r;
        }
        total += bonus;

        Self {
            roll_count,
            sides,
            bonus,
            total,
            rolls,
        }
    }
}

impl std::fmt::Display for Roll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "╭─🎲 {}d{} 🎲\n", self.roll_count, self.sides)?;

        if self.rolls.len() > 1 || self.bonus != 0 {
            write!(f, "│Rolls: {:?}\n", self.rolls)?;
        }

        if self.bonus != 0 {
            write!(
                f,
                "│{}{}\n",
                self.bonus.is_positive().then(|| "+").unwrap_or(""),
                self.bonus
            )?;
        }
        write!(f, "╰─Total: {}", self.total)?;

        Ok(())
    }
}

async fn roll(ctx: Context) -> Result<Fluff, RollError> {
    // Prompt for input
    let Some(input) = ctx.input() else {
        return Ok(Fluff::Input(
            r#"Example Usage:
 1d20 + 1
 1d8 - 1
 d6"#
            .to_string(),
        ));
    };

    // Get bonus, and strip it from input
    let (input, bonus) = {
        if let Some((input, bonus)) = input.rsplit_once('+') {
            (input.to_string(), bonus.parse::<i32>().unwrap_or(0))
        } else if let Some((input, bonus)) = input.rsplit_once('-') {
            (input.to_string(), -bonus.parse::<i32>().unwrap_or(0))
        } else {
            (input, 0)
        }
    };

    // Split roll inputession (e.g 1d4)
    let Some((roll_count, sides)) = input.rsplit_once('d') else {
        return Err(RollError::BadRoll);
    };

    // Parse both parts of the split into i32
    let roll_count = roll_count.parse::<i32>().unwrap_or(1);
    let sides = sides.parse::<i32>().map_err(|_| RollError::BadRoll)?;

    if sides < 2 {
        return Err(RollError::InvalidSides(sides));
    }

    if roll_count < 1 {
        return Err(RollError::InvalidRolls(roll_count));
    }

    let roll = Roll::new(roll_count, sides, bonus);

    Ok(Fluff::Text(format!(
        "=> /roll Roll again\n\n```\n{roll}\n```"
    )))
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    App::default()
        .route("/", |_| async { "# 🎲 Dice\n\n=> /roll Roll!" })
        .route("/roll", roll)
        .run()
        .await
        .unwrap();
}
