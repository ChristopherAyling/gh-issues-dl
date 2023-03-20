use clap::Parser;
use octocrab::{models, params};
use std::{thread, time};

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    repo: String,
    #[clap(short, long)]
    token: Option<String>,
    #[clap(short, long, default_value = "stdio")]
    output: String,
    // quiet is false by default
    #[clap(short, long, default_value = "false")]
    quiet: bool,
}

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let args = Args::parse();
    let token = args.token;

    let octocrab = match  token {
        Some(token) => octocrab::Octocrab::builder()
            .personal_token(token)
            .build()?,
        None => octocrab::Octocrab::builder()
            .build()?,
    };

    // split repo into owner and repo destructure
    let (owner, repo) = match args.repo.split_once("/") {
        Some((owner, repo)) => (owner, repo),
        None => {
            eprintln!("Invalid repo name. Expected OWNER/REPONAME. Got {}", args.repo);
            std::process::exit(1);
        }
    };


    // Returns the first page of all issues.
    let mut page = octocrab
        .issues(owner, repo)
        .list()
        // Optional Parameters
        .state(params::State::All)
        .per_page(50)
        .send()
        .await?;
    
    // Go through every page of issues. Warning: There's no rate limiting so
    // be careful.
    let mut all_issues = Vec::new();

    loop {
        for issue in &page {
            all_issues.push(issue.clone());
            if !args.quiet && args.output != "stdio" {
                print!("{} - {}", issue.number, issue.title);
                issue.labels.iter().for_each(|label| {
                    print!(" #{}", label.name);
                });
            }
            println!();
        }
        page = match octocrab
            .get_page::<models::issues::Issue>(&page.next)
            .await?
        {
            Some(next_page) => next_page,
            None => break,
        };
        thread::sleep(time::Duration::from_secs(2));
    }

    // write to file
    if args.output != "stdio" {
        let mut file = std::fs::File::create(args.output).unwrap();
        serde_json::to_writer_pretty(&mut file, &all_issues).unwrap();
    } else {
        let mut file = std::io::stdout();
        serde_json::to_writer_pretty(&mut file, &all_issues).unwrap();
    };

    Ok(())
}
