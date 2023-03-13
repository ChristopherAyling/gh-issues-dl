use octocrab::{models, params};
use std::{thread, time, env, vec::Vec};

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    println!("Hello, world!");

    let token = env::var("GH_PAT").unwrap();

    let octocrab = octocrab::Octocrab::builder()
        .personal_token(token)
        .build()?;
    // Returns the first page of all issues.
    let mut page = octocrab
        .issues("ChristopherAyling", "self.step")
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
            print!("{} - {}", issue.number, issue.title);
            issue.labels.iter().for_each(|label| {
                print!(" #{}", label.name);
            });
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
    let mut file = std::fs::File::create("issues.json").unwrap();
    serde_json::to_writer_pretty(&mut file, &all_issues).unwrap();

    Ok(())
}
