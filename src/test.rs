use crate::Program;


#[test]
fn test_add() {
    let out = |s| {
        assert_eq!(s, "3");
    };

    Program::with_stdout(out).compile_str("a=1\nb=2\na+=b\n<a").run();
}

#[test]
fn test_sub() {
    let out = |s| {
        assert_eq!(s, "1");
    };

    Program::with_stdout(out).compile_str("a=3\nb=2\na-=b\n<a").run();
}

#[test]
fn test_mul() {
    let out = |s| {
        assert_eq!(s, "6");
    };

    Program::with_stdout(out).compile_str("a=3\nb=2\na*=b\n<a").run();
}

#[test]
fn test_div() {
    let out = |s| {
        assert_eq!(s, "3");
    };

    Program::with_stdout(out).compile_str("a=6\nb=2\na/=b\n<a").run();
}

#[test]
fn test_mod() {
    let out = |s| {
        assert_eq!(s, "1");
    };

    Program::with_stdout(out).compile_str("a=6\nb=5\na%=b\n<a").run();
}

#[test]
fn test_min() {
    let out = |s| {
        assert_eq!(s, "3");
    };

    Program::with_stdout(out).compile_str("a=12\nb=3\na min= b\n<a").run();
}

#[test]
fn test_max() {
    let out = |s| {
        assert_eq!(s, "12");
    };

    Program::with_stdout(out).compile_str("a=12\nb=3\na max= b\n<a").run();
}

#[test]
fn test_invert() {
    let out = |s| {
        assert_eq!(s, "0");
    };

    Program::with_stdout(out).compile_str("a=123\na invert\n<a").run();
}

#[test]
fn test_print() {
    let out = |s| {
        assert_eq!(s, "256 hello world!");
    };

    Program::with_stdout(out).compile_str("\n< 256, hello world!").run();
}

#[test]
fn test_tag() {
    let out = |s| {
        assert_eq!(s, "101");
    };

    Program::with_stdout(out).compile_str("thing=100\njmp my_tag\n<thing\n@my_tag\nthing +=1\n<thing").run();
}

#[test]
fn test_stacked_tag() {
    let mut i = 0;

    let out = move |s| {
        match i {
            0 => assert_eq!(s, "100"),
            1 => assert_eq!(s, "101"),
            2 => assert_eq!(s, "ended"),
            _ => panic!()
        }
        i += 1;
    };

    let mut program = Program::with_stdout(out).compile_str("thing=100\njmp my_tag\n<thing\nreturn\n@@my_tag\n<thing\nthing +=1").run();
    (program.stdout_function)("ended".to_string());
}