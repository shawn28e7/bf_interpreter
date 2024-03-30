use std::{error::Error, fs};

pub fn strip_args(mut args: impl Iterator<Item = String>) -> Result<String, &'static str>
{
    args.next();

    let file_path = match args.next()
    {
        Some(args) => args,
        None => return Err("unable to get file"),
    };

    return Ok(file_path);
}

pub fn read_code(file_path: &String) -> Result<String, Box<dyn Error>>
{
    let contents: String = fs::read_to_string(file_path)?.trim().parse()?;
    return Ok(contents);
}

#[derive(PartialEq, Debug)]
pub enum Status
{
    MoveLeft,
    MoveRight,
    LoopLeft,
    LoopRight,
    Add,
    Sub,
    Output,
}
pub fn check_n_format_code(code: &String) -> Result<Vec<Status>, &'static str>
{
    let vaild_code = String::from("<>[]+-.# \n");
    let code = code.lines();
    let mut results: Vec<Status> = Vec::new();
    for lines in code
    {
        for i in lines.chars()
        {
            if !vaild_code.contains(i)
            {
                return Err("invaild code");
            }
            else
            {
                results.push(match i
                {
                    '<' => Status::MoveLeft,
                    '>' => Status::MoveRight,
                    '[' => Status::LoopLeft,
                    ']' => Status::LoopRight,
                    '+' => Status::Add,
                    '-' => Status::Sub,
                    '.' => Status::Output,
                    '#' => break,
                    ' ' | '\n' => continue,
                    _ => return Err("unexpected error"),
                });
            }
        }
    }
    Ok(results)
}

pub fn run(code: &Vec<Status>) -> Result<(), &'static str>
{
    let mut arr: Vec<u8> = vec![0];
    let mut ptr: usize = 0;
    let mut i: usize = 0;
    while i < code.len()
    {
        match &code[i]
        {
            Status::MoveRight =>
            {
                if ptr + 1 < arr.len()
                {
                    ptr += 1
                }
                else
                {
                    arr.push(0);
                    ptr += 1;
                }
            }
            Status::MoveLeft =>
            {
                if ptr != 0
                {
                    ptr -= 1
                }
                else
                {
                    return Err("out of bound");
                }
            }
            Status::LoopLeft =>
            {
                if arr[ptr] == 0
                {
                    while code[i] != Status::LoopRight
                    {
                        i += 1;
                        if i > code.len()
                        {
                            return Err("no matched ]");
                        }
                    }
                }
            }
            Status::LoopRight =>
            {
                if arr[ptr] != 0
                {
                    while code[i] != Status::LoopLeft
                    {
                        if i <= 0
                        {
                            return Err("no matched [");
                        }
                        i -= 1;
                    }
                }
            }
            Status::Add => arr[ptr] += 1,
            Status::Sub => arr[ptr] -= 1,
            Status::Output => print!("{}", arr[ptr] as char),
        }
        i += 1;
    }
    Ok(())
}
