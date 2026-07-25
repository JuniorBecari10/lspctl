enum Segment<'a> {
    Text(&'a str),
    Expr(&'a str), // slice inside the braces
}
