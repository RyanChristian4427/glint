use crate::cli;
use crossterm as ct;
use glint::string;
use glint::{Config, Git};
use std::io::Write as _Write;
use std::{io, iter};

pub fn log(params: cli::Log, _config: Config) {
    let git = match Git::from_cwd() {
        Ok(git) => git,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };
    let size = ct::terminal().terminal_size();
    let width = std::cmp::max(size.0, 60) as usize;
    let height = params
        .num
        .unwrap_or_else(|| std::cmp::max(size.1, 15) as usize);
    let count_arg = format!("-{}", height);
    let args = iter::once(&count_arg).chain(params.git_args.iter());
    let logs = git.log_parsed(args).expect("parse logs");

    let stdout = &mut io::stdout();
    for log in logs {
        let conv = log.as_conventional();

        let scope = conv.clone().and_then(|c| c.scope.map(String::from));
        let ty = match conv {
            Some(ref conv) => conv.ty.to_string(),
            None => "unknown".to_string(),
        };

        let message = match conv {
            Some(ref conv) => conv.message,
            None => &log.message,
        };

        let message = message
            .split("\n")
            .filter(|s| s.chars().any(|c| !c.is_whitespace()))
            .collect::<Vec<&str>>()
            .join(" ⏎");
        let message = string::split_at(&message, width).0.to_string();

        if params.debug {
            println!("----------\nItem: {:#?}\nas_conventional: {:#?}", log, conv);
        } else {
            ct::queue!(
                stdout,
                ct::SetFg(ct::Color::Yellow),
                ct::Output(log.commit[..8].into()),
                ct::Output(" ".into()),
                ct::SetFg(ct::Color::Magenta),
                ct::Output(ty),
                ct::SetFg(ct::Color::Grey),
                ct::Output(match scope {
                    Some(_) => "(".into(),
                    None => "".into(),
                }),
                ct::SetFg(ct::Color::Blue),
                ct::Output(match scope {
                    Some(ref scope) => scope.into(),
                    None => "".into(),
                }),
                ct::SetFg(ct::Color::Grey),
                ct::Output(match scope {
                    Some(_) => "): ".into(),
                    None => ": ".into(),
                }),
                ct::SetFg(ct::Color::Reset),
                ct::Output(message),
                ct::Output("\n".into())
            )
            .unwrap();
        }
    }
    stdout.flush().unwrap();
}
