use std::ptr::null;

const MAX_VALUE : u32 = 10000000;

const OP_PLUS : char =  '+';
const OP_MINUS : char =  '-';
const OP_INTERSECT : char =  '*';
const OP_DIVIDE : char =  '/';

#[derive(Clone)]
#[derive(Debug)]
struct Operands {
    symbol: char,
    in_use: bool,
    answer: usize,
}

struct Datum {
    name: String,
    types: Vec<Operands>,
    left : u32,
    right : u32,
    result : u32,
}

pub fn tmp() -> String {
   "hello".to_string()
}
fn calc(param: &mut Datum)  {

    let mut new_types:Vec<Operands> = vec![];

    for (_k , v) in &mut param.types.iter().enumerate() {
        if  v.in_use == true { 
            let new_opr = match v.symbol  {
                OP_PLUS =>
                    Operands {
                        symbol: v.symbol,
                        in_use: v.in_use,
                        answer: add(
                            param.left.try_into().unwrap() , 
                            param.right.try_into().unwrap()
                        ),
                    },
                OP_MINUS => 
                    Operands {
                        symbol: v.symbol,
                        in_use: v.in_use,
                        answer: minus(
                            param.left.try_into().unwrap() , 
                            param.right.try_into().unwrap()
                        ),
                    },
                OP_INTERSECT => 
                    Operands {
                        symbol: v.symbol,
                        in_use: v.in_use,
                        answer: intersect(
                            param.left.try_into().unwrap() , 
                            param.right.try_into().unwrap()
                        ),
                    },
                OP_DIVIDE => 
                    Operands {
                        symbol: v.symbol,
                        in_use: v.in_use,
                        answer: divide(
                            param.left.try_into().unwrap() , 
                            param.right.try_into().unwrap()
                        ),
                    },
                _ => 
                    Operands {
                        symbol: '無',
                        in_use: v.in_use,
                        answer: 0,
                },
            };

            new_types.push(new_opr)
        }
    
    }
    chg(param, &mut new_types);

    println!("result: {} MAX_VALUE: {} ", &param.result, MAX_VALUE);
    // return param
}

fn chg(strc : &mut Datum, new_operand : &mut Vec<Operands>) {
    strc.types = new_operand.to_vec();
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn minus(left: usize, right: usize) -> usize {
    left - right
}
pub fn intersect(left: usize, right: usize) -> usize {
    left * right
}
pub fn divide(left: usize, right: usize) -> usize {
    left / right
}

pub fn eew() -> () {
    let vector = vec!["a", "b", "c", "d"]; 
    for (k, t) in vector.iter().enumerate() {
        println!("result k:{}  v: {}", k, t);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn minus_is_works() {
        let result : usize = minus(10, 7);
        assert_eq!(result, 3);
    }

    #[test]
    fn mix() {

        // 演算の設定
        let ope: Vec<Operands> = vec![
            Operands {
                symbol: OP_PLUS,
                in_use: true,
                answer: 0,
            },
            Operands {
                symbol: OP_MINUS,
                in_use: true,
                answer: 0,
            },
            Operands {
                symbol: OP_INTERSECT,
                in_use: true,
                answer: 0,
            },
            Operands {
                symbol: OP_DIVIDE,
                in_use: true,
                answer: 0,
            },
        ];

        // 演算対象のパタメータ設定
        let mut d: Datum = Datum {
            name: String::from("演算"),
            types: ope,
            left: 19,
            right: 12,
            result: 0,
        };

        // 演算
        calc(&mut d) ;
        println!("ret:{}", d.result);
        assert_eq!(d.result, 0);
        for (k, v) in d.types.iter().enumerate() {
            println!("No : {} target:{} =>  {}", k, v.symbol, v.answer);

            if v.symbol == '+' {
                assert_eq!(31, v.answer);
            } else if v.symbol == '-' {
                assert_eq!(7, v.answer);
            } else if v.symbol == '*' {
                assert_eq!(228, v.answer);
            } else if v.symbol == '/' {
                assert_eq!(1, v.answer);
            } else {
                assert_eq!("error!", "not defined!");
            }
        }
        // println!("test: {}", *const pt.answer.);
        // assert_eq!(pt.answer.to_owned(), 31);

    }

    #[test]
    fn tmptest() {
        let ret = tmp();
        assert_eq!(ret, "hello".to_string());        
    }

}
