use crate::{ data::{ DirectoryRef, DirectoryEntry, FileEntry }, errors::{ Error, ParsingError } };
use lazy_static::lazy_static;
use regex::Regex;

struct ParsingContext {
    current_directory: Option<DirectoryRef>,
    directory_stack: Vec<DirectoryRef>
}

impl ParsingContext {
    fn new() -> ParsingContext {
        ParsingContext { current_directory: None, directory_stack: Vec::<_>::new() }
    }
}

trait LogParserRule {
    fn matches(&self, line: &str) -> bool;
    fn apply_to(&self, context: ParsingContext, line: &str) -> Result<ParsingContext, Error>;
}

struct CdIntoRule { regex: Regex }
struct CdUpRule { regex: Regex }
struct LsRule { regex: Regex }
struct DirEntryRule { regex: Regex }
struct FileEntryRule { regex: Regex }

impl CdIntoRule {
    fn new() -> Result<CdIntoRule, Error> {
        lazy_static! {
            static ref CD_INTO: Result<Regex, regex::Error> = Regex::new(r"\$ cd (?P<dirname>(\w|\.|/)+)");
        }

        let regex = CD_INTO.as_ref()?.to_owned();
        Ok(CdIntoRule { regex })
    }
}

impl LogParserRule for CdIntoRule {
    fn matches(&self, line: &str) -> bool {
        self.regex.is_match(line)
    }

    fn apply_to(&self, mut context: ParsingContext, line: &str) -> Result<ParsingContext, Error> {
        let dirname = self.regex
            .captures(line)
            .and_then(|captures| captures.name("dirname"))
            .map(|dirname| dirname.as_str().to_string())
            .ok_or(ParsingError::InvalidLine(line.to_string()))?;

        let into_directory = match &context.current_directory {
            Some(directory) => {
                directory.borrow_mut()
                    .directories
                    .entry(dirname)
                    .or_insert(DirectoryEntry::new_ref())
                    .clone()
            },
            None => DirectoryEntry::new_ref()
        };

        if let Some(current_directory) = context.current_directory {
            context.directory_stack.push(current_directory);
        }

        context.current_directory = Some(into_directory);
        Ok(context)
    }
}

impl CdUpRule {
    fn new() -> Result<CdUpRule, Error> {
        lazy_static! {
            static ref CD_UP: Result<Regex, regex::Error> = Regex::new(r"\$ cd \.\.");
        }

        let regex = CD_UP.as_ref()?.to_owned();
        Ok(CdUpRule { regex })
    }
}

impl LogParserRule for CdUpRule {
    fn matches(&self, line: &str) -> bool {
        self.regex.is_match(line)
    }

    fn apply_to(&self, mut context: ParsingContext, _: &str) -> Result<ParsingContext, Error> {
        let up_directory = context.directory_stack.pop().ok_or(ParsingError::NoParentDirectory)?;
        context.current_directory = Some(up_directory);
        Ok(context)
    }
}

impl LsRule {
    fn new() -> Result<LsRule, Error> {
        lazy_static! {
            static ref LS: Result<Regex, regex::Error> = Regex::new(r"\$ ls");
        }

        let regex = LS.as_ref()?.to_owned();
        Ok(LsRule { regex })
    }
}

impl LogParserRule for LsRule {
    fn matches(&self, line: &str) -> bool {
        self.regex.is_match(line)
    }

    fn apply_to(&self, context: ParsingContext, _: &str) -> Result<ParsingContext, Error> {
        Ok(context)
    }
}

impl DirEntryRule {
    fn new() -> Result<DirEntryRule, Error> {
        lazy_static! {
            static ref DIR_ENTRY: Result<Regex, regex::Error> = Regex::new(r"dir (?P<dirname>(\w|\.)+)");
        }

        let regex = DIR_ENTRY.as_ref()?.to_owned();
        Ok(DirEntryRule { regex })
    }
}

impl LogParserRule for DirEntryRule {
    fn matches(&self, line: &str) -> bool {
        self.regex.is_match(line)
    }

    fn apply_to(&self, context: ParsingContext, line: &str) -> Result<ParsingContext, Error> {
        let dirname = self.regex
            .captures(&line)
            .and_then(|captures| captures.name("dirname"))
            .map(|dirname| dirname.as_str().to_string())
            .ok_or(ParsingError::InvalidLine(line.to_string()))?;

        context.current_directory.as_ref().ok_or(ParsingError::NoCurrentDirectory)?
            .borrow_mut()
            .directories
            .entry(dirname)
            .or_insert(DirectoryEntry::new_ref());

        Ok(context)
    }
}

impl FileEntryRule {
    fn new() -> Result<FileEntryRule, Error> {
        lazy_static! {
            static ref FILE_ENTRY: Result<Regex, regex::Error> = Regex::new(r"(?P<filesize>\d+) (?P<filename>(\w|\.)+)");
        }

        let regex = FILE_ENTRY.as_ref()?.to_owned();
        Ok(FileEntryRule { regex })
    }
}

impl LogParserRule for FileEntryRule {
    fn matches(&self, line: &str) -> bool {
        self.regex.is_match(line)
    }

    fn apply_to(&self, context: ParsingContext, line: &str) -> Result<ParsingContext, Error> {
        let (filesize, filename) = self.regex
            .captures(&line)
            .and_then(|captures| match (captures.name("filesize"), captures.name("filename")) {
                (Some(filesize), Some(filename)) => Some((filesize.as_str().to_string(), filename.as_str().to_string())),
                _ => None
            })
            .ok_or(ParsingError::InvalidLine(line.to_string()))?;
    
        let filesize = filesize.as_str().parse::<usize>().map_err(|_| Error::ParsingError(ParsingError::InvalidFileSize))?;
        context.current_directory.as_ref().ok_or(Error::ParsingError(ParsingError::NoCurrentDirectory))?
            .borrow_mut()
            .files
            .entry(filename)
            .or_insert(FileEntry::new_ref(filesize));

        Ok(context)
    }
}

pub struct LogParser {
    rules: Vec<Box<dyn LogParserRule>>
}

impl LogParser {
    pub fn default() -> Result<LogParser, Error> {
        Ok(LogParser { rules: vec![
            Box::new(CdUpRule::new()?),
            Box::new(CdIntoRule::new()?),
            Box::new(LsRule::new()?),
            Box::new(DirEntryRule::new()?),
            Box::new(FileEntryRule::new()?)
        ] })
    }

    pub fn parse_log_lines<Iter, IterError>(&self, lines: Iter) -> Result<DirectoryRef, Error>
    where Iter: Iterator<Item = Result<String, IterError>>
        , Error: From<IterError>
    {
        let mut context = ParsingContext::new();
        
        for line_result in lines {
            let line = line_result?;
            let matching_rule = self.rules.iter()
                .filter(|rule| rule.matches(&line))
                .next()
                .ok_or(ParsingError::UnrecognizedSyntax(line.to_string()))?;

            context = matching_rule.apply_to(context, &line)?;
        }

        context.directory_stack
            .first()
            .map(|first_directory_ref| first_directory_ref.clone())
            .or(context.current_directory)
            .ok_or(Error::ParsingError(ParsingError::NoRootDirectory))
    }
}