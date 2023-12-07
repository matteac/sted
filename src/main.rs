mod buffer;
mod program;
mod repl;
pub const HELP: &str = "\n\
Commands:\n\n\
\thelp|h|?\t\t\tShow this help message\n\
\topen|o\t\t<filepath>\tOpen a new buffer\n\
\tfocus|f\t\t<id|filepath>\tFocus the given buffer\n\
\tclose|c\t\t<id|filepath>\tClose the given buffer\n\
\tinsert|i\t<line>\t\tInsert text at the given line\n\
\tprint|p\t\t\t\tPrint the current buffer\n\
\tsave|s\t\t\t\tSave the current buffer\n\
\tsave-all|sa\t\t\tSave all buffers\n\
\tlist|l\t\t\t\tList all buffers\n\
\tlist-dir|ld\t<path>\t\tList all files in the given directory\n\
\tclear|cls\t\t\tClear the screen\n\
\texit|q\t\t\t\tExit the program\n";

fn main() {
    let mut p = program::Program::new();
    repl::start(&mut p);
}
