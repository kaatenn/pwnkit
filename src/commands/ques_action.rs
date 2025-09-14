use crate::data::question::Question;
use crate::error::PkError;
use clap::Subcommand;
use rusqlite::ToSql;

#[derive(Subcommand)]
pub enum QuesAction {
    List {
        #[arg(short, long)]
        competition: Option<String>,
        #[arg(short, long)]
        tags: Option<Vec<String>>,
    },
    Add {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        competition: String,
        #[arg(short, long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },
    Remove {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        competition: String,
    },
}

impl QuesAction {
    pub fn execute(&self) -> Result<(), PkError> {
        match self {
            QuesAction::List { competition, tags } => {
                Self::list_questions(competition, tags)?;
                Ok(())
            }
            QuesAction::Add { name, competition, tags } => {
                Self::add_question(name, competition, tags)?;
                println!("Add a new competition");
                Ok(())
            },
            QuesAction::Remove { name, competition } => {
                Self::remove_question(name, competition)?;
                println!("Remove a question");
                Ok(())
            }
        }
    }

    fn list_questions(
        competition: &Option<String>,
        tags: &Option<Vec<String>>,
    ) -> Result<(), PkError> {
        let mut sql = "SELECT name, competition, tags FROM questions WHERE 1=1 ".to_string();
        let mut params: Vec<String> = Vec::new();

        if let Some(competition) = competition {
            sql.push_str(" AND competition = ? ");
            params.push(competition.clone());
        }

        if let Some(tags) = tags {
            for (i, tag) in tags.iter().enumerate() {
                if i == 0 {
                    sql.push_str(" AND (tag LIKE ?");
                } else {
                    sql.push_str(" OR tag LIKE ? ");
                }
                params.push(format!("%{}%", tag));
            }
            if !tags.is_empty() {
                sql.push(')');
            }
        }

        let conn = crate::database::Database::init_db()?;
        let mut stmt = conn.prepare(&sql)?;
        let param_refs: Vec<&dyn ToSql> = params.iter().map(|i| i as &dyn ToSql).collect();
        let rows = stmt.query_map(&param_refs[..], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?;
        println!("Questions:");
        for row in rows {
            let (name, comp, tags) = row?;
            println!("- {} ({}) [{}]", name, comp, tags);
        }
        Ok(())
    }

    fn add_question(name: &String, competition: &String, tags: &Option<Vec<String>>) -> Result<(), PkError> {
        let question = Question::new(name.clone(), competition.clone(), tags.clone());
        question.add_question()?;
        Ok(())
    }

    fn remove_question(name: &String, competition: &String) -> Result<(), PkError> {
        let question = Question::new(name.clone(), competition.clone(), None);
        question.remove_ques()?;

        Ok(())
    }
}
