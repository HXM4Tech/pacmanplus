use crate::printtr;
use crate::util::executable_name;

pub fn help() {
    let exe = executable_name();
    printtr!("usage:  {exe} <operation> [...]", exe = exe);
    println!();
    printtr!("operations:");
    printtr!("    {exe} {{-h --help}}", exe = exe);
    printtr!("    {exe} {{-V --version}}", exe = exe);
    printtr!("    {exe} {{-D --database}}    <options> <package(s)>", exe = exe);
    printtr!("    {exe} {{-F --files}}       [options] [package(s)]", exe = exe);
    printtr!("    {exe} {{-Q --query}}       [options] [package(s)]", exe = exe);
    printtr!("    {exe} {{-R --remove}}      [options] <package(s)>", exe = exe);
    printtr!("    {exe} {{-S --sync}}        [options] [package(s)]", exe = exe);
    printtr!("    {exe} {{-T --deptest}}     [options] [package(s)]", exe = exe);
    printtr!("    {exe} {{-U --upgrade}}     [options] [file(s)]", exe = exe);
    println!();
    printtr!("use '{exe} {{-h --help}}' with an operation for available options", exe = exe);
}
