use std::{error::Error, fs};

pub fn strip_args(args: &Vec<String>) -> Result<String, &'static str>
{
    if args.len() < 2
    {
        return Err("not enough arguments");
    }
    return Ok(args[1].clone());
}

pub fn read_code(file_path: &String) -> Result<String, Box<dyn Error>>
{
    let contents: String = fs::read_to_string(file_path)?.trim().parse()?;
    return Ok(contents);
}

pub fn check_n_format_code(code: &String) -> Result<Vec<i32>, &'static str>
{
    let vaild_code = String::from("<>[]+-.");
    let mut results: Vec<i32> = Vec::new();
    for i in code.chars()
    {
        if !vaild_code.contains(i)
        {
            return Err("invaild code");
        }
        else
        {
            results.push(match i
            {
                '>' => 1,
                '<' => 2,
                '[' => 3,
                ']' => 4,
                '+' => 5,
                '-' => 6,
                '.' => 7,
                _ => return Err("?"),
            });
        }
    }
    Ok(results)
}

pub fn run(code: &Vec<i32>) -> Result<(), &'static str>
{
    let mut arr: Vec<u8> = vec![0];
    let mut ptr: usize = 0;
    let mut i: usize = 0;
    while i < code.len()
    {
        match &code[i]
        {
            1 =>
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
            2 =>
            {
                if ptr > 0
                {
                    ptr -= 1
                }
                else
                {
                    return Err("out of bound");
                }
            }
            3 =>
            {
                if arr[ptr] == 0
                {
                    while code[i] != 4
                    {
                        i += 1;
                        if i > code.len()
                        {
                            return Err("no matched ]");
                        }
                    }
                }
            }
            4 =>
            {
                if arr[ptr] != 0
                {
                    while code[i] != 3
                    {
                        if i <= 0
                        {
                            return Err("no matched [");
                        }
                        i -= 1;
                    }
                }
            }
            5 => arr[ptr] += 1,
            6 => arr[ptr] -= 1,
            7 => print!("{}", arr[ptr] as char),
            _ => return Err("WTF"),
        }
        i += 1;
    }
    Ok(())
}
