#[macro_use]
extern crate nom;

use std::fs::File;
use std::env;
use nom::is_digit;
use std::str::from_utf8;
use std::io::Read;

fn main() {
    let filename = env::args().nth(1).expect("filename not specified");
    let mut fd = File::open(filename).expect("unable to open file");
    let mut buf = Vec::new();
    fd.read_to_end(&mut buf).expect("unable to read file");

    let (_, steps) = danceparser(&buf).unwrap();
    println!("{:?}", steps);
}

#[derive(Debug, PartialEq)]
enum Dance {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

named!(danceparser<&[u8], Vec<Dance>>,
       separated_list_complete!(
           tag!(","),
           alt!(
               spinparser |
               exchangeparser |
               partnerparser
           )));

named!(usize_digits<&[u8], usize>,
       map_res!(
           map_res!(
               take_while1!(is_digit),
               from_utf8
           ),
           |x: &str| {x.parse::<usize>()}
       ));

named!(spinparser<&[u8], Dance>,
       map_res!(
           do_parse!(
               tag!("s") >>
                   howmany: usize_digits >>
                   (howmany)
           ),
           |x| {Ok(Dance::Spin(x)) as Result<Dance, ()>}
       ));

named!(exchangeparser<&[u8], Dance>,
       map_res!(
           do_parse!(
               tag!("x") >>
               places: separated_pair!(usize_digits,
                                       tag!("/"),
                                       usize_digits) >>
               (places)
           ),
           |(a, b)| {Ok(Dance::Exchange(a, b)) as Result<Dance, ()>}
       ));

named!(partnerparser<&[u8], Dance>,
       map_res!(
           do_parse!(
               tag!("p") >>
               a: take!(1) >>
               tag!("/") >>    
               b: take!(1) >>
               (a, b)),
           |(a, b): (&[u8], &[u8])| { Ok(Dance::Partner(a[0] as char, b[0] as char)) as Result<Dance, ()>}
       ));

#[cfg(test)]
mod test {
    use super::*;

    static INPUT_DAY16_PT1: &'static [u8] = b"x3/4,pm/e,x15/7,pp/l,x5/8,s15,x2/6,s9,x0/4,s7,x8/12,pc/b,x2/14,pe/d,x10/11,pg/o,x9/14,ph/n,x15/8,s13,pe/i,x3/7,s2,pd/b,x4/15,s11,x0/14,s10,x7/3,s8,x2/1,pj/i,s8,x10/14,s5,x4/1,pa/h,s11,x12/6,s1,x1/14,s11,x5/12,s4,pn/b,x7/9,pj/l,x1/8,pd/e,s6,pp/g,x10/2,pi/o,x15/5,pp/l,x6/13,s3,x5/12,pk/f,x1/8,s4,x11/10,s10,x4/15,pe/n,x1/5,s14,x7/2,pc/i,x4/10,pj/p,x6/15,s12,x5/0,pa/n,x8/12,s14,x3/5,pb/d,x4/12,s3,x9/6,pg/h,x3/1,s5,x4/10,pm/f,x12/2,s11,x15/13,pa/g,x0/2,s13,pj/i,s13,x6/10,pn/a,x2/1,pm/f,x10/12,s4,pi/c,x14/13,s4,x15/0,s7,x14/8,s5,x5/2,pe/d,x3/9,s14,x7/0,s10,x6/14,s8,x3/15,pn/j,x2/1,pf/a,x13/10,pe/m,x8/0,s2,x4/7,pc/l,s4,x2/0,s4,x3/4,s12,x11/2,s3,x5/8,s2,x4/6,pn/f,x15/5,s15,x4/12,s11,x1/13,pm/b,s13,pj/h,s15,x9/3";

    #[test]
    fn parses_a_spin() {
        let input: &[u8] = b"s12";

        let res = spinparser(input);
        assert!(res.is_done());

        let (_, val) = res.unwrap();
        assert_eq!(val, Dance::Spin(12));
    }

    #[test]
    fn only_parses_a_spin() {
        let input = b"x12";

        let res = spinparser(input);
        assert!(res.is_err());
    }

    #[test]
    fn parses_an_exchange() {
        let input = b"x147/12";

        let res = exchangeparser(input);
        assert!(res.is_done());

        let (_, val) = res.unwrap();
        assert_eq!(val, Dance::Exchange(147, 12));
    }

    #[test]
    fn parses_a_partner() {
        let input = b"pA/B";

        let res = partnerparser(input);
        assert!(res.is_done());

        let (_, val) = res.unwrap();
        assert_eq!(val, Dance::Partner('A', 'B'));
    }

    #[test]
    fn parses_full() {
        let res = danceparser(INPUT_DAY16_PT1);
        assert!(res.is_done());

        let (_, val) = res.unwrap();
        assert_eq!(val[0], Dance::Exchange(3, 4));
        assert_eq!(val[1], Dance::Partner('m', 'e'));
        assert_eq!(val[2], Dance::Exchange(15, 7));
    }
}
